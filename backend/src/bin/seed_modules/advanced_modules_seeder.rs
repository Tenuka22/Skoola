use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::*;
use backend::models::ids::IdPrefix;
use backend::schema::*;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use std::collections::HashSet;

pub struct AdvancedModulesSeeder;

impl AdvancedModulesSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for AdvancedModulesSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Advanced Modules (Final Pass)...");

        // 1. IAM/Auth/Permissions
        println!("Seeding Permission details...");
        let roles = vec!["Admin", "Teacher", "Student", "Guest", "Parent"];
        for r_id in roles {
            for p in &["UserRead", "StaffRead", "StudentRead"] {
                insert_into(role_permissions::table)
                    .values(&(
                        role_permissions::role_id.eq(r_id),
                        role_permissions::permission.eq(p),
                    ))
                    .execute(conn).ok();
            }
        }

        // 2. Uniform Items
        println!("Seeding uniform_items...");
        for i in 1..=10 {
            insert_into(uniform_items::table)
                .values(&(
                    uniform_items::id.eq(next_id(conn, IdPrefix::PROPERTY)),
                    uniform_items::item_name.eq(format!("Uniform Shirt {}", i)),
                    uniform_items::size.eq("M"),
                    uniform_items::gender.eq("Other"),
                    uniform_items::price.eq(1200.0),
                    uniform_items::quantity.eq(100),
                    uniform_items::created_at.eq(Utc::now().naive_utc()),
                    uniform_items::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn).ok();
        }

        // 3. Staff Detailed (Gap Fill)
        println!("Seeding Staff HR details...");
        for s_id in context.staff_ids.iter().take(100) {
            insert_into(staff_cvs::table)
                .values(&(
                    staff_cvs::id.eq(next_id(conn, IdPrefix::STAFF)),
                    staff_cvs::staff_id.eq(s_id.clone()),
                    staff_cvs::file_name.eq("Resume.pdf"),
                    staff_cvs::file_url.eq("http://example.com/cv.pdf"),
                    staff_cvs::file_type.eq("pdf"),
                    staff_cvs::uploaded_at.eq(Utc::now().naive_utc()),
                    staff_cvs::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn).ok();

            insert_into(staff_subject_expertise::table)
                .values(&(
                    staff_subject_expertise::staff_id.eq(s_id.clone()),
                    staff_subject_expertise::subject_id.eq(get_random_id(&context.subject_ids)),
                    staff_subject_expertise::expertise_level.eq("Advanced"),
                    staff_subject_expertise::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn).ok();
        }

        // 4. Student Health (Gap Fill)
        println!("Seeding Student Health details...");
        for stu_id in context.student_ids.iter().take(100) {
            insert_into(student_allergies::table)
                .values(&(
                    student_allergies::id.eq(next_id(conn, IdPrefix::STUDENT)),
                    student_allergies::student_id.eq(stu_id.clone()),
                    student_allergies::allergen_type.eq("Food"),
                    student_allergies::allergen_name.eq("Nuts"),
                    student_allergies::reaction_severity.eq("Moderate"),
                    student_allergies::created_at.eq(Utc::now().naive_utc()),
                    student_allergies::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn).ok();
        }

        // 5. Co-curricular (Gap Fill)
        println!("Seeding Sport Teams and Members...");
        for s_id in context.sport_ids.iter().take(5) {
            let team_id = next_id(conn, IdPrefix::CO_CURRICULAR);
            insert_into(sport_teams::table)
                .values(&(
                    sport_teams::id.eq(team_id.clone()),
                    sport_teams::sport_id.eq(s_id.clone()),
                    sport_teams::team_name.eq(format!("Team {}", generate_realistic_title())),
                    sport_teams::grade_level.eq("Senior"),
                    sport_teams::coach_id.eq(get_random_id(&context.staff_ids)),
                    sport_teams::created_at.eq(Utc::now().naive_utc()),
                    sport_teams::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn).ok();

            for stu_id in context.student_ids.iter().take(10) {
                insert_into(sport_team_members::table)
                    .values(&(
                        sport_team_members::team_id.eq(team_id.clone()),
                        sport_team_members::student_id.eq(stu_id.clone()),
                        sport_team_members::position.eq(Some("Player")),
                        sport_team_members::joined_date.eq(Utc::now().date_naive()),
                        sport_team_members::created_at.eq(Utc::now().naive_utc()),
                        sport_team_members::updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(conn).ok();
            }
        }

        // 6. Tokens & Sessions
        println!("Seeding Auth Tokens and Sessions...");
        for u_id in context.user_ids.iter().take(500) {
            let tid = next_id(conn, IdPrefix::AUTH_TOKEN);
            insert_into(auth_tokens::table)
                .values(&(
                    auth_tokens::id.eq(tid),
                    auth_tokens::user_id.eq(u_id.clone()),
                    auth_tokens::token_hash.eq(format!("hash-{}", u_id)),
                    auth_tokens::token_type.eq(AuthTokenType::Refresh),
                    auth_tokens::issued_at.eq(Utc::now().naive_utc()),
                    auth_tokens::expires_at.eq(Utc::now().naive_utc()),
                    auth_tokens::is_active.eq(true),
                ))
                .execute(conn).ok();

            insert_into(sessions::table)
                .values(&(
                    sessions::id.eq(next_id(conn, IdPrefix::SESSION)),
                    sessions::user_id.eq(u_id.clone()),
                    sessions::created_at.eq(Utc::now().naive_utc()),
                    sessions::expires_at.eq(Utc::now().naive_utc()),
                    sessions::is_active.eq(true),
                ))
                .execute(conn).ok();
        }

        // 7. Curriculum Details
        println!("Seeding Curriculum details...");
        for sub_id in context.subject_ids.iter().take(10) {
            insert_into(substitution_plans::table)
                .values(&(
                    substitution_plans::id.eq(next_id(conn, IdPrefix::SUBSTITUTION_PLAN)),
                    substitution_plans::subject_id.eq(sub_id.clone()),
                    substitution_plans::medium.eq("English"),
                    substitution_plans::plan_name.eq(generate_realistic_title()),
                    substitution_plans::created_at.eq(Utc::now().naive_utc()),
                    substitution_plans::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn).ok();
        }

        println!("Seeding Advanced Modules Completed!");
        Ok(())
    }
}
