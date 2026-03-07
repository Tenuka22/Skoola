use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::RoleEnum;
use backend::database::tables::*;
use backend::models::ids::IdPrefix;
use backend::schema::*;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use std::collections::HashSet;

pub struct CustomUserSeeder;

impl CustomUserSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for CustomUserSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        password_hash: &str,
        used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Custom User module...");

        let mut seed_user = |email: &str, name: &str, role: RoleEnum| -> Result<()> {
            let u_id = next_id(conn, IdPrefix::USER);
            let p_id = next_id(conn, IdPrefix::PROFILE);

            insert_into(users::table)
                .values(&User {
                    id: u_id.clone(),
                    email: email.to_string(),
                    password_hash: password_hash.to_string(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                    role,
                })
                .execute(conn)?;

            insert_into(profiles::table)
                .values(&Profile {
                    id: p_id.clone(),
                    name: name.to_string(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;

            insert_into(user_profiles::table)
                .values(&UserProfile {
                    user_id: u_id.clone(),
                    profile_id: p_id.clone(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;

            insert_into(user_security::table)
                .values(&UserSecurity {
                    user_id: u_id.clone(),
                    google_id: None,
                    github_id: None,
                    verification_token: None,
                    verification_sent_at: None,
                    password_reset_token: None,
                    password_reset_sent_at: None,
                    failed_login_attempts: 0,
                    lockout_until: None,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;

            insert_into(user_status::table)
                .values(&UserStatus {
                    user_id: u_id.clone(),
                    is_verified: true,
                    is_active: true,
                    disabled_at: None,
                    disabled_reason: None,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;

            context.user_ids.push(u_id);
            context.profile_ids.push(p_id);
            used_emails.insert(email.to_string());
            Ok(())
        };

        seed_user(
            "fulladmin.test@main.co",
            "Test Full Admin",
            RoleEnum::FullAdmin,
        )?;
        seed_user("teacher.test@main.co", "Test Teacher", RoleEnum::Teacher)?;
        seed_user("student.test@main.co", "Test Student", RoleEnum::Student)?;

        Ok(())
    }
}
