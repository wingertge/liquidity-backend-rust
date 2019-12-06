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
        let perm = "view:election".to_string();

        if !&context.user.as_ref()
            .ok_or("Must be logged in to view elections")?
            .permissions.contains(&perm) {
            return Err("You don't have permission to view elections".into())
        }
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
        match &context.user {
            Some(user) => {
                let perm = "create:election".to_string();
                if !user.permissions.contains(&perm) {
                    return Err("You don't have permission to view elections".into())
                }

                use crate::db::elections;
                let conn = &*context.db.get()?;
                let result = elections::create_election(&input, &user.id, conn)?;
                Ok(Some(result.into()))
            },
            None => Err("Must be logged in".into())
        }
    }
}