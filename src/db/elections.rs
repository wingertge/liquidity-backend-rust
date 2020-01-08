use chrono::Utc;
use uuid::Uuid;
use crate::graphql::schema::{ElectionInput, Election, Importance::Regular};
use eventstore::{EventData, Connection, ResolvedEvent, OperationError};
use futures::{StreamExt, compat::{Stream01CompatExt, Future01CompatExt}};
use std::sync::Arc;
use super::models::CreateElectionEvent;
use crate::db::ESResultExt;
use std::error::Error;

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
/// # use backend_rust::graphql::schema::ElectionInput;
/// # use eventstore::{Connection, Credentials};
/// # use std::sync::Arc;
/// # let conn = Arc::new(
/// #     Connection::builder()
/// #         .with_default_user(Credentials::new("admin", "changeit"))
/// #         .single_node_connection(([127, 0, 0, 1], 1113).into())
/// # );
/// use backend_rust::db::elections;
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
/// let result = elections::create_election(election_input, "auth0|test", conn)
///     .await.unwrap().unwrap();
///
/// assert_eq!(result.name, "test_name".to_string());
/// # })
/// ```
pub async fn create_election(election: ElectionInput, creator_id: &str, conn: Arc<Connection>) -> Result<Option<Election>, Box<dyn Error>> {
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
        .await;

    log::info!("{:?}", result);
    result.map(|_| Some(event_data.into())).map_err(Into::into)
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
/// # use eventstore::{Connection, Credentials};
/// # use std::sync::Arc;
/// # use uuid::Uuid;
/// # let conn = Arc::new(
/// #     Connection::builder()
/// #         .with_default_user(Credentials::new("admin", "changeit"))
/// #         .single_node_connection(([127, 0, 0, 1], 1113).into())
/// # );
///
/// use backend_rust::db::elections;
/// let id = Uuid::new_v4();
///
/// let election = elections::find_election(&id, conn)
///     .await.unwrap();
///
/// assert_eq!(election, None);
/// # })
/// ```
pub async fn find_election(id: &Uuid, conn: Arc<Connection>) -> Result<Option<Election>, OperationError> {
    let stream = conn.read_stream(format!("election-{}", id))
        .forward()
        .iterate_over();

    let result = stream
        .compat()
        .fold(Ok(None), project_election)
        .await
        .map_not_found();

    result
}



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