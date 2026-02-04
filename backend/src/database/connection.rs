use diesel::sqlite::SqliteConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::errors::APIError;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection(database_url: &str) -> Result<DbPool, APIError> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Ok(Pool::builder()
        .build(manager)
        .map_err(|e| APIError::internal(&format!("Failed to create DB pool: {}", e)))?)
}
