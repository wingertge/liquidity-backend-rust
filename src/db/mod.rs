use diesel::{PgConnection, r2d2::ConnectionManager};
use r2d2::Pool;

/// Contains Repository functions for elections
pub mod elections;
/// Contains database model types
pub mod models;
mod schema;

pub type DbConnection = PgConnection;
pub type DbPool = Pool<ConnectionManager<DbConnection>>;

/// Create a database pool
///
/// This function creates a database pool with a connection manager.
///
/// # Example
///
/// ```ignore
/// let db_pool = Arc::new(backend_rust::db::create_db_pool());
/// ```
pub fn create_db_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(&database_url);
    r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect(&format!("Error creating connection pool for {}", database_url))
}