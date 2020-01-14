use liquidity::{Context, Uuid};
use liquidity_api::elections::{schema::{Election, ElectionInput}, self};
use crate::auth::JWTError;
use juniper::FieldResult;

pub struct Mutation;

#[juniper::graphql_object(
    Context = Result<Context, JWTError>
)]
impl Mutation {
    #[graphql(
        description="Create a new election",
        arguments(
            input(
                description = "The input data for the new election"
            )
        )
    )]
    pub async fn create_election(input: ElectionInput, context: &Result<Context, JWTError>) -> FieldResult<Election> {
        let context = context.as_ref()?;

        elections::mutation::create_election(input, context).await.map_err(Into::into)
    }

    #[graphql(
        description="Edit an election",
        arguments(
            id(
                description = "The id of the election"
            ),
            input(
                description = "The fields to update"
            )
        )
    )]
    pub fn edit_election(id: Uuid, input: ElectionInput, context: &Result<Context, JWTError>) -> FieldResult<Election> {
        let context = context.as_ref()?;
        let result = elections::mutation::edit_election(id, input, context)?;
        Ok(result)
    }
}