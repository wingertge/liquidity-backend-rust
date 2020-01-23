use crate::db::DbConnection;
use std::fmt;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub permissions: Vec<String>
}

pub trait Context<DB: DbConnection> : fmt::Debug {
    fn db(&self) -> DB;
    fn user(&self) -> &Option<User>;
}