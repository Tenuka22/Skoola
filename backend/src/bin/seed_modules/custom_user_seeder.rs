use diesel::prelude::*;
use diesel::insert_into;
use anyhow::Result;
use backend::schema::{users, profiles, user_profiles};
use backend::config::Config;
use std::collections::HashSet;
use super::utils::*;
use super::{SeedModule, SeederContext};
use backend::models::auth::{NewProfile, NewUser, NewUserProfile};
use backend::database::enums::RoleEnum;
use chrono::Utc;
use crate::hash_password;

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
        config: &Config,
        password_hash: &str,
        used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Custom User module...");

        // Helper function to create a user and their profile
        let seed_user = |conn: &mut SqliteConnection,
                           password_hash: &str,
                           used_emails: &mut HashSet<String>,
                           context: &mut SeederContext,
                           email_prefix: &str,
                           email_domain: &str,
                           name: &str,
                           role: RoleEnum| -> Result<()> {
            let user_email = format!("{}{}", email_prefix, email_domain);
            let user_id = generate_uuid();
            let new_user = NewUser {
                id: user_id.clone(),
                email: user_email.clone(),
                password_hash: password_hash.to_string(),
                google_id: None,
                github_id: None,
                is_verified: true,
                verification_token: None,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                verification_sent_at: None,
                password_reset_token: None,
                password_reset_sent_at: None,
                failed_login_attempts: 0,
                lockout_until: None,
                role: role.clone(),
            };
            insert_into(users::table)
                .values(&new_user)
                .execute(conn)?;
            context.user_ids.push(user_id.clone());

            let profile_id = generate_uuid();
            let new_profile = NewProfile {
                id: profile_id.clone(),
                name: name.to_string(),
                address: Some(generate_random_address()),
                phone: Some(generate_random_phone_number()),
                photo_url: None,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            };
            insert_into(profiles::table)
                .values(&new_profile)
                .execute(conn)?;
            context.profile_ids.push(profile_id.clone());
            insert_into(user_profiles::table)
                .values(&NewUserProfile { user_id: user_id.clone(), profile_id: profile_id.clone(), created_at: Utc::now().naive_utc(), updated_at: Utc::now().naive_utc() })
                .execute(conn)?;
            println!("Seeded {} user: {}", role, user_email);
            Ok(())
        };

        // Generate a FullAdmin user
        seed_user(conn, password_hash, used_emails, context, "fulladmin.test", "@main.co", "Full Admin User", RoleEnum::FullAdmin)?;

        // Generate an Admin user
        seed_user(conn, password_hash, used_emails, context, &generate_random_email_prefix(), "@admin.com", "Admin User", RoleEnum::Admin)?;

        // Generate a Teacher user
        seed_user(conn, password_hash, used_emails, context, &generate_random_email_prefix(), "@teacher.com", "Teacher User", RoleEnum::Teacher)?;

        // Generate a Student user
        seed_user(conn, password_hash, used_emails, context, &generate_random_email_prefix(), "@student.com", "Student User", RoleEnum::Student)?;

        Ok(())
    }
}
