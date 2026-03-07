use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use diesel::prelude::SqliteConnection;
use std::collections::HashSet;

pub struct ExamsSeeder;

impl ExamsSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for ExamsSeeder {
    fn seed(
        &self,
        _conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        _context: &mut SeederContext,
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Skipping legacy exams seeding: exam schema updated.");
        Ok(())
    }
}
