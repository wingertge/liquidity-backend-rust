use liquidity::{Context, Uuid};
use liquidity_api::elections::{schema::Election, self};
use crate::auth::JWTError;
use juniper::FieldResult;

pub struct Query;

#[juniper::graphql_object(
    Context = Result<Context, JWTError>
)]
impl Query {
    #[graphql(
        description="Fetch an election by id",
            arguments(
            id(
                description = "The id of the election"
            )
        )
    )]
    pub fn election(id: Uuid, context: &Result<Context, JWTError>) -> FieldResult<Option<Election>> {
        let context = context.as_ref()?;
        elections::query::election(id, context).map_err(Into::into)
    }
}