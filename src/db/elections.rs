use super::{models::{Election, Choice}, schema::{elections, choices}};
use failure::{Error, ResultExt};
use crate::db::models::{NewElection, NewChoice};
use chrono::Utc;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use uuid::Uuid;
use crate::graphql::schema::{ElectionInput, Importance::Regular};
use crate::db::DbConnection;

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
pub fn create_election(election: &ElectionInput, creator_id: &str, conn: &DbConnection) -> Result<(Election, Vec<Choice>), Error> {
    let id = Uuid::new_v4();

    let new_election = NewElection {
        id: &id,
        created_by_id: creator_id,
        name: &election.name,
        description: &election.description.clone().unwrap_or("".to_string()),
        start_date: &election.start_date.unwrap_or_else(Utc::now),
        end_date: &election.end_date.unwrap_or_else(Utc::now),
        importance: &election.importance.as_ref().unwrap_or(&Regular).to_string(),
        created_at: &Utc::now(),
        updated_at: &Utc::now()
    };

    let election_result = diesel::insert_into(elections::table)
        .values(&new_election)
        .get_result::<Election>(conn)
        .context("Failed to create new election")?;

    let new_choices = match &election.choices {
        Some(choices) => {
            let mut i = -1;

            choices.iter().map(|choice| {
                i += 1;
                NewChoice {
                    election_id: &election_result.id,
                    ballot_index: i.clone(),
                    value: choice.to_owned()
                }
            }).collect()
        },
        None => vec![]
    };

    let choices_result = diesel::insert_into(choices::table)
        .values(new_choices)
        .get_results::<Choice>(conn)
        .context("Failed to create choices for election")?;


    Ok((election_result, choices_result))
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
pub fn find_election(id: &Uuid, conn: &DbConnection) -> Result<(Election, Vec<Choice>), Error> {
    let election: Election = elections::table.find(id).first::<Election>(conn).context("Couldn't find the election")?;
    let choices: Vec<Choice> = choices::table
        .filter(choices::election_id.eq(&election.id))
        .order_by(choices::ballot_index.asc())
        .load::<Choice>(conn)
        .context("Failed to load associated choices")?;

    Ok((election, choices))
}