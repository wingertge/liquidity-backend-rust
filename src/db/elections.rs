use failure::Error;
use chrono::Utc;
use uuid::Uuid;
use crate::graphql::schema::{ElectionInput, Election, Importance::Regular};
use eventstore::{EventData, Connection};
use futures::{Future, future, stream::Stream};
use std::sync::Arc;
use super::models::CreateElectionEvent;

/// Create a new election in the database
///
/// This inserts a new election given an election input and the ID of the user creating it.
/// This will error if the insert fails for some reason (at this point it should only be due to connection errors).
///
/// # Arguments
///
/// `election` - The input object with the user input data for the new election
/// `creator_id` - The id of the user calling the creation function
/// `conn` - The database connection to execute the insert on
///
/// # Example
///
/// ```ignore
/// //currently not tested, since it's annoying having to set up a test DB
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
/// let result = elections::create_election(&election_input, "auth0|asdqwe", &conn);
///
/// ```
pub fn create_election(election: ElectionInput, creator_id: &str, conn: Arc<Connection>) -> Result<Option<Election>, Error> {
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
        .execute();

    result.map(|result| {
        log::info!("{:?}", result);
        event_data.into()
    }).map_err(Into::into).wait().map(|e| Some(e))
}

/// Find an election by its id
///
/// # Arguments
///
/// `id` - The id of the election
/// `conn` - The database connection
///
/// # Returns
///
/// The election or the database error if one occurred
///
/// # Example
///
/// ```ignore
/// //currently not tested, since it's annoying having to set up a test DB
/// use backend_rust::db::elections;
/// let election = elections::find_election(&id, &conn);
/// ```
pub fn find_election(id: &Uuid, conn: Arc<Connection>) -> Result<Option<Election>, Error> {
    let stream = conn.read_stream(format!("election-{}", id))
        .forward();

    stream.iterate_over()
        .fold(None, |acc: Option<Election>, item| {
            let event = item.event.unwrap();
            let result = match event.event_type.as_str() {
                "election-create" => {
                    let payload = event.as_json::<CreateElectionEvent>().unwrap();
                    Some(payload.into())
                },
                _ => acc
            };
            future::ok(result)
        })
        .map_err(Into::into).wait()
}