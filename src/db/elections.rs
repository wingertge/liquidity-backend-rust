use super::{DbConnection, models::{Election, Choice}, schema::{elections, choices}};
use failure::{Error, ResultExt};
use crate::schema::ElectionInput;
use crate::db::models::{NewElection, NewChoice};
use chrono::Utc;
use crate::schema::Importance::Regular;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods};
use uuid::Uuid;

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

pub fn find_election(id: &Uuid, conn: &DbConnection) -> Result<(Election, Vec<Choice>), Error> {
    let election: Election = elections::table.find(id).first::<Election>(conn).context("Couldn't find the election")?;
    let choices: Vec<Choice> = choices::table
        .filter(choices::election_id.eq(&election.id))
        .order_by(choices::ballot_index.asc())
        .load::<Choice>(conn)
        .context("Failed to load associated choices")?;

    Ok((election, choices))
}