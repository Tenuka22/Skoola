use crate::Config;
use anyhow::Result;
use diesel::sqlite::SqliteConnection;
use std::collections::HashSet; // Import Config from the main crate (seed.rs)

pub mod academic_detail_seeder;
pub mod attendance_seeder;
pub mod audit_log;
pub mod behavior_management;
pub mod core_entities_seeder;
pub mod curriculum_management;
pub mod custom_user_seeder;
pub mod exams;
pub mod extracurricular_seeder;
pub mod finance;
pub mod library_seeder;
pub mod message_seeder; // New
pub mod resource_management;
pub mod seeder_verifier;
pub mod staff_student_detail_seeder;
pub mod system_seeder;
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
    pub asset_category_ids: Vec<String>,
    pub inventory_item_ids: Vec<String>,
    pub resource_ids: Vec<String>,
    pub asset_allocation_ids: Vec<String>,
    pub curriculum_standard_ids: Vec<String>,
    pub syllabus_ids: Vec<String>,
    pub behavior_incident_type_ids: Vec<String>,
    pub exam_type_ids: Vec<String>,
    pub grading_scheme_ids: Vec<String>,
    pub exam_ids: Vec<String>,
    pub term_ids: Vec<String>,
    pub report_card_ids: Vec<String>,
    pub budget_category_ids: Vec<String>,
    pub income_source_ids: Vec<String>,
    pub expense_category_ids: Vec<String>,
    pub fee_category_ids: Vec<String>,
    pub salary_component_ids: Vec<String>,
    pub chart_of_account_ids: Vec<String>,
    pub stream_ids: Vec<String>,
    pub fee_structure_ids: Vec<String>,
    pub student_fee_ids: Vec<String>,
    pub activity_type_ids: Vec<String>,
    pub activity_ids: Vec<String>,
    pub club_ids: Vec<String>,
    pub sport_ids: Vec<String>,
    pub sport_team_ids: Vec<String>,
    pub competition_ids: Vec<String>,
    pub cultural_event_ids: Vec<String>,
    pub conversation_ids: Vec<String>,
    pub library_category_ids: Vec<i32>,
    pub library_book_ids: Vec<i32>,
    pub timetable_ids: Vec<String>,
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
            asset_category_ids: Vec::new(),
            inventory_item_ids: Vec::new(),
            resource_ids: Vec::new(),
            asset_allocation_ids: Vec::new(),
            curriculum_standard_ids: Vec::new(),
            syllabus_ids: Vec::new(),
            behavior_incident_type_ids: Vec::new(),
            exam_type_ids: Vec::new(),
            grading_scheme_ids: Vec::new(),
            exam_ids: Vec::new(),
            term_ids: Vec::new(),
            report_card_ids: Vec::new(),
            budget_category_ids: Vec::new(),
            income_source_ids: Vec::new(),
            expense_category_ids: Vec::new(),
            fee_category_ids: Vec::new(),
            salary_component_ids: Vec::new(),
            chart_of_account_ids: Vec::new(),
            stream_ids: Vec::new(),
            fee_structure_ids: Vec::new(),
            student_fee_ids: Vec::new(),
            activity_type_ids: Vec::new(),
            activity_ids: Vec::new(),
            club_ids: Vec::new(),
            sport_ids: Vec::new(),
            sport_team_ids: Vec::new(),
            competition_ids: Vec::new(),
            cultural_event_ids: Vec::new(),
            conversation_ids: Vec::new(),
            library_category_ids: Vec::new(),
            library_book_ids: Vec::new(),
            timetable_ids: Vec::new(),
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
        context: &mut SeederContext,
        seed_count_config: &crate::SeedCountConfig, // Add SeedCountConfig here
    ) -> Result<()>;
}
