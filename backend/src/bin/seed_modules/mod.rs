use diesel::sqlite::SqliteConnection;
use anyhow::Result;
use std::collections::HashSet;
use crate::Config; // Import Config from the main crate (seed.rs)

pub mod utils;

pub struct SeederContext {
    // Vectors to hold IDs of seeded entities
    pub academic_year_ids: Vec<String>,
    pub grade_level_ids: Vec<String>,
    pub subject_ids: Vec<String>,
    pub profile_ids: Vec<String>, // includes admin, teacher, guardian, student profiles
    pub user_ids: Vec<String>,    // includes admin, teacher, guardian, student users
    pub staff_ids: Vec<String>,
    pub student_ids: Vec<String>,
    pub class_ids: Vec<String>,
    // Add more as needed
}

impl SeederContext {
    pub fn new() -> Self {
        Self {
            academic_year_ids: Vec::new(),
            grade_level_ids: Vec::new(),
            subject_ids: Vec::new(),
            profile_ids: Vec::new(),
            user_ids: Vec::new(),
            staff_ids: Vec::new(),
            student_ids: Vec::new(),
            class_ids: Vec::new(),
        }
    }
}

pub trait SeedModule {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        config: &Config,
        password_hash: &str,
        used_emails: &mut HashSet<String>,
        context: &mut SeederContext, // Add SeederContext here
    ) -> Result<()>;
}