use super::{schema::{ElectionInput, Election}, context::Context, permissions};
use juniper::FieldResult;
use uuid::Uuid;

/// GraphQL Query type
pub struct Query;
/// GraphQL Mutation type
pub struct Mutation;

#[juniper::object(
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
        elections::find_election(&id, db).map_err(Into::into)
    }
}

#[juniper::object(
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
    pub fn create_election(
        input: ElectionInput,
        context: &Context
    ) -> FieldResult<Option<Election>> {
        permissions::check("create:election", &context.user)?;
        let db = context.db.clone();
        match &context.user {
            Some(user) => {
                use crate::db::elections;
                elections::create_election(input, &user.id, db).map_err(Into::into)
            },
            None => Err("Must be logged in".into())
        }
    }
}