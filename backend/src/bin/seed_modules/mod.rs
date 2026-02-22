use diesel::sqlite::SqliteConnection;
use anyhow::Result;

pub mod utils;

pub trait SeedModule {
    fn seed(&self, conn: &mut SqliteConnection) -> Result<()>;
}