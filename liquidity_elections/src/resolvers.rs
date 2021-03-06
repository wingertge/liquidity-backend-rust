use crate::repository::ElectionRepository;
use liquidity::{Uuid, Context, Error, permissions};
use crate::schema::{Election, ElectionInput};
use std::time::Duration;
use liquidity::db::DbConnection;

#[derive(Debug)]
pub struct ElectionResolvers {
    repository: ElectionRepository
}

impl ElectionResolvers {
    pub fn new(cache_capacity: usize, cache_ttl: Duration) -> ElectionResolvers {
        ElectionResolvers {
            repository: ElectionRepository::new(cache_capacity, cache_ttl)
        }
    }

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
    ///     createElection(input: {name: "test"}) {
    ///        id
    ///        name
    ///        description
    ///     }
    /// }
    /// ```
    #[instrument]
    pub async fn create_election<T: DbConnection, C: Context<T>>(
        &self,
        input: ElectionInput,
        context: &C
    ) -> Result<Election, Error> {
        permissions::check("create:election", context.user())?;
        if input.name.is_none() { return Err("Name cannot be null".into()) }

        let db = context.db();
        let user = context.user().as_ref().unwrap();

        let result = self.repository.create_election(input, &user.id, db).await?;
        Ok(result)
    }

    /// Edit an election
    ///
    /// # Arguments
    ///
    /// `id` - The id of the election to be edited
    /// `input` - The fields to edit
    /// `context` - The request context, passed automatically
    ///
    /// # Permissions Required
    ///
    /// `update:election`
    ///
    /// # Returns
    ///
    /// The edited election or an error if the update failed
    ///
    /// # Example
    ///
    /// ```ignore
    /// mutation {
    ///     editElection(id: "54c8ed41-b4f4-4f1c-8903-f9bbbe2d992d", input: {description: "new description"}) {
    ///        id
    ///        name
    ///        description
    ///     }
    /// }
    /// ```
    #[instrument]
    pub async fn edit_election<T: DbConnection, C: Context<T>>(
        &self,
        id: Uuid,
        input: ElectionInput,
        context: &C
    ) -> Result<Election, Error> {
        permissions::check("update:election", &context.user())?;
        let db = context.db();

        let result = self.repository.update_election(&id, input, db).await?;
        Ok(result)
    }

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
    /// The election if it exists, None if it doesn't, Error if an issue has occurred
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
    #[instrument]
    pub async fn election<T: DbConnection, C: Context<T>>(&self, id: Uuid, context: &C) -> Result<Option<Election>, Error> {
        permissions::check("view:election", &context.user())?;

        let db = context.db();
        let result = self.repository.find_election(&id, db).await?;
        Ok(result)
    }
}