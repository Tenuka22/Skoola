use anyhow::Result;
use backend::config::Config;
use backend::database::connection::establish_connection;
use chrono::{NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::sql_query;
use dotenvy::dotenv;
use std::collections::HashSet;

mod seed_modules;
use seed_modules::*;

pub struct SeedCountConfig {
    pub is_test: bool,
    pub users: usize,
    pub staff: usize,
    pub students: usize,
    pub guardians: usize,
    pub academic_years: usize,
    pub grade_levels: usize,
    pub terms: usize,
    pub subjects: usize,
    pub classes: usize,
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
    pub student_attendance_entries: usize,
    pub student_period_attendance_entries: usize,
    pub staff_attendance_entries: usize,
    pub exit_passes: usize,
    pub pre_approved_absences: usize,
    pub emergency_roll_calls: usize,
    pub emergency_roll_call_entries_per_roll_call: usize,
    pub attendance_audit_logs: usize,
    pub audit_log_entries: usize,
    pub behavior_incident_types: usize,
    pub behavior_incidents: usize,
    pub curriculum_standards: usize,
    pub syllabi: usize,
    pub lesson_progress_entries: usize,
    pub conversations: usize,
    pub messages_per_conversation: usize,
    pub conversation_participants_per_conversation: usize,
    pub library_categories: usize,
    pub library_books: usize,
    pub library_issues: usize,
    pub activity_types: usize,
    pub activities: usize,
    pub activity_participants_per_activity: usize,
    pub sports: usize,
    pub clubs: usize,
    pub club_activities_per_club: usize,
    pub sport_teams: usize,
    pub competitions: usize,
    pub cultural_events: usize,
    pub club_members_per_club: usize,
    pub competition_participants_per_competition: usize,
    pub cultural_event_participants_per_event: usize,
    pub student_achievements: usize,
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
    pub staff_departments: usize,
    pub staff_qualifications_per_staff: usize,
    pub staff_employment_history_per_staff: usize,
    pub staff_leaves_per_staff: usize,
    pub student_medical_info_per_student: usize,
    pub student_emergency_contacts_per_student: usize,
    pub student_previous_schools_per_student: usize,
    pub uniform_items: usize,
    pub uniform_issues_per_student: usize,
    pub calendar_entries: usize,
}

impl SeedCountConfig {
    pub fn new(is_test: bool) -> Self {
        if is_test {
            Self {
                is_test: true,
                users: 10,
                staff: 10,
                students: 10,
                guardians: 10,
                academic_years: 2,
                grade_levels: 13,
                terms: 3,
                subjects: 10,
                classes: 10,
                grade_streams: 10,
                grade_streams_per_grade: 2,
                grade_subjects_per_grade: 5,
                stream_subjects_per_stream: 5,
                student_class_assignments: 10,
                subject_enrollments_per_student: 5,
                teacher_class_assignments_per_class: 1,
                teacher_subject_assignments_per_teacher: 2,
                class_subject_teachers_per_class: 2,
                timetable_entries_per_class_and_day: 2,
                substitutions: 10,
                student_attendance_entries: 10,
                student_period_attendance_entries: 10,
                staff_attendance_entries: 10,
                exit_passes: 10,
                pre_approved_absences: 10,
                emergency_roll_calls: 5,
                emergency_roll_call_entries_per_roll_call: 5,
                attendance_audit_logs: 10,
                audit_log_entries: 10,
                behavior_incident_types: 10,
                behavior_incidents: 10,
                curriculum_standards: 10,
                syllabi: 10,
                lesson_progress_entries: 10,
                conversations: 10,
                messages_per_conversation: 5,
                conversation_participants_per_conversation: 2,
                library_categories: 5,
                library_books: 10,
                library_issues: 10,
                activity_types: 5,
                activities: 10,
                activity_participants_per_activity: 5,
                sports: 5,
                clubs: 5,
                club_activities_per_club: 2,
                sport_teams: 5,
                competitions: 5,
                cultural_events: 5,
                club_members_per_club: 5,
                competition_participants_per_competition: 5,
                cultural_event_participants_per_event: 5,
                student_achievements: 10,
                chart_of_accounts: 10,
                budget_categories: 5,
                budgets: 5,
                income_sources: 5,
                income_transactions: 10,
                expense_categories: 5,
                expense_transactions: 10,
                fee_categories: 5,
                fee_structures: 10,
                student_fees: 10,
                fee_payments: 10,
                salary_components: 5,
                staff_salaries: 10,
                salary_payments: 10,
                petty_cash_transactions: 10,
                general_ledger_entries: 10,
                staff_departments: 5,
                staff_qualifications_per_staff: 2,
                staff_employment_history_per_staff: 1,
                staff_leaves_per_staff: 2,
                student_medical_info_per_student: 1,
                student_emergency_contacts_per_student: 1,
                student_previous_schools_per_student: 1,
                uniform_items: 5,
                uniform_issues_per_student: 1,
                calendar_entries: 10,
            }
        } else {
            Self {
                is_test: false,
                users: 1000,
                staff: 500,
                students: 2000,
                guardians: 1000,
                academic_years: 2,
                grade_levels: 13,
                terms: 3,
                subjects: 100,
                classes: 100,
                grade_streams: 20,
                grade_streams_per_grade: 10,
                grade_subjects_per_grade: 20,
                stream_subjects_per_stream: 20,
                student_class_assignments: 2000,
                subject_enrollments_per_student: 10,
                teacher_class_assignments_per_class: 3,
                teacher_subject_assignments_per_teacher: 10,
                class_subject_teachers_per_class: 15,
                timetable_entries_per_class_and_day: 8,
                substitutions: 1000,
                student_attendance_entries: 5000,
                student_period_attendance_entries: 10000,
                staff_attendance_entries: 2000,
                exit_passes: 1000,
                pre_approved_absences: 1000,
                emergency_roll_calls: 20,
                emergency_roll_call_entries_per_roll_call: 200,
                attendance_audit_logs: 1000,
                audit_log_entries: 2000,
                behavior_incident_types: 50,
                behavior_incidents: 2000,
                curriculum_standards: 100,
                syllabi: 500,
                lesson_progress_entries: 1000,
                conversations: 500,
                messages_per_conversation: 50,
                conversation_participants_per_conversation: 10,
                library_categories: 50,
                library_books: 1000,
                library_issues: 2000,
                activity_types: 50,
                activities: 500,
                activity_participants_per_activity: 100,
                sports: 50,
                clubs: 100,
                club_activities_per_club: 20,
                sport_teams: 100,
                competitions: 50,
                cultural_events: 20,
                club_members_per_club: 100,
                competition_participants_per_competition: 50,
                cultural_event_participants_per_event: 100,
                student_achievements: 1000,
                chart_of_accounts: 200,
                budget_categories: 50,
                budgets: 100,
                income_sources: 50,
                income_transactions: 1000,
                expense_categories: 50,
                expense_transactions: 1000,
                fee_categories: 20,
                fee_structures: 500,
                student_fees: 2000,
                fee_payments: 5000,
                salary_components: 50,
                staff_salaries: 1000,
                salary_payments: 2000,
                petty_cash_transactions: 1000,
                general_ledger_entries: 5000,
                staff_departments: 20,
                staff_qualifications_per_staff: 5,
                staff_employment_history_per_staff: 3,
                staff_leaves_per_staff: 10,
                student_medical_info_per_student: 1,
                student_emergency_contacts_per_student: 3,
                student_previous_schools_per_student: 1,
                uniform_items: 100,
                uniform_issues_per_student: 5,
                calendar_entries: 1000,
            }
        }
    }
}

pub fn generate_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}

pub fn random_date_in_past(years: u32) -> NaiveDate {
    let now = Utc::now().date_naive();
    let days = rand::random::<u32>() % (365 * years + 1).max(1);
    now - chrono::Duration::days(days as i64)
}

pub fn random_datetime_in_past(years: u32) -> NaiveDateTime {
    let now = Utc::now().naive_utc();
    let seconds = rand::random::<u32>() % (365 * 24 * 3600 * years + 1).max(1);
    now - chrono::Duration::seconds(seconds as i64)
}

fn truncate_all_tables(conn: &mut SqliteConnection) -> Result<()> {
    println!("Truncating all tables...");
    let tables: Vec<String> = sql_query("SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE 'diesel_%' AND name != 'id_sequences';")
        .load::<crate::seed_modules::seeder_verifier::TableName>(conn)?
        .into_iter()
        .map(|t| t.name)
        .collect();

    sql_query("PRAGMA foreign_keys = OFF;").execute(conn)?;
    for table in tables {
        sql_query(format!("DELETE FROM {};", table)).execute(conn)?;
    }
    sql_query("PRAGMA foreign_keys = ON;").execute(conn)?;
    Ok(())
}

fn main() -> Result<()> {
    dotenv().ok();
    let config = Config::from_env().expect("Failed to load config");
    let pool = establish_connection(&config.database_url).expect("Failed to create DB pool");
    let mut conn = pool.get().expect("Failed to get connection from pool");

    truncate_all_tables(&mut conn)?;

    let password_hash = if let Some(pwd) = &config.seed_user_password {
        backend::utils::security::hash_password(pwd).unwrap()
    } else {
        backend::utils::security::hash_password("password123").unwrap()
    };
    let mut used_emails = HashSet::new();
    let mut context = SeederContext::new();
    
    // START IN NORMAL MODE (Ramped down slightly for speed)
    let seed_count_config = SeedCountConfig::new(false);

    println!("Starting database seeding (Thousands Mode)...");

    let seeders: Vec<Box<dyn SeedModule>> = vec![
        Box::new(core_entities_seeder::CoreEntitiesSeeder::new()),
        Box::new(custom_user_seeder::CustomUserSeeder::new()),
        Box::new(academic_detail_seeder::AcademicDetailSeeder::new()),
        Box::new(staff_student_detail_seeder::StaffStudentDetailSeeder::new()),
        Box::new(curriculum_management::CurriculumManagementSeeder::new()),
        Box::new(exams::ExamsSeeder::new()),
        Box::new(extracurricular_seeder::ExtracurricularSeeder::new()),
        Box::new(attendance_seeder::AttendanceSeeder::new()),
        Box::new(behavior_management::BehaviorManagementSeeder::new()),
        Box::new(finance::FinanceSeeder::new()),
        Box::new(library_seeder::LibrarySeeder::new()),
        Box::new(message_seeder::MessageSeeder::new()),
        Box::new(resource_management::ResourceManagementSeeder::new()),
        Box::new(advanced_modules_seeder::AdvancedModulesSeeder::new()),
        Box::new(audit_log::AuditLogSeeder::new()),
        Box::new(system_seeder::SystemSeeder::new()),
        Box::new(seeder_verifier::SeederVerifier::new()),
    ];

    for seeder in seeders {
        seeder.seed(
            &mut conn,
            &config,
            &password_hash,
            &mut used_emails,
            &mut context,
            &seed_count_config,
        )?;
    }

    println!("Database seeding completed successfully!");

    Ok(())
}
