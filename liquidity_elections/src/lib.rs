#[macro_use] extern crate tracing;

pub mod repository;
pub mod resolvers;
pub mod schema;
mod models;

pub use resolvers::{query, mutation};

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use liquidity::{Connection, Credentials, Uuid, Merge};
    use tokio_test::block_on;
    use crate::schema::{ElectionInput, Election, Importance};
    use crate::repository;
    use liquidity::db::DatabaseError;
    use chrono::Utc;
    use crate::models::UpdateElectionEvent;

    fn conn() -> Arc<Connection> {
        Arc::new(
            Connection::builder()
                .with_default_user(Credentials::new("admin", "changeit"))
                .single_node_connection(([127, 0, 0, 1], 1113).into())
        )
    }

    fn create_test_election(conn: Arc<Connection>) -> Election {
        let res = async {
            let input = ElectionInput {
                name: Some("test_name".to_string()),
                description: Some("test_description".to_string()),
                choices: Some(vec!["test1".to_string(), "test2".to_string()]),
                ..ElectionInput::default()
            };

            repository::create_election(input, "some_user_id", conn.clone())
                .await
                .expect("Creating an election should not fail")
        };

        block_on(res)
    }

    fn update_test_election(id: &Uuid, conn: Arc<Connection>) -> Election {
        let res = async {
            let update_input = ElectionInput {
                description: Some("test_description_2".to_string()),
                ..ElectionInput::default()
            };

            repository::update_election(id, update_input, conn.clone())
                .await
                .expect("Updating the election should not fail")
        };

        block_on(res)
    }

    fn find_test_election(id: &Uuid, conn: Arc<Connection>) -> Election {
        let res = async {
            repository::find_election(id, conn)
                .await
                .expect("Finding the election should not fail")
                .expect("The election should not be None")
        };

        block_on(res)
    }

    #[test]
    fn create_works() {
        let conn = conn();
        let election = create_test_election(conn);

        assert_eq!(election.name, "test_name".to_string());
        assert_eq!(election.description, "test_description".to_string());
        assert_eq!(election.choices, vec!["test1".to_string(), "test2".to_string()]);
        assert_eq!(election.importance, Importance::Regular)
    }

    #[test]
    fn update_works() {
        let conn = conn();

        let election = create_test_election(conn.clone());
        let updated = update_test_election(&election.id, conn);

        assert_eq!(election.name, updated.name);
        assert_eq!(election.importance, updated.importance);
        assert_eq!(updated.description, "test_description_2".to_string());
        assert_ne!(election.description, updated.description);
    }

    #[test]
    fn update_not_found_works() {
        let conn = conn();

        let input = ElectionInput {
            name: Some("test".to_string()),
            ..ElectionInput::default()
        };

        let id = Uuid::new_v4();
        let updated = block_on(repository::update_election(&id, input, conn));

        match updated {
            Err(DatabaseError::NotFound) => {},
            _ => assert!(false)
        }
    }

    #[test]
    fn find_works() {
        let conn = conn();

        let election = create_test_election(conn.clone());

        let find_create = find_test_election(&election.id, conn.clone());

        assert_eq!(find_create.name, "test_name".to_string());
        assert_eq!(find_create.description, "test_description");

        update_test_election(&election.id, conn.clone());
        let find_update = find_test_election(&election.id, conn.clone());

        assert_eq!(find_update.name, "test_name".to_string());
        assert_eq!(find_update.description, "test_description_2".to_string());
    }

    #[test]
    fn find_not_found_works() {
        let conn = conn();

        let id = Uuid::new_v4();
        let election = block_on(repository::find_election(&id, conn))
            .expect("This shouldn't throw an error");

        assert_eq!(election, None);
    }

    #[test]
    fn merge_works() {
        let original = Election {
            id: Uuid::new_v4(),
            name: "test_name".to_string(),
            description: "test_description".to_string(),
            importance: Importance::Regular,
            choices: vec!["test_choice_1".to_string(), "test_choice_2".to_string()],
            start_date: Utc::now(),
            end_date: Utc::now()
        };

        let update = UpdateElectionEvent {
            name: None,
            description: Some("test_description_2".to_string()),
            importance: None,
            choices: None,
            start_date: None,
            end_date: None
        };

        let original_id = original.id.clone();
        let original_description = original.description.clone();
        let result = original.merge_with(update);

        assert_eq!(result.id, original_id);
        assert_eq!(result.name, "test_name".to_string());
        assert_ne!(result.description, original_description);
        assert_eq!(result.description, "test_description_2".to_string());
    }
}