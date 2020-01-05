use std::sync::Arc;
use eventstore::Connection;

pub struct Context {
    pub db: Arc<Connection>,
    pub user: Option<User>
}

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub permissions: Vec<String>
}
