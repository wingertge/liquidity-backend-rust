use std::sync::Arc;
use eventstore::Connection;
use crate::metrics::Metrics;

pub struct Context {
    pub db: Arc<Connection>,
    pub user: Option<User>,
    pub metrics: Arc<Metrics>
}

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub permissions: Vec<String>
}