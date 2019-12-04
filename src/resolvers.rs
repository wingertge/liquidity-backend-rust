use crate::schema::{ElectionInput, Election};
use juniper::{FieldResult};
use crate::schema::Importance::Regular;
use chrono::{Utc};
use crate::{Context, models};
use diesel::{QueryDsl, RunQueryDsl};

pub struct Query;
pub struct Mutation;

#[juniper::object(
    Context = Context
)]
impl Query {
    fn election(id: String, context: &Context) -> FieldResult<Option<Election>> {
        use crate::db_schema::elections::dsl::elections;

        let conn = &*context.db.get()?;
        let results = elections.find(id.parse::<u64>()?).load::<models::Election>(conn)?;
        println!("{:?}", results);
        Ok(Some(Election { id, name: "Test election".to_string(), description: "".to_string(), choices: Vec::new(), importance: Regular, start_date: Utc::now(), end_date: Utc::now() }))
    }
}

#[juniper::object(
    Context = Context
)]
impl Mutation {
    fn create_election(
        input: ElectionInput,
        context: &Context
    ) -> FieldResult<Option<Election>> {
        match &context.user {
            Some(user) => {

                Ok(None)
            },
            None => Err("Must be logged in".into())
        }
    }
}