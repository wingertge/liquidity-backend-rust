use super::models::CreateElectionEvent;
use crate::{
    models::UpdateElectionEvent,
    schema::{Election, ElectionInput, Importance::Regular}
};
use chrono::Utc;
use futures::lock::Mutex;
use liquidity::{
    db::{DatabaseError, DbConnection},
    Merge, Uuid
};
use std::{fmt, sync::Arc, time::Duration};
use ttl_cache::TtlCache;

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
    /// # let conn = MockConnection::default();
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
    pub async fn create_election<T: DbConnection>(
        &self,
        election: ElectionInput,
        creator_id: &str,
        conn: T
    ) -> Result<Election, DatabaseError> {
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

        let result = conn.create(stream_id, event_data.clone()).await;

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
    /// # let conn = MockConnection::default();
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
    pub async fn update_election<T: DbConnection>(
        &self,
        id: &Uuid,
        input: ElectionInput,
        conn: T
    ) -> Result<Election, DatabaseError> {
        let stream_id = format!("election-{}", id);

        let original = self
            .find_election(id, conn.clone())
            .await?
            .ok_or(DatabaseError::NotFound)?;

        if input.eq(&ElectionInput::default()) {
            return Ok(original);
        }

        let event_data = UpdateElectionEvent {
            name: input.name,
            description: input.description,
            choices: input.choices,
            start_date: input.start_date,
            end_date: input.end_date,
            importance: input.importance
        };

        let result = conn.update(stream_id, event_data.clone()).await;

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
    /// # let conn = MockConnection::default();
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
    pub async fn find_election<T: DbConnection>(
        &self,
        id: &Uuid,
        conn: T
    ) -> Result<Option<Election>, DatabaseError> {
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
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        models::{CreateElectionEvent, UpdateElectionEvent},
        repository::ElectionRepository,
        schema::{Election, ElectionInput, Importance}
    };
    use liquidity::db::EventType;
    use liquidity_test_utils::connection::MockConnection;
    use serde_json::Value;
    use std::{sync::Arc, time::Duration};
    use tokio_test::block_on;

    fn conn() -> MockConnection {
        MockConnection::default()
    }

    fn repository() -> Arc<ElectionRepository> {
        Arc::new(ElectionRepository::new(10, Duration::from_secs(600)))
    }

    fn test_election_input() -> ElectionInput {
        ElectionInput {
            name: Some("test_name".to_string()),
            description: Some("test_description".to_string()),
            choices: Some(vec!["test1".to_string(), "test2".to_string()]),
            ..ElectionInput::default()
        }
    }

    fn test_update_input() -> ElectionInput {
        ElectionInput {
            description: Some("test_description_2".to_string()),
            ..ElectionInput::default()
        }
    }

    #[test]
    fn create_works() {
        block_on(async {
            let conn = conn();
            let repository = repository();

            let election = repository
                .create_election(test_election_input(), "test_creator_id", conn.clone())
                .await
                .expect("Creating the election shouldn't fail");

            assert_eq!(election.name, "test_name".to_string());
            assert_eq!(election.description, "test_description".to_string());
            assert_eq!(
                election.choices,
                vec!["test1".to_string(), "test2".to_string()]
            );
            assert_eq!(election.importance, Importance::Regular);

            let sent = conn.data.lock().unwrap();
            let stream_id = format!("election-{}", election.id);
            let (event_type, value): (String, Value) = sent[&stream_id][0].clone();
            let payload: CreateElectionEvent = serde_json::from_value(value)
                .expect("The event payload should have the right type");

            assert_eq!(event_type, EventType::Create.as_ref());
            assert_eq!(payload.id, election.id);
            assert_eq!(payload.description, "test_description");
            assert_eq!(
                payload.choices,
                vec!["test1".to_string(), "test2".to_string()]
            )
        })
    }

    #[test]
    fn update_works() {
        block_on(async {
            let conn = conn();
            let repository = repository();

            let election = repository
                .create_election(test_election_input(), "test_creator_id", conn.clone())
                .await
                .expect("Creating the election shouldn't fail");

            let updated = repository
                .update_election(&election.id, test_update_input(), conn.clone())
                .await
                .expect("Updating the election shouldn't fail");

            assert_eq!(election.name, updated.name);
            assert_eq!(election.importance, updated.importance);
            assert_eq!(updated.description, "test_description_2".to_string());
            assert_ne!(election.description, updated.description);

            let stream_id = format!("election-{}", election.id);
            let events = conn
                .data
                .lock()
                .unwrap()
                .get(&stream_id)
                .expect("Stream should've been created")
                .clone();

            let create_event = events[0].clone();
            let create_payload = serde_json::from_value::<CreateElectionEvent>(create_event.1)
                .expect("Create event should be of the right type");
            let update_event = events[1].clone();
            let update_payload = serde_json::from_value::<UpdateElectionEvent>(update_event.1)
                .expect("Update event should be of the right type");

            assert_eq!(create_event.0, EventType::Create.as_ref());
            assert_eq!(create_payload.name, "test_name".to_string());
            assert_eq!(update_payload.name, None);
            assert_eq!(
                update_payload.description,
                Some("test_description_2".to_string())
            );
        })
    }

    #[test]
    fn find_works() {
        block_on(async {
            let conn = conn();
            let repository = repository();

            let election = repository
                .create_election(test_election_input(), "test_creator_id", conn.clone())
                .await
                .expect("Creating the election shouldn't fail");

            let read = repository
                .find_election(&election.id, conn.clone())
                .await
                .expect("Finding the election shouldn't fail")
                .expect("The returned election shouldn't be none");

            assert_eq!(read.name, "test_name".to_string());
            assert_eq!(read.description, "test_description".to_string());

            let cache_entry: Election = repository
                .cache
                .lock()
                .await
                .get(&election.id)
                .cloned()
                .expect("Election should exist in the cache");

            assert_eq!(cache_entry.name, election.name);
            assert_eq!(cache_entry.description, election.description);

            let updated = repository
                .update_election(&election.id, test_update_input(), conn.clone())
                .await
                .expect("Updating the election shouldn't fail");

            let read = repository
                .find_election(&election.id, conn.clone())
                .await
                .expect("Finding the election shouldn't fail")
                .expect("The returned election shouldn't be none");

            assert_eq!(read.name, election.name);
            assert_eq!(read.description, updated.description);

            let cache_entry: Election = repository
                .cache
                .lock()
                .await
                .get(&election.id)
                .cloned()
                .expect("Election should exist in the cache");

            assert_eq!(cache_entry.description, updated.description);
            assert_eq!(cache_entry.name, election.name);
        })
    }
}
