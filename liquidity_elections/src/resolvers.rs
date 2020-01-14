pub mod query {
    use liquidity::{Context, Uuid, permissions, Error};
    use futures::executor::block_on;
    use crate::schema::Election;

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
    pub fn election(id: Uuid, context: &Context) -> Result<Option<Election>, Error> {
        let span = trace_span!("election", "id: {}, context: {:?}", id, context);

        span.in_scope(|| {
            permissions::check("view:election", &context.user)?;
            use crate::repository::find_election;

            let db = context.db.clone();
            let result = block_on(find_election(&id, db)); //TODO: Don't block as soon as futures 0.3 support arrives for eventstore
            result.map_err(Into::into)
        })
    }
}

pub mod mutation {
    use futures::executor::block_on;
    use liquidity::{Context, permissions, Error, Uuid};
    use crate::schema::{Election, ElectionInput};

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
    pub async fn create_election(
        input: ElectionInput,
        context: &Context
    ) -> Result<Election, Error> {
        let span = trace_span!("create_election", "input: {:?}, context: {:?}", input, context);
        let _enter = span.enter();
        permissions::check("create:election", &context.user)?;
        if input.name.is_none() { return Err("Name cannot be null".into()) }

        let db = context.db.clone();
        let user = context.user.as_ref().unwrap();

        use crate::repository::create_election;
        let result = create_election(input, &user.id, db).await?;
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
    pub fn edit_election(
        id: Uuid,
        input: ElectionInput,
        context: &Context
    ) -> Result<Election, Error> {
        permissions::check("update:election", &context.user)?;
        let db = context.db.clone();

        use crate::repository::update_election;
        let result = block_on(update_election(&id, input, db))?;
        Ok(result)
    }
}