use liquidity::Uuid;
use liquidity_api::elections::schema::Election;
use crate::auth::JWTError;
use juniper::FieldResult;
use liquidity_api::APIContext;

pub struct Query;

#[juniper::graphql_object(
    Context = Result<APIContext, JWTError>
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
    pub async fn election(id: Uuid, context: &Result<APIContext, JWTError>) -> FieldResult<Option<Election>> {
        let context = context.as_ref()?;
        Ok(context.elections().election(id, context).await?)
    }
}