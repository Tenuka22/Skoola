use diesel::prelude::*;
use diesel::insert_into;
use anyhow::Result;
use crate::schema::*;
use crate::Config;
use std::collections::HashSet;
use crate::bin::seed_modules::utils::*;
use crate::bin::seed_modules::{SeedModule, SeederContext};
use crate::models::{
    AuditLog,
};
use rand::Rng;

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
    ) -> Result<()> {
        println!("Seeding Audit Log module...");

        // Seed Audit Logs
        if context.user_ids.is_empty() {
            println!("Skipping AuditLog seeding: user_ids are empty. Ensure relevant seeders run first.");
        } else {
            let audit_logs_data = (1..=50).map(|i| {
                AuditLog {
                    id: generate_uuid(),
                    user_id: get_random_id(&context.user_ids),
                    action_type: match i % 3 {
                        0 => "CREATE".to_string(),
                        1 => "UPDATE".to_string(),
                        _ => "DELETE".to_string(),
                    },
                    table_name: match i % 5 {
                        0 => "users".to_string(),
                        1 => "students".to_string(),
                        2 => "staff".to_string(),
                        3 => "classes".to_string(),
                        _ => "inventory_items".to_string(),
                    },
                    record_pk: generate_uuid(),
                    old_value_json: if rand::thread_rng().gen_bool(0.5) { Some(format!(r#"{{"field": "old_value_{}"}}"#, i)) } else { None },
                    new_value_json: if rand::thread_rng().gen_bool(0.5) { Some(format!(r#"{{"field": "new_value_{}"}}"#, i)) } else { None },
                    timestamp: random_datetime_in_past(1),
                }
            }).collect::<Vec<AuditLog>>();

            insert_into(audit_log::table)
                .values(&audit_logs_data)
                .execute(conn)?;

            println!("Seeded {} audit log entries.", audit_logs_data.len());
        }

        Ok(())
    }
}
