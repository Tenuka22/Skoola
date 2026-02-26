use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config; // Added
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{BigInt, Text}; // Combined and added Text
use std::collections::HashSet; // Added

#[derive(QueryableByName, Debug)]
pub struct TableName {
    #[diesel(sql_type = Text)] // Updated attribute
    pub name: String,
}

// Struct to hold the count result
#[derive(QueryableByName, Debug)]
struct CountResult {
    #[diesel(sql_type = BigInt)]
    count: i64,
}

pub struct SeederVerifier;

impl SeederVerifier {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for SeederVerifier {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        _context: &mut SeederContext,
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Verifying seeded data counts...");

        let table_names: Vec<TableName> = sql_query(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE 'diesel_%';"
        )
        .load(conn)?;

        for table in table_names {
            let count_result: CountResult =
                sql_query(format!("SELECT COUNT(*) as count FROM {};", table.name))
                    .get_result(conn)?;
            println!("Table '{}' has {} records.", table.name, count_result.count);
        }

        println!("Seeded data verification complete!");
        Ok(())
    }
}
