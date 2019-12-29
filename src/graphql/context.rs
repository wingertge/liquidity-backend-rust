use std::sync::Arc;
use eventstore::Connection;

pub struct Context {
    pub db: Arc<Connection>,
    pub user: Option<Box<User>>
}

pub struct User {
    pub id: String,
    pub permissions: Vec<String>
}
