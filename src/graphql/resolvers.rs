use super::{schema::{ElectionInput, Election}, context::Context, permissions};
use juniper::FieldResult;
use uuid::Uuid;
use futures::executor::block_on;
use crate::auth::JWTError;

/// GraphQL Query type
pub struct Query;
/// GraphQL Mutation type
pub struct Mutation;

#[juniper::graphql_object(
    Context = Result<Context, JWTError>
)]
impl Query {
    /// Fetch an election by id
    ///
    /// # Arguments
    ///
    /// `id` - The id to look up the election by
    /// `context` - The request context, passed automatically
    ///
    /// # Permissions Required
    ///
    /// `view:election`
    ///
    /// # Returns
    ///
    /// The election if it exists, an error if it doesn't
    ///
    /// # Example
    ///
    /// ```ignore
    /// query {
    ///     election(id: "some_uuid") {
    ///         id
    ///         name
    ///         choices
    ///     }
    /// }
    /// ```
    #[graphql(
        description="Fetch an election by id",
        arguments(
            id(
                description = "The id of the election"
            )
        )
    )]
    pub fn election(id: Uuid, context: &Result<Context, JWTError>) -> FieldResult<Option<Election>> {
        match context {
            Ok(context) => {
                permissions::check("view:election", &context.user)?;
                use crate::db::elections;

                let db = context.db.clone();
                let result = block_on(elections::find_election(&id, db)); //TODO: Don't block as soon as futures 0.3 support arrives for eventstore
                result.map_err(Into::into)
            },
            Err(e) => Err(e.into())
        }
    }
}

#[juniper::graphql_object(
    Context = Result<Context, JWTError>
)]
impl Mutation {
    /// Create a new election
    ///
    /// # Arguments
    ///
    /// `input` - The election user input object
    /// `context` - The request context, passed automatically
    ///
    /// # Permissions Required
    ///
    /// `create:election`
    ///
    /// # Returns
    ///
    /// The new Election or an error if the creation failed
    ///
    /// # Example
    ///
    /// ```ignore
    /// mutation {
    //      createElection(input: {name: "test"}) {
    //          id
    //          name
    //          description
    //      }
    //  }
    /// ```
    #[graphql(
        description="Create a new election",
        arguments(
            input(
                description = "The input data for the new election"
            )
        )
    )]
    pub async fn create_election(
        input: ElectionInput,
        context: &Result<Context, JWTError>
    ) -> FieldResult<Option<Election>> {
        match context {
            Ok(context) => {
                permissions::check("create:election", &context.user)?;
                let db = context.db.clone();
                match &context.user {
                    Some(user) => {
                        use crate::db::elections;
                        elections::create_election(input, &user.id, db).await.map_err(Into::into)
                    },
                    None => Err("Must be logged in".into())
                }
            },
            Err(e) => Err(e.into())
        }
    }
}