use crate::schema::{ElectionInput, Election};
use juniper::FieldResult;
use crate::context::Context;
use uuid::Uuid;
use crate::db::elections;
use crate::permissions;

pub struct Query;
pub struct Mutation;

#[juniper::object(
    Context = Context
)]
impl Query {
    fn election(id: Uuid, context: &Context) -> FieldResult<Option<Election>> {
        permissions::check("view:election", &context.user)?;
        let conn = &*context.db.get()?;
        let result = elections::find_election(&id, conn)?;

        Ok(Some(result.into()))
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
        permissions::check("create:election", &context.user)?;
        match &context.user {
            Some(user) => {
                use crate::db::elections;
                let conn = &*context.db.get()?;
                let result = elections::create_election(&input, &user.id, conn)?;
                Ok(Some(result.into()))
            },
            None => Err("Must be logged in".into())
        }
    }
}