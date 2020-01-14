#[macro_use] extern crate tracing;

pub mod repository;
pub mod resolvers;
pub mod schema;
mod models;

pub use resolvers::{query, mutation};

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use liquidity::{Connection, Credentials};
    use tokio_test::block_on;
    use crate::schema::{ElectionInput};
    use crate::repository;

    fn conn() -> Arc<Connection> {
        Arc::new(
            Connection::builder()
                .with_default_user(Credentials::new("admin", "changeit"))
                .single_node_connection(([127, 0, 0, 1], 1113).into())
        )
    }

    #[test]
    fn find_works() {
        block_on(async {
            let conn = conn();

            let create_input = ElectionInput {
                name: Some("test_name".to_string()),
                description: Some("test_description".to_string()),
                choices: Some(vec!["test1".to_string(), "test2".to_string()]),
                ..ElectionInput::default()
            };

            let update_input = ElectionInput {
                description: Some("test_description_2".to_string()),
                ..ElectionInput::default()
            };

            let election = repository::create_election(create_input, "some_user_id", conn.clone())
                .await
                .expect("Creating an election should not fail");

            let find_create = repository::find_election(&election.id, conn.clone())
                .await
                .expect("Finding the election should not fail")
                .expect("The election should not be None");

            assert_eq!(find_create.name, "test_name".to_string());
            assert_eq!(find_create.description, "test_description");

            repository::update_election(&election.id, update_input, conn.clone())
                .await
                .expect("Updating the election should not fail");

            let find_update = repository::find_election(&election.id, conn.clone())
                .await
                .expect("Finding the election should not fail")
                .expect("The election should not be None");

            assert_eq!(find_update.name, "test_name".to_string());
            assert_eq!(find_update.description, "test_description_2".to_string());
        })
    }
}