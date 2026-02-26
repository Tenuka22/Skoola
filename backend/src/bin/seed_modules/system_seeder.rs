use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::{DayType, RoleEnum};
use backend::models::auth::permission::{RolePermission, UserPermission, UserSet, UserSetUser};
use backend::models::resources::inventory::{UniformIssue, UniformItem};
use backend::models::system::calendar::SchoolCalendar;
use backend::models::system::setting::SchoolSetting;
use backend::schema::*;
use chrono::{Datelike, NaiveDate, Utc};
use diesel::insert_into;
use diesel::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;

#[derive(Insertable)]
#[diesel(table_name = role_sets)]
pub struct RoleSet {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = role_set_roles)]
pub struct RoleSetRole {
    pub role_set_id: String,
    pub role_id: String,
}

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
        seed_count_config: &crate::SeedCountConfig, // Add SeedCountConfig here
    ) -> Result<()> {
        println!("Seeding System module...");

        let mut rng = rand::thread_rng();

        // 1. School Calendar
        let mut calendar_data = Vec::new();
        let start_date = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        for i in 0..seed_count_config.calendar_entries {
            let date = start_date + chrono::Duration::days(i as i64);
            let day_type = match date.weekday() {
                chrono::Weekday::Sat | chrono::Weekday::Sun => DayType::Weekend,
                _ => DayType::Working,
            };
            let is_academic_day = matches!(day_type, DayType::Working);
            calendar_data.push(SchoolCalendar {
                date,
                day_type,
                name: None,
                is_academic_day,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });
        }
        insert_into(school_calendar::table)
            .values(&calendar_data)
            .execute(conn)?;
        println!("Seeded {} calendar entries.", calendar_data.len());

        // 2. School Settings
        let settings_data = vec![
            SchoolSetting {
                setting_key: "school_name".to_string(),
                setting_value: "Skoola International".to_string(),
                description: Some("Main school name".to_string()),
                updated_at: Utc::now().naive_utc(),
            },
            SchoolSetting {
                setting_key: "contact_email".to_string(),
                setting_value: "info@skoola.com".to_string(),
                description: Some("Main contact".to_string()),
                updated_at: Utc::now().naive_utc(),
            },
        ];
        insert_into(school_settings::table)
            .values(&settings_data)
            .execute(conn)?;
        println!("Seeded {} school settings.", settings_data.len());

        // 3. Uniform Items & Issues
        let uniforms_data: Vec<UniformItem> = (0..seed_count_config.uniform_items)
            .map(|i| UniformItem {
                id: generate_uuid(),
                item_name: format!(
                    "{} Uniform Item {}",
                    if i % 2 == 0 { "Boys" } else { "Girls" },
                    i + 1
                ),
                size: vec!["XS", "S", "M", "L", "XL"]
                    .choose(&mut rng)
                    .unwrap()
                    .to_string(),
                gender: if i % 2 == 0 {
                    "Male".to_string()
                } else {
                    "Female".to_string()
                },
                grade_level: if rng.gen_bool(0.5) {
                    Some(format!("Grade {}", rng.gen_range(1..=12)))
                } else {
                    None
                },
                price: rng.gen_range(1000.0..=5000.0),
                quantity: rng.gen_range(50..=200),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            })
            .collect();
        insert_into(uniform_items::table)
            .values(&uniforms_data)
            .execute(conn)?;
        println!("Seeded {} uniform items.", uniforms_data.len());

        if !context.student_ids.is_empty()
            && !context.staff_ids.is_empty()
            && !uniforms_data.is_empty()
        {
            let mut issues = Vec::new();
            for _ in 0..(context
                .student_ids
                .len()
                .min(seed_count_config.uniform_issues_per_student * uniforms_data.len()))
            {
                issues.push(UniformIssue {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    uniform_item_id: uniforms_data.choose(&mut rng).unwrap().id.clone(),
                    quantity: rng.gen_range(1..=3),
                    issue_date: random_datetime_in_past(0),
                    issued_by: get_random_id(&context.staff_ids),
                    amount_collected: rng.gen_range(1000.0..=5000.0),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                });
            }
            insert_into(uniform_issues::table)
                .values(&issues)
                .execute(conn)?;
            println!("Seeded {} uniform issues.", issues.len());
        }

        // 4. IAM - Role Sets, User Sets
        let role_set_id = generate_uuid();
        let role_set = RoleSet {
            id: role_set_id.clone(),
            name: "Academic Staff".to_string(),
            description: Some("Set for all teachers and academic leads".to_string()),
        };
        insert_into(role_sets::table)
            .values(&role_set)
            .execute(conn)?;

        let role_set_role = RoleSetRole {
            role_set_id: role_set_id.clone(),
            role_id: RoleEnum::Teacher.to_string(),
        };
        insert_into(role_set_roles::table)
            .values(&role_set_role)
            .execute(conn)?;

        let user_set_id = generate_uuid();
        let user_set = UserSet {
            id: user_set_id.clone(),
            name: "Senior Management".to_string(),
            description: Some("User set for school principals".to_string()),
        };
        insert_into(user_sets::table)
            .values(&user_set)
            .execute(conn)?;

        if !context.user_ids.is_empty() {
            let user_set_user = UserSetUser {
                user_set_id: user_set_id.clone(),
                user_id: context.user_ids[0].clone(),
            };
            insert_into(user_set_users::table)
                .values(&user_set_user)
                .execute(conn)?;
        }

        // 5. IAM Permissions
        let mut role_perms = Vec::new();
        role_perms.push(RolePermission {
            role_id: RoleEnum::Teacher.to_string(),
            permission: "StudentRead".to_string(),
        });
        role_perms.push(RolePermission {
            role_id: RoleEnum::Teacher.to_string(),
            permission: "AttendanceManage".to_string(),
        });
        insert_into(role_permissions::table)
            .values(&role_perms)
            .execute(conn)?;

        if !context.user_ids.is_empty() {
            let user_perm = UserPermission {
                user_id: context.user_ids[0].clone(),
                permission: "SystemAdmin".to_string(),
            };
            insert_into(user_permissions::table)
                .values(&user_perm)
                .execute(conn)?;
        }

        println!("Seeded IAM (role sets, user sets, permissions).");

        Ok(())
    }
}
