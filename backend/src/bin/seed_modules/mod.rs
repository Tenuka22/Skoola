use diesel::sqlite::SqliteConnection;
use anyhow::Result;

pub trait SeedModule {
    fn seed(&self, conn: &mut SqliteConnection) -> Result<()>;
}