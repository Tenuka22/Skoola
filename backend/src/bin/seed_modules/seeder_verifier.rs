use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::Text;
use anyhow::Result;
use crate::Config;
use std::collections::HashSet;
use crate::bin::seed_modules::{SeedModule, SeederContext};

#[derive(QueryableByName, Debug)]
pub struct TableName {
    #[sql_type = "Text"]
    pub name: String,
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
    ) -> Result<()> {
        println!("Verifying seeded data counts...");

        let table_names: Vec<TableName> = sql_query(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE 'diesel_%';"
        )
        .load(conn)?;

        for table in table_names {
            let count: i64 = sql_query(format!("SELECT COUNT(*) FROM {};", table.name))
                .load(conn)?
                .into_iter()
                .map(|row| diesel::row::Row::get::<i64, _>(&row, 0))
                .next()
                .unwrap_or(0);
            println!("Table '{}' has {} records.", table.name, count);
        }

        println!("Seeded data verification complete!");
        Ok(())
    }
}
