use crate::Connection;
use std::{fmt, sync::Arc};

pub struct Context {
    pub db: Arc<Connection>,
    pub user: Option<User>
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "user: {:?}", self.user)
    }
}

#[derive(Debug)]
pub struct User {
    pub id: String,
    pub permissions: Vec<String>
}