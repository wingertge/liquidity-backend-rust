use chrono::Utc;
use crate::schema::{Election, Importance::Regular, ElectionInput};
use super::models::CreateElectionEvent;
use liquidity::{Uuid, Merge};
use crate::models::{UpdateElectionEvent};
use liquidity::db::{DatabaseError, DbConnection};
use futures::lock::Mutex;
use std::sync::Arc;
use std::fmt;
use ttl_cache::TtlCache;
use std::time::Duration;

type Cache = Arc<Mutex<TtlCache<Uuid, Election>>>;

pub struct ElectionRepository {
    cache: Cache,
    time_to_live: Duration
}

impl fmt::Debug for ElectionRepository {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Nothing for now")
    }
}

impl ElectionRepository {
    pub fn new(cache_capacity: usize, time_to_live: Duration) -> ElectionRepository {
        ElectionRepository {
            cache: Arc::new(Mutex::new(TtlCache::new(cache_capacity))),
            time_to_live
        }
    }

    /// Create a new election in the database
    ///
    /// This inserts a new election given an election input and the ID of the user creating it.
    /// This will error if the insert fails for some reason (at this point it should only be due to connection errors).
    ///
    /// # Arguments
    ///
    /// * `election` - The input object with the user input data for the new election
    /// * `creator_id` - The id of the user calling the creation function
    /// * `conn` - The database connection to execute the insert on
    ///
    /// # Example
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// # use liquidity::{Connection, Credentials, Uuid};
    /// # use liquidity_elections::{repository::ElectionRepository, schema::ElectionInput};
    /// # use liquidity_test_utils::connection::MockConnection;
    /// # use std::time::Duration;
    /// # let conn = MockConnection::new();
    /// # let repository = ElectionRepository::new(0, Duration::from_secs(0));
    ///
    /// let election_input = ElectionInput {
    ///     name: Some("test_name".to_string()),
    ///     description: Some("This is a test description".to_string()),
    ///     choices: Some(vec!["test1".to_string(), "test2".to_string()]),
    ///     ..ElectionInput::default()
    /// };
    ///
    /// let result = repository.create_election(election_input, "auth0|test", conn)
    ///     .await.unwrap();
    ///
    /// assert_eq!(result.name, "test_name".to_string());
    /// # })
    /// ```
    #[instrument(skip(conn))]
    pub async fn create_election<T: DbConnection>(&self, election: ElectionInput, creator_id: &str, conn: T) -> Result<Election, DatabaseError> {
        let id = Uuid::new_v4();
        let stream_id = format!("election-{}", id);

        let event_data = CreateElectionEvent {
            id,
            created_by_id: creator_id.to_string(),
            name: election.name.unwrap(),
            description: election.description.unwrap_or_else(|| "".to_string()),
            start_date: election.start_date.unwrap_or_else(Utc::now),
            end_date: election.end_date.unwrap_or_else(Utc::now),
            importance: election.importance.unwrap_or(Regular),
            choices: election.choices.unwrap_or_else(|| vec![])
        };

        let result = conn
            .create(stream_id, event_data.clone())
            .await;

        match &result {
            Ok(event_data) => debug!("{:?}", event_data),
            Err(e) => error!("{:?}", e)
        };

        result?;

        let election: Election = event_data.into();

        Ok(election)
    }

    /// Update an election in the database
    ///
    /// This update an election in the database given an election input and the ID of the election.
    /// This will error if the stream doesn't exist or the update fails because of connection errors.
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the election to be updated
    /// * `election` - The input object with the fields to update
    /// * `conn` - The database connection to execute the update on
    ///
    /// # Example
    ///
    /// ```
    /// # use liquidity_elections::schema::Election;
    /// use liquidity::Uuid;
    /// futures::executor::block_on(async {
    /// # use liquidity::{Connection, Credentials, Uuid};
    /// # use liquidity_elections::{repository::ElectionRepository, schema::ElectionInput};
    /// # use liquidity_test_utils::connection::MockConnection;
    /// # use std::time::Duration;
    /// # let conn = MockConnection::new();
    /// # let repository = ElectionRepository::new(0, Duration::from_secs(0));
    ///
    /// let election_input = ElectionInput {
    ///     description: Some("This is a new test description".to_string()),
    ///     ..ElectionInput::default()
    /// };
    ///
    /// let id = Uuid::new_v4();
    /// let result = repository.update_election(&id, election_input, conn).await;
    ///
    /// assert!(result.is_err())
    /// # })
    /// ```
    #[instrument(skip(conn))]
    pub async fn update_election<T: DbConnection>(&self, id: &Uuid, input: ElectionInput, conn: T) -> Result<Election, DatabaseError> {
        let stream_id = format!("election-{}", id);

        let original = self.find_election(id, conn.clone()).await?
            .ok_or(DatabaseError::NotFound)?;

        if input.eq(&ElectionInput::default()) { return Ok(original) }

        let event_data = UpdateElectionEvent {
            name: input.name,
            description: input.description,
            choices: input.choices,
            start_date: input.start_date,
            end_date: input.end_date,
            importance: input.importance
        };

        let result = conn
            .update(stream_id, event_data.clone())
            .await;

        match &result {
            Ok(event_data) => debug!("{:?}", event_data),
            Err(e) => error!("{:?}", e)
        };

        result?;

        let mut cache = self.cache.lock().await;
        cache.remove(id);

        Ok(original.merge_with(event_data))
    }

    /// Find an election by its id
    ///
    /// # Arguments
    ///
    /// * `id` - The id of the election
    /// * `conn` - The database connection
    ///
    /// # Returns
    ///
    /// The election or the database error if one occurred
    ///
    /// # Example
    ///
    /// ```
    /// # futures::executor::block_on(async {
    /// # use liquidity::{Connection, Credentials, Uuid};
    /// # use liquidity_elections::repository::ElectionRepository;
    /// # use liquidity_test_utils::connection::MockConnection;
    /// # use std::time::Duration;
    /// # let conn = MockConnection::new();
    /// # let repository = ElectionRepository::new(0, Duration::from_secs(0));
    ///
    /// let id = Uuid::new_v4();
    ///
    /// let election = repository.find_election(&id, conn).await.unwrap();
    ///
    /// assert_eq!(election, None);
    /// # })
    /// ```
    #[instrument(skip(conn))]
    pub async fn find_election<T: DbConnection>(&self, id: &Uuid, conn: T) -> Result<Option<Election>, DatabaseError> {
        let mut cache = self.cache.lock().await;
        let cached_result = cache.get(id).map(|election| Election::to_owned(election));

        match cached_result {
            Some(election) => Ok(Some(election)),
            None => {
                let stream_id = format!("election-{}", id);

                let result = conn
                    .read::<_, Election, CreateElectionEvent, UpdateElectionEvent>(stream_id)
                    .await?;

                if let Some(ref election) = result {
                    cache.insert(id.to_owned(), election.clone(), self.time_to_live);
                }

    Ok(result)
}

#[instrument]
async fn project_election(acc: Result<Option<Election>, OperationError>, item: Result<ResolvedEvent, OperationError>) -> Result<Option<Election>, OperationError> {
    let acc = acc?;
    let event = item?.event.unwrap();
    match event.event_type.to_owned().into() {
        ElectionEventType::Create => {
            let payload = event.as_json::<CreateElectionEvent>().unwrap();
            Ok(Some(payload.into()))
        },
        ElectionEventType::Update => {
            let payload = event.as_json::<UpdateElectionEvent>().unwrap();
            Ok(acc.map(|acc| acc.merge_with(payload)))
        }
    }
}