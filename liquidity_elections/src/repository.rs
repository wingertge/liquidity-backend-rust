use chrono::Utc;
use crate::schema::{ElectionInput, Election, Importance::Regular};
use eventstore::{EventData, Connection, ResolvedEvent, OperationError};
use futures::{StreamExt, compat::{Stream01CompatExt, Future01CompatExt}};
use std::sync::Arc;
use super::models::CreateElectionEvent;
use liquidity::{db::{ESResultExt, StupidConnectionWrapper}, Uuid};
use std::error::Error;
use tracing_futures::Instrument;

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
/// # use liquidity_elections::schema::ElectionInput;
/// # use liquidity::{Connection, Credentials};
/// # use std::sync::Arc;
/// # let conn = Arc::new(
/// #     Connection::builder()
/// #         .with_default_user(Credentials::new("admin", "changeit"))
/// #         .single_node_connection(([127, 0, 0, 1], 1113).into())
/// # );
/// use liquidity_elections::repository;
///
/// let election_input = ElectionInput {
///     name: "test_name".to_string(),
///     description: Some("This is a test description".to_string()),
///     choices: Some(vec!["test1".to_string(), "test2".to_string()]),
///     permissions: None,
///     start_date: None,
///     end_date: None,
///     importance: None
/// };
///
/// let result = repository::create_election(election_input, "auth0|test", conn)
///     .await.unwrap().unwrap();
///
/// assert_eq!(result.name, "test_name".to_string());
/// # })
/// ```
pub async fn create_election(election: ElectionInput, creator_id: &str, conn: Arc<Connection>) -> Result<Option<Election>, Box<dyn Error>> {
    #[instrument]
    async fn create_election(election: ElectionInput, creator_id: &str, conn_wrapper: StupidConnectionWrapper) -> Result<Option<Election>, Box<dyn Error>> {
        let conn = conn_wrapper.0;
        let id = Uuid::new_v4();
        let stream_id = format!("election-{}", id);

        let event_data = CreateElectionEvent {
            id,
            created_by_id: creator_id.to_string(),
            name: election.name,
            description: election.description.unwrap_or("".to_string()),
            start_date: election.start_date.unwrap_or_else(|| Utc::now()),
            end_date: election.end_date.unwrap_or_else(|| Utc::now()),
            importance: election.importance.unwrap_or(Regular),
            choices: election.choices.unwrap_or(vec![])
        };

        let event_payload = serde_json::to_value(event_data.clone())?;

        let event = EventData::json("election-create", event_payload)?;

        let result = conn
            .write_events(stream_id)
            .push_event(event)
            .execute()
            .compat()
            .instrument(trace_span!("store_event"))
            .await;

        match &result {
            Ok(event_data) => debug!("{:?}", event_data),
            Err(e) => error!("{:?}", e)
        };

        result.map(|_| Some(event_data.into())).map_err(Into::into)
    }

    create_election(election, creator_id, StupidConnectionWrapper(conn)).await
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
/// # use std::sync::Arc;
/// # let conn = Arc::new(
/// #     Connection::builder()
/// #         .with_default_user(Credentials::new("admin", "changeit"))
/// #         .single_node_connection(([127, 0, 0, 1], 1113).into())
/// # );
///
/// use liquidity_elections::repository;
/// let id = Uuid::new_v4();
///
/// let election = repository::find_election(&id, conn)
///     .await.unwrap();
///
/// assert_eq!(election, None);
/// # })
/// ```
pub async fn find_election(id: &Uuid, conn: Arc<Connection>) -> Result<Option<Election>, OperationError> {
    #[instrument]
    async fn find(id: &Uuid, conn_wrapper: StupidConnectionWrapper) -> Result<Option<Election>, OperationError> {
        let conn = conn_wrapper.0;
        let stream_id = format!("election-{}", id);

        let stream = trace_span!("open_stream", "id: {}", stream_id).in_scope(|| {
            conn.read_stream(stream_id)
                .forward()
                .iterate_over()
        });

        let result = async {
            let result = stream
                .compat()
                .fold(Ok(None), project_election)
                .await;

            result
        }.instrument(trace_span!("fold_election"));

        result.await.map_not_found()
    }

    find(id, StupidConnectionWrapper(conn)).await
}

#[instrument]
async fn project_election(acc: Result<Option<Election>, OperationError>, item: Result<ResolvedEvent, OperationError>) -> Result<Option<Election>, OperationError> {
    let acc = acc?;
    let event = item?.event.unwrap();
    match event.event_type.as_str() {
        "election-create" => {
            let payload = event.as_json::<CreateElectionEvent>().unwrap();
            Ok(Some(payload.into()))
        },
        _ => Ok(acc)
    }
}