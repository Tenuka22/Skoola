use diesel::sqlite::SqliteConnection;
use anyhow::Result;
use std::collections::HashSet;
use crate::Config; // Import Config from the main crate (seed.rs)

pub mod utils;

pub trait SeedModule {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        config: &Config,
        password_hash: &str,
        used_emails: &mut HashSet<String>,
    ) -> Result<()>;
}