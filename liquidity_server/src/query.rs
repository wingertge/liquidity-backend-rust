use liquidity::Uuid;
use liquidity_api::elections::schema::Election;
use juniper::FieldResult;
use liquidity_api::APIContext;

pub struct Query;

#[graphql_object(
    Context = APIContext
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
    pub async fn election(id: Uuid, context: &APIContext) -> FieldResult<Option<Election>> {
        Ok(context.elections().election(id, context).await?)
    }
}