use crate::db::DbPool;

pub struct Context {
    pub db: DbPool,
    pub user: Option<Box<User>>
}

pub struct User {
    pub id: String,
    pub permissions: Vec<String>
}
