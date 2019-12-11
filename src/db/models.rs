use chrono::{DateTime, Utc};
use super::schema::{choices, elections};
use uuid::Uuid;
use crate::graphql::{self, schema::Importance};

/// Election database type
#[derive(Identifiable, Queryable, Debug)]
pub struct Election {
    pub id: Uuid,
    pub created_by_id: String,
    pub name: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub importance: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

/// Easy conversion of database results into the GraphQL type
impl From<(Election, Vec<Choice>)> for graphql::schema::Election {
    fn from((election, choices): (Election, Vec<Choice>)) -> Self {
        graphql::schema::Election {
            id: election.id,
            name: election.name,
            description: election.description,
            choices: choices.iter().map(|x| x.value.to_owned()).collect(),
            importance: election.importance.parse::<Importance>().expect("Database contains invalid importance"),
            start_date: election.start_date,
            end_date: election.end_date
        }
    }
}

/// Type for inserting new elections into the database
#[derive(Insertable)]
#[table_name = "elections"]
pub struct NewElection<'a> {
    pub id: &'a Uuid,
    pub created_by_id: &'a str,
    pub name: &'a str,
    pub description: &'a str,
    pub start_date: &'a DateTime<Utc>,
    pub end_date: &'a DateTime<Utc>,
    pub importance: &'a str,
    pub created_at: &'a DateTime<Utc>,
    pub updated_at: &'a DateTime<Utc>
}

/// Type for election choices in the database
#[derive(Identifiable, Queryable, Associations)]
#[table_name = "choices"]
#[belongs_to(Election, foreign_key = "election_id")]
pub struct Choice {
    pub id: Uuid,
    pub election_id: Uuid,
    pub ballot_index: i16,
    pub value: String
}

/// Type for inserting new choices into the database
#[derive(Insertable)]
#[table_name="choices"]
pub struct NewChoice<'a> {
    pub id: Uuid,
    pub election_id: &'a Uuid,
    pub ballot_index: i16,
    pub value: String
}