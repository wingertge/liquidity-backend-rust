use liquidity::Uuid;
use liquidity_api::elections::schema::{Election, ElectionInput};
use juniper::FieldResult;
use liquidity_api::APIContext;

pub struct Mutation;

#[graphql_object(
    Context = APIContext
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
    pub async fn create_election(input: ElectionInput, context: &APIContext) -> FieldResult<Election> {
        Ok(context.elections().create_election(input, context).await?)
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
    pub async fn edit_election(id: Uuid, input: ElectionInput, context: &APIContext) -> FieldResult<Election> {
        Ok(context.elections().edit_election(id, input, context).await?)
    }
}