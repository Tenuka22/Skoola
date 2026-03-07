use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::models::ids::IdPrefix;
use backend::schema::*;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use std::collections::HashSet;

pub struct AuditLogSeeder;

impl AuditLogSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for AuditLogSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Audit Log module...");

        // 1. audit_log
        println!("Seeding audit_log...");
        // Since AuditLog might not be in tables.rs, let's use the tuple values correctly
        for i in 0..10 {
            insert_into(audit_log::table)
                .values((
                    audit_log::id.eq(next_id(conn, IdPrefix::AUDIT)),
                    audit_log::user_id.eq(get_random_id(&context.user_ids)),
                    audit_log::action_type.eq("CREATE"),
                    audit_log::table_name.eq("students"),
                    audit_log::record_pk.eq(get_random_id(&context.student_ids)),
                    audit_log::old_value_json.eq(None as Option<String>),
                    audit_log::new_value_json.eq(Some(format!(r#"{{"name": "New student {}"}}"#, i))),
                    audit_log::timestamp.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        Ok(())
    }
}
