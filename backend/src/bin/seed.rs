mod seed_modules; // Declare the seed_modules module

use backend::config::Config;
use backend::database::connection::establish_connection;
use backend::utils::security::hash_password; // Still needed for password hashing in modules
use anyhow::Result;
use diesel::QueryableByName; // New import
use rand::Rng; // New import
use seed_modules::SeedModule;
use seed_modules::message_seeder::MessageSeeder; // New import
use seed_modules::academic_detail_seeder::AcademicDetailSeeder; // New import
use seed_modules::attendance_seeder::AttendanceSeeder; // New import
use seed_modules::extracurricular_seeder::ExtracurricularSeeder; // New import
use seed_modules::staff_student_detail_seeder::StaffStudentDetailSeeder; // New import
use seed_modules::library_seeder::LibrarySeeder; // New import
use seed_modules::system_seeder::SystemSeeder; // New import
use seed_modules::resource_management::ResourceManagementSeeder; // New import
use seed_modules::curriculum_management::CurriculumManagementSeeder; // New import
use seed_modules::behavior_management::BehaviorManagementSeeder; // New import
use seed_modules::audit_log::AuditLogSeeder; // New import
use seed_modules::exams::ExamsSeeder; // New import
use seed_modules::finance::FinanceSeeder; // New import
use seed_modules::seeder_verifier::SeederVerifier; // New import
use seed_modules::custom_user_seeder::CustomUserSeeder; // New import
use seed_modules::core_entities_seeder::CoreEntitiesSeeder;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use clap::Parser; // Keep for Args
use diesel::sqlite::SqliteConnection; // Keep for functions that receive connection
use diesel::{sql_query, RunQueryDsl}; // Keep for delete_all_tables
use diesel::sql_types::Text; // Keep for TableName
use uuid::Uuid; // Keep for generate_uuid
use std::collections::HashSet; // Keep for used_emails
use chrono::{NaiveDate, NaiveDateTime, Utc, Datelike, DateTime}; // Keep for date/time generation

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    // Future arguments can be added here
}

#[derive(QueryableByName, Debug)]
pub struct TableName {
    #[diesel(sql_type = Text)]
    pub name: String,
}

pub fn delete_all_tables(conn: &mut SqliteConnection) -> Result<()> {
    sql_query("PRAGMA foreign_keys = OFF;")
        .execute(conn)?;
    println!("Foreign key checks disabled.");

    let table_names: Vec<TableName> = sql_query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE 'diesel_%';"
    )
    .load(conn)?;

    for table in table_names {
        let drop_table_sql = format!("DROP TABLE IF EXISTS {};", table.name);
        sql_query(&drop_table_sql)
            .execute(conn)?;
        println!("Dropped table: {}", table.name);
    }

    sql_query("PRAGMA foreign_keys = ON;")
        .execute(conn)?;
    println!("Foreign key checks enabled.");

    Ok(())
}

pub fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn random_datetime_in_past(years: u32) -> NaiveDateTime {
    let mut rng = rand::thread_rng();
    let now = Utc::now().naive_utc();
    let years_ago_date = NaiveDate::from_ymd_opt(now.year() - years as i32, now.month(), now.day()).unwrap_or(now.date());
    let years_ago_datetime = NaiveDateTime::new(years_ago_date, now.time());
    
    let start_timestamp = years_ago_datetime.and_utc().timestamp();
    let end_timestamp = now.and_utc().timestamp();
    let random_timestamp = rng.gen_range(start_timestamp..=end_timestamp);
    DateTime::from_timestamp(random_timestamp, 0).unwrap().naive_utc()
}

pub fn random_date_in_past(years: u32) -> NaiveDate {
    let mut rng = rand::thread_rng();
    let now = Utc::now().naive_utc().date();
    let years_ago = NaiveDate::from_ymd_opt(now.year() - years as i32, now.month(), now.day()).unwrap_or(now);

    let start_timestamp = years_ago.and_hms_opt(0, 0, 0).unwrap().and_utc().timestamp();
    let end_timestamp = now.and_hms_opt(23, 59, 59).unwrap().and_utc().timestamp();
    let random_timestamp = rng.gen_range(start_timestamp..=end_timestamp);
    DateTime::from_timestamp(random_timestamp, 0).unwrap().date_naive()
}

// Struct to define the number of items to generate for each table/entity
pub struct SeedCountConfig {
    pub users: usize,
    pub staff: usize,
    pub students: usize,
    pub guardians: usize,
    pub classes: usize,
    pub academic_years: usize,
    pub grade_levels: usize,
    pub subjects: usize,
    pub terms: usize,
    pub conversations: usize,
    pub conversation_participants_per_conversation: usize,
    pub messages_per_conversation: usize,
    pub asset_categories: usize,
    pub inventory_items: usize,
    pub resources: usize,
    pub asset_allocations: usize,
    pub maintenance_requests: usize,
    pub resource_bookings: usize,
    pub curriculum_standards: usize,
    pub syllabi: usize,
    pub lesson_progress_entries: usize,
    pub behavior_incident_types: usize,
    pub behavior_incidents: usize,
    pub exam_types: usize,
    pub grading_schemes: usize,
    pub grading_criteria: usize,
    pub exams: usize,
    pub exam_subjects: usize,
    pub student_marks: usize,
    pub report_cards: usize,
    pub report_card_marks: usize,
    pub al_exams: usize,
    pub ol_exams: usize,
    pub scholarship_exams: usize,
    pub zscore_calculations: usize,
    pub student_zscores: usize,
    pub chart_of_accounts: usize,
    pub budget_categories: usize,
    pub budgets: usize,
    pub income_sources: usize,
    pub income_transactions: usize,
    pub expense_categories: usize,
    pub expense_transactions: usize,
    pub fee_categories: usize,
    pub fee_structures: usize,
    pub student_fees: usize,
    pub fee_payments: usize,
    pub salary_components: usize,
    pub staff_salaries: usize,
    pub salary_payments: usize,
    pub petty_cash_transactions: usize,
    pub general_ledger_entries: usize,
    pub library_categories: usize,
    pub library_books: usize,
    pub library_issues: usize,
    pub activity_types: usize,
    pub activities: usize,
    pub activity_participants_per_activity: usize,
    pub sports: usize,
    pub sport_teams: usize,
    pub clubs: usize,
    pub club_members_per_club: usize,
    pub club_activities_per_club: usize,
    pub competitions: usize,
    pub competition_participants_per_competition: usize,
    pub cultural_events: usize,
    pub cultural_event_participants_per_event: usize,
    pub student_achievements: usize,
    pub staff_departments: usize,
    pub staff_qualifications_per_staff: usize,
    pub staff_employment_history_per_staff: usize,
    pub staff_leaves_per_staff: usize,
    pub student_medical_info_per_student: usize,
    pub student_emergency_contacts_per_student: usize,
    pub student_previous_schools_per_student: usize,
    pub student_attendance_entries: usize,
    pub student_period_attendance_entries: usize,
    pub staff_attendance_entries: usize,
    pub exit_passes: usize,
    pub pre_approved_absences: usize,
    pub emergency_roll_calls: usize,
    pub emergency_roll_call_entries_per_roll_call: usize,
    pub attendance_audit_logs: usize,
    pub audit_log_entries: usize,
    pub calendar_entries: usize,
    pub grade_streams: usize,
    pub grade_streams_per_grade: usize,
    pub grade_subjects_per_grade: usize,
    pub stream_subjects_per_stream: usize,
    pub student_class_assignments: usize,
    pub subject_enrollments_per_student: usize,
    pub teacher_class_assignments_per_class: usize,
    pub teacher_subject_assignments_per_teacher: usize,
    pub class_subject_teachers_per_class: usize,
    pub timetable_entries_per_class_and_day: usize,
    pub substitutions: usize,
    pub uniform_items: usize,
    pub uniform_issues_per_student: usize,
}

impl Default for SeedCountConfig {
    fn default() -> Self {
        Self {
            users: 5000,
            staff: 500,
            students: 4000,
            guardians: 2000,
            classes: 200,
            academic_years: 3,
            grade_levels: 13,
            subjects: 20,
            terms: 3,
            conversations: 500,
            conversation_participants_per_conversation: 5,
            messages_per_conversation: 20,
            asset_categories: 10,
            inventory_items: 50,
            resources: 30,
            asset_allocations: 100,
            maintenance_requests: 50,
            resource_bookings: 75,
            curriculum_standards: 50,
            syllabi: 100,
            lesson_progress_entries: 150,
            behavior_incident_types: 10,
            behavior_incidents: 100,
            exam_types: 5,
            grading_schemes: 5,
            grading_criteria: 20,
            exams: 20,
            exam_subjects: 50,
            student_marks: 10000, // Thousands for marks
            report_cards: 2000,
            report_card_marks: 5000,
            al_exams: 20,
            ol_exams: 40,
            scholarship_exams: 20,
            zscore_calculations: 50,
            student_zscores: 100,
            chart_of_accounts: 10,
            budget_categories: 10,
            budgets: 50,
            income_sources: 5,
            income_transactions: 100,
            expense_categories: 10,
            expense_transactions: 100,
            fee_categories: 5,
            fee_structures: 50,
            student_fees: 200,
            fee_payments: 300,
            salary_components: 5,
            staff_salaries: 50,
            salary_payments: 100,
            petty_cash_transactions: 100,
            general_ledger_entries: 5000, // Thousands for ledger entries
            library_categories: 5,
            library_books: 50,
            library_issues: 10,
            activity_types: 5,
            activities: 25,
            activity_participants_per_activity: 10,
            sports: 5,
            sport_teams: 10,
            clubs: 10,
            club_members_per_club: 10,
            club_activities_per_club: 3,
            competitions: 10,
            competition_participants_per_competition: 50,
            cultural_events: 5,
            cultural_event_participants_per_event: 100,
            student_achievements: 100,
            staff_departments: 5,
            staff_qualifications_per_staff: 1,
            staff_employment_history_per_staff: 1,
            staff_leaves_per_staff: 3,
            student_medical_info_per_student: 1,
            student_emergency_contacts_per_student: 1,
            student_previous_schools_per_student: 1,
            student_attendance_entries: 2000,
            student_period_attendance_entries: 1000,
            staff_attendance_entries: 500,
            exit_passes: 100,
            pre_approved_absences: 50,
            emergency_roll_calls: 5,
            emergency_roll_call_entries_per_roll_call: 20,
            attendance_audit_logs: 100,
            audit_log_entries: 5000, // Thousands for audit logs
            calendar_entries: 1000,
            grade_streams: 2,
            grade_streams_per_grade: 2,
            grade_subjects_per_grade: 5,
            stream_subjects_per_stream: 3,
            student_class_assignments: 4000,
            subject_enrollments_per_student: 3,
            teacher_class_assignments_per_class: 1,
            teacher_subject_assignments_per_teacher: 2,
            class_subject_teachers_per_class: 4,
            timetable_entries_per_class_and_day: 4,
            substitutions: 50,
            uniform_items: 5,
            uniform_issues_per_student: 2,
        }
    }
}

fn main() -> Result<()> {

    let config = Config::from_env().expect("Failed to load config");
    let pool = establish_connection(&config.database_url).expect("Failed to establish connection");
    let mut connection = pool.get().expect("Failed to get connection from pool");
    let mut used_emails: HashSet<String> = HashSet::new();
    let default_password_hash = hash_password(config.seed_user_password.as_deref().unwrap_or("password123"))?;
    let mut seeder_context = seed_modules::SeederContext::new(); // Initialize SeederContext
    let seed_count_config = SeedCountConfig::default(); // Initialize seed count configuration

    let args = Args::parse();
    println!("Seeding the database with args: {:?}", args);
    println!("Database connection established.");
    if let Some(password) = &config.seed_user_password {
        println!("Seed user password: {}", password);
    } else {
        println!("Seed user password not found in config. Using 'password123'");
    }

    println!("Purging existing data...");
    delete_all_tables(&mut connection)?;
    println!("Data purging complete.");

    // Run migrations to recreate the schema
    println!("Running database migrations...");
    connection.run_pending_migrations(MIGRATIONS)
        .expect("Failed to run migrations");
    println!("Database migrations complete.");

    // Orchestrate seeding modules here
    let seeders: Vec<Box<dyn SeedModule>> = vec![
        Box::new(CustomUserSeeder::new()), // Custom user seeder should run first if custom roles are needed early
        Box::new(CoreEntitiesSeeder::new()), // This should run first
        Box::new(AcademicDetailSeeder::new()),
        Box::new(AttendanceSeeder::new()),
        Box::new(ExtracurricularSeeder::new()),
        Box::new(StaffStudentDetailSeeder::new()),
        Box::new(LibrarySeeder::new()),
        Box::new(SystemSeeder::new()),
        Box::new(MessageSeeder::new()),
        Box::new(ResourceManagementSeeder::new()),
        Box::new(CurriculumManagementSeeder::new()),
        Box::new(BehaviorManagementSeeder::new()),
        Box::new(AuditLogSeeder::new()),
        Box::new(ExamsSeeder::new()),
        Box::new(FinanceSeeder::new()),
        Box::new(SeederVerifier::new()), // Add other seeders here
    ];

    for seeder in seeders {
        seeder.seed(&mut connection, &config, &default_password_hash, &mut used_emails, &mut seeder_context, &seed_count_config)?;
    }

    println!("Database seeding complete!");

    Ok(())
}