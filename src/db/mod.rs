use diesel::{PgConnection, r2d2::ConnectionManager};

pub mod elections;
pub mod models;
pub mod schema;

pub type DbConnection = PgConnection;
pub type DbPool = r2d2::Pool<ConnectionManager<DbConnection>>;
//pub type PooledDbConnection = r2d2::PooledConnection<ConnectionManager<DbConnection>>;

pub fn create_db_pool() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::new(&database_url);
    r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .expect(&format!("Error creating connection pool for {}", database_url))
}