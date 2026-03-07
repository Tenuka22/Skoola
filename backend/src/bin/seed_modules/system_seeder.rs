use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::{DayType, RoleEnum};
use backend::models::system::calendar::SchoolCalendar;
use backend::models::system::setting::SchoolSetting;
use backend::database::tables::{RoleSet, RoleSetRole, UserPermission};
use backend::models::ids::IdPrefix;
use backend::schema::*;
use chrono::{Datelike, NaiveDate, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use std::collections::HashSet;

pub struct SystemSeeder;

impl SystemSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for SystemSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding System module...");

        // 1. school_calendar
        println!("Seeding school_calendar...");
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        for i in 0..10 {
            let date = start_date + chrono::Duration::days(i);
            let day_type = match date.weekday() {
                chrono::Weekday::Sat | chrono::Weekday::Sun => DayType::Weekend,
                _ => DayType::Working,
            };
            insert_into(school_calendar::table)
                .values(&SchoolCalendar {
                    date,
                    day_type: day_type.clone(),
                    name: None,
                    is_academic_day: day_type == DayType::Working,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;
        }

        // 2. school_settings
        println!("Seeding school_settings...");
        let settings = vec![
            ("school_name", "Skoola International"),
            ("contact_email", "info@skoola.com"),
        ];
        for (key, val) in settings {
            insert_into(school_settings::table)
                .values(&SchoolSetting {
                    setting_key: key.to_string(),
                    setting_value: val.to_string(),
                    description: Some("System setting".to_string()),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;
        }

        // 3. role_sets & roles
        println!("Seeding IAM structures...");
        let rs_id = next_id(conn, IdPrefix::ROLE_SET);
        insert_into(role_sets::table)
            .values(&RoleSet {
                id: rs_id.clone(),
                name: "Academic Staff".to_string(),
                description: Some("Teachers and Principals".to_string()),
            })
            .execute(conn)?;

        insert_into(role_set_roles::table)
            .values(&RoleSetRole {
                role_set_id: rs_id.clone(),
                role_id: RoleEnum::Teacher.to_string(),
            })
            .execute(conn)?;

        // 4. user_permissions
        if !context.user_ids.is_empty() {
            insert_into(user_permissions::table)
                .values(&UserPermission {
                    user_id: context.user_ids[0].clone(),
                    permission: "SystemAdmin".to_string(),
                })
                .execute(conn)?;
        }

        Ok(())
    }
}
