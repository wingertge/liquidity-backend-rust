use crate::db::DbPool;
use std::sync::Arc;

pub struct Context {
    pub db: Arc<DbPool>,
    pub user: Option<Box<User>>
}

pub struct User {
    pub id: String,
    pub permissions: Vec<String>
}
