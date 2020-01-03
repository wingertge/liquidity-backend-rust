use super::{schema::{ElectionInput, Election}, context::Context, permissions};
use juniper::FieldResult;
use uuid::Uuid;
use futures::executor::block_on;

/// GraphQL Query type
pub struct Query;
/// GraphQL Mutation type
pub struct Mutation;

#[juniper::graphql_object(
    Context = Context
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
    pub fn election(id: Uuid, context: &Context) -> FieldResult<Option<Election>> {
        permissions::check("view:election", &context.user)?;
        use crate::db::elections;

        let db = context.db.clone();
        let result = block_on(elections::find_election(&id, db)); //TODO: Don't block as soon as futures 0.3 support arrives for eventstore
        result.map_err(Into::into)
    }
}

#[juniper::graphql_object(
    Context = Context
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
    pub async fn create_election(
        input: ElectionInput,
        context: &Context
    ) -> FieldResult<Option<Election>> {
        permissions::check("create:election", &context.user)?;
        let db = context.db.clone();
        match &context.user {
            Some(user) => {
                use crate::db::elections;
                elections::create_election(input, &user.id, db).await.map_err(Into::into)
            },
            None => Err("Must be logged in".into())
        }
    }
}