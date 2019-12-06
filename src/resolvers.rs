use crate::schema::{ElectionInput, Election};
use juniper::FieldResult;
use crate::Context;
use uuid::Uuid;
use crate::db::elections;

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
                let (election, choices) = elections::create_election(&input, &user.id, conn)?;
                Ok(Some(
                    Election {
                        id: election.id,
                        name: election.name,
                        description: election.description,
                        start_date: election.start_date,
                        end_date: election.end_date,
                        importance: election.importance.parse().expect("Invalid importance"),
                        choices: choices.iter().map(|choice| choice.value.clone()).collect()
                    }
                ))
            },
            None => Err("Must be logged in".into())
        }
    }
}