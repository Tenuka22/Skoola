use crate::config::Config;
use crate::errors::APIError;
use crate::handlers::system::seed::{
    academic_structure, co_curricular, exams_and_grading, financial, inventory_and_assets, library,
    students, users_and_staff,
};
use crate::schema::seeds;
use chrono::{NaiveDateTime, Utc};
use diesel::SqliteConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = seeds)]
pub struct Seed {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = seeds)]
pub struct NewSeed {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub created_at: NaiveDateTime,
}

pub fn seed_data(
    conn: &mut PooledConnection<ConnectionManager<SqliteConnection>>,
    app_config: &Config,
) -> Result<(), APIError> {
    diesel::sql_query("PRAGMA foreign_keys = OFF;").execute(conn)?;
    let result = conn.transaction::<_, APIError, _>(|conn| {
        // First, remove all previously seeded data
        unseed_data(conn)?;

        // 1. Users and Staff
        let (
            seeded_user_ids,
            _seeded_role_ids,
            _seeded_permission_ids,
            seeded_staff_ids,
            seeded_qualification_ids,
            seeded_employment_history_ids,
            seeded_department_ids,
            _seeded_staff_role_ids,
            seeded_staff_subject_ids,
            seeded_teacher_class_assignment_ids,
            seeded_teacher_subject_assignment_ids,
            seeded_attendance_ids,
            seeded_leave_ids,
            seeded_session_ids,
            seeded_user_permission_ids,
            _seeded_permission_set_ids,
            _seeded_role_set_ids,
            _seeded_user_set_ids,
        ) = users_and_staff::seed_all(conn, app_config)?;

        track_seeded_ids(conn, "users", seeded_user_ids)?;
        // Permissions not tracked via IDs anymore as they are static/enum based or not seeded as records
        track_seeded_ids(conn, "staff", seeded_staff_ids.clone())?;
        track_seeded_ids(conn, "staff_qualifications", seeded_qualification_ids)?;
        track_seeded_ids(
            conn,
            "staff_employment_history",
            seeded_employment_history_ids,
        )?;
        track_seeded_ids(conn, "staff_departments", seeded_department_ids)?;
        track_seeded_ids(conn, "staff_subjects", seeded_staff_subject_ids)?;
        track_seeded_ids(
            conn,
            "teacher_class_assignments",
            seeded_teacher_class_assignment_ids,
        )?;
        track_seeded_ids(
            conn,
            "teacher_subject_assignments",
            seeded_teacher_subject_assignment_ids,
        )?;
        track_seeded_ids(conn, "staff_attendance", seeded_attendance_ids)?;
        track_seeded_ids(conn, "staff_leaves", seeded_leave_ids)?;
        track_seeded_ids(conn, "sessions", seeded_session_ids)?;
        track_seeded_ids(conn, "user_permissions", seeded_user_permission_ids)?;

        // 2. Academic Structure
        let (
            seeded_academic_year_ids,
            seeded_grade_level_ids,
            seeded_stream_ids,
            seeded_grade_stream_ids,
            seeded_class_ids,
            seeded_subject_ids,
            seeded_grade_subject_ids,
            seeded_stream_subject_ids,
            seeded_class_subject_teacher_ids,
            seeded_timetable_ids,
        ) = academic_structure::seed_all(conn, app_config)?;

        track_seeded_ids(conn, "academic_years", seeded_academic_year_ids.clone())?;
        track_seeded_ids(conn, "grade_levels", seeded_grade_level_ids.clone())?;
        track_seeded_ids(conn, "streams", seeded_stream_ids)?;
        track_seeded_ids(conn, "grade_streams", seeded_grade_stream_ids)?;
        track_seeded_ids(conn, "classes", seeded_class_ids.clone())?;
        track_seeded_ids(conn, "subjects", seeded_subject_ids.clone())?;
        track_seeded_ids(conn, "grade_subjects", seeded_grade_subject_ids)?;
        track_seeded_ids(conn, "stream_subjects", seeded_stream_subject_ids)?;
        track_seeded_ids(
            conn,
            "class_subject_teachers",
            seeded_class_subject_teacher_ids,
        )?;
        track_seeded_ids(conn, "timetable", seeded_timetable_ids)?;

        // 3. Students
        let (
            seeded_student_ids,
            seeded_guardian_ids,
            seeded_medical_info_ids,
            seeded_emergency_contact_ids,
            seeded_previous_school_ids,
            seeded_class_assignment_ids,
            seeded_student_attendance_ids,
        ) = students::seed_all(
            conn,
            app_config,
            &seeded_academic_year_ids,
            &seeded_grade_level_ids,
            &seeded_class_ids,
            &seeded_staff_ids,
        )?;

        track_seeded_ids(conn, "students", seeded_student_ids)?;
        track_seeded_ids(conn, "student_guardians", seeded_guardian_ids)?;
        track_seeded_ids(conn, "student_medical_info", seeded_medical_info_ids)?;
        track_seeded_ids(
            conn,
            "student_emergency_contacts",
            seeded_emergency_contact_ids,
        )?;
        track_seeded_ids(conn, "student_previous_schools", seeded_previous_school_ids)?;
        track_seeded_ids(
            conn,
            "student_class_assignments",
            seeded_class_assignment_ids,
        )?;
        track_seeded_ids(conn, "student_attendance", seeded_student_attendance_ids)?;

        // 4. Exams and Grading
        let (
            seeded_exam_type_ids,
            seeded_exam_ids,
            seeded_grading_scheme_ids,
            seeded_grading_criteria_ids,
            seeded_student_mark_ids,
            seeded_term_ids,
        ) = exams_and_grading::seed_all(
            conn,
            app_config,
            &seeded_academic_year_ids,
            &seeded_class_ids,
            &seeded_subject_ids,
            &seeded_staff_ids,
        )?;

        track_seeded_ids(conn, "exam_types", seeded_exam_type_ids)?;
        track_seeded_ids(conn, "terms", seeded_term_ids)?;
        track_seeded_ids(conn, "exams", seeded_exam_ids)?;
        track_seeded_ids(conn, "student_marks", seeded_student_mark_ids)?;
        track_seeded_ids(conn, "grading_schemes", seeded_grading_scheme_ids)?;
        track_seeded_ids(conn, "grading_criteria", seeded_grading_criteria_ids)?;

        // 5. Financial
        let (
            seeded_budget_category_ids,
            seeded_budget_ids,
            seeded_income_source_ids,
            seeded_income_transaction_ids,
            seeded_expense_category_ids,
            seeded_expense_transaction_ids,
            seeded_fee_category_ids,
            seeded_fee_structure_ids,
            seeded_student_fee_ids,
            seeded_fee_payment_ids,
            seeded_petty_cash_transaction_ids,
            seeded_salary_component_ids,
            seeded_salary_payment_ids,
        ) = financial::seed_all(
            conn,
            app_config,
            &seeded_academic_year_ids,
            &seeded_grade_level_ids,
            &seeded_staff_ids,
        )?;

        track_seeded_ids(conn, "fee_categories", seeded_fee_category_ids)?;
        track_seeded_ids(conn, "fee_structures", seeded_fee_structure_ids)?;
        track_seeded_ids(conn, "student_fees", seeded_student_fee_ids)?;
        track_seeded_ids(conn, "fee_payments", seeded_fee_payment_ids)?;
        track_seeded_ids(conn, "budget_categories", seeded_budget_category_ids)?;
        track_seeded_ids(conn, "budgets", seeded_budget_ids)?;
        track_seeded_ids(conn, "income_sources", seeded_income_source_ids)?;
        track_seeded_ids(conn, "income_transactions", seeded_income_transaction_ids)?;
        track_seeded_ids(conn, "expense_categories", seeded_expense_category_ids)?;
        track_seeded_ids(conn, "expense_transactions", seeded_expense_transaction_ids)?;
        track_seeded_ids(
            conn,
            "petty_cash_transactions",
            seeded_petty_cash_transaction_ids,
        )?;
        track_seeded_ids(conn, "salary_components", seeded_salary_component_ids)?;
        track_seeded_ids(conn, "salary_payments", seeded_salary_payment_ids)?;

        // 6. Inventory and Assets
        let (
            seeded_asset_category_ids,
            seeded_inventory_item_ids,
            seeded_maintenance_request_ids,
            seeded_asset_allocation_ids,
            seeded_uniform_item_ids,
            seeded_uniform_issue_ids,
        ) = inventory_and_assets::seed_all(conn, app_config, &seeded_staff_ids)?;

        track_seeded_ids(conn, "asset_categories", seeded_asset_category_ids)?;
        track_seeded_ids(conn, "inventory_items", seeded_inventory_item_ids)?;
        track_seeded_ids(conn, "asset_allocations", seeded_asset_allocation_ids)?;
        track_seeded_ids(conn, "maintenance_requests", seeded_maintenance_request_ids)?;
        track_seeded_ids(conn, "uniform_items", seeded_uniform_item_ids)?;
        track_seeded_ids(conn, "uniform_issues", seeded_uniform_issue_ids)?;

        // 7. Library
        let (
            seeded_library_category_ids,
            seeded_library_book_ids,
            seeded_library_issue_ids,
            seeded_library_setting_ids,
        ) = library::seed_all(conn, app_config, &seeded_staff_ids)?;
        track_seeded_ids(
            conn,
            "library_categories",
            seeded_library_category_ids
                .into_iter()
                .map(|id| id.to_string())
                .collect(),
        )?;
        track_seeded_ids(
            conn,
            "library_books",
            seeded_library_book_ids
                .into_iter()
                .map(|id| id.to_string())
                .collect(),
        )?;
        track_seeded_ids(
            conn,
            "library_issues",
            seeded_library_issue_ids
                .into_iter()
                .map(|id| id.to_string())
                .collect(),
        )?;
        track_seeded_ids(
            conn,
            "library_settings",
            seeded_library_setting_ids
                .into_iter()
                .map(|id| id.to_string())
                .collect(),
        )?;

        // 8. Co-curricular
        let (
            seeded_sport_ids,
            seeded_sport_team_ids,
            seeded_sport_team_member_ids,
            seeded_sport_event_ids,
            seeded_sport_event_participant_ids,
            seeded_club_ids,
            seeded_club_member_ids,
            seeded_club_activity_ids,
            seeded_competition_ids,
            seeded_competition_participant_ids,
            seeded_cultural_event_ids,
            seeded_cultural_event_participant_ids,
            seeded_student_achievement_ids,
        ) = co_curricular::seed_all(conn, app_config)?;

        track_seeded_ids(conn, "clubs", seeded_club_ids)?;
        track_seeded_ids(conn, "club_members", seeded_club_member_ids)?;
        track_seeded_ids(conn, "club_activities", seeded_club_activity_ids)?;
        track_seeded_ids(conn, "sports", seeded_sport_ids)?;
        track_seeded_ids(conn, "sport_teams", seeded_sport_team_ids)?;
        track_seeded_ids(conn, "sport_team_members", seeded_sport_team_member_ids)?;
        track_seeded_ids(conn, "sport_events", seeded_sport_event_ids)?;
        track_seeded_ids(
            conn,
            "sport_event_participants",
            seeded_sport_event_participant_ids,
        )?;
        track_seeded_ids(conn, "competitions", seeded_competition_ids)?;
        track_seeded_ids(
            conn,
            "competition_participants",
            seeded_competition_participant_ids,
        )?;
        track_seeded_ids(conn, "cultural_events", seeded_cultural_event_ids)?;
        track_seeded_ids(
            conn,
            "cultural_event_participants",
            seeded_cultural_event_participant_ids,
        )?;
        track_seeded_ids(conn, "student_achievements", seeded_student_achievement_ids)?;

        Ok(())
    });
    diesel::sql_query("PRAGMA foreign_keys = ON;").execute(conn)?;
    result
}

pub fn track_seeded_ids(
    conn: &mut SqliteConnection,
    table_name: &str,
    record_ids: Vec<String>,
) -> Result<(), APIError> {
    let new_seeds: Vec<NewSeed> = record_ids
        .into_iter()
        .map(|record_id| NewSeed {
            id: Uuid::new_v4().to_string(),
            table_name: table_name.to_string(),
            record_id,
            created_at: Utc::now().naive_utc(),
        })
        .collect();

    diesel::insert_into(seeds::table)
        .values(&new_seeds)
        .execute(conn)?;
    Ok(())
}

pub fn unseed_data(conn: &mut SqliteConnection) -> Result<(), APIError> {
    diesel::sql_query("PRAGMA foreign_keys = OFF;").execute(conn)?;

    use crate::schema::{
        academic_years, al_exams, asset_allocations, asset_categories, budget_categories, budgets,
        class_subject_teachers, classes, club_activities, club_members, clubs,
        competition_participants, competitions, cultural_event_participants, cultural_events,
        exam_subjects, exam_types, exams, expense_categories, expense_transactions, fee_categories,
        fee_payments, fee_structures, grade_levels, grade_streams, grade_subjects,
        grading_criteria, grading_schemes, income_sources, income_transactions, inventory_items,
        library_books, library_categories, library_issues, library_settings, maintenance_requests,
        ol_exams, petty_cash_transactions, report_card_marks, report_cards, role_permissions,
        salary_components, salary_payments, scholarship_exams, sessions, sport_event_participants,
        sport_events, sport_team_members, sport_teams, sports, staff, staff_attendance,
        staff_departments, staff_employment_history, staff_leaves, staff_qualifications,
        staff_salaries, staff_subjects, stream_subjects, streams, student_achievements,
        student_attendance, student_class_assignments, student_emergency_contacts, student_fees,
        student_guardians, student_marks, student_medical_info, student_previous_schools,
        student_zscores, students, subjects, teacher_class_assignments,
        teacher_subject_assignments, terms, timetable, uniform_issues, uniform_items,
        user_permissions, user_set_permissions, user_set_users, user_sets, users,
        zscore_calculations,
    };

    // Explicitly delete test users and staff users by email pattern
    diesel::delete(users::table.filter(users::email.like("%.test@main.co"))).execute(conn)?;
    diesel::delete(users::table.filter(users::email.like("staff%@example.com"))).execute(conn)?;

    // Explicitly delete tables with hardcoded integer primary keys to prevent
    // unique constraint violations if previous seeding or tracking was incomplete.
    // Order matters due to foreign key constraints: issues -> books -> categories
    diesel::delete(library_issues::table).execute(conn)?;
    diesel::delete(library_books::table).execute(conn)?;
    diesel::delete(library_categories::table).execute(conn)?;
    diesel::delete(library_settings::table).execute(conn)?;
    diesel::delete(user_set_users::table).execute(conn)?;
    diesel::delete(user_sets::table).execute(conn)?;

    // Clear assignment tables
    diesel::delete(role_permissions::table).execute(conn)?;
    diesel::delete(user_permissions::table).execute(conn)?;
    diesel::delete(user_set_permissions::table).execute(conn)?;

    let seeded_entries = seeds::table.load::<Seed>(conn)?;

    let mut records_by_table: HashMap<String, Vec<String>> = HashMap::new();
    for entry in seeded_entries {
        records_by_table
            .entry(entry.table_name)
            .or_default()
            .push(entry.record_id);
    }

    let deletion_order = vec![
        "student_zscores",
        "report_card_marks",
        "al_exams",
        "ol_exams",
        "scholarship_exams",
        "student_achievements",
        "student_attendance",
        "student_class_assignments",
        "student_emergency_contacts",
        "fee_payments",
        "student_fees",
        "student_guardians",
        "student_marks",
        "student_medical_info",
        "student_previous_schools",
        "competition_participants",
        "cultural_event_participants",
        "club_members",
        "sport_event_participants",
        "sport_team_members",
        "uniform_issues",
        "asset_allocations",
        "maintenance_requests",
        "income_transactions",
        "expense_transactions",
        "petty_cash_transactions",
        "salary_payments",
        "staff_attendance",
        "staff_employment_history",
        "staff_leaves",
        "staff_qualifications",
        "staff_salaries",
        "staff_subjects",
        "teacher_class_assignments",
        "teacher_subject_assignments",
        "classes",
        "timetable",
        "exam_subjects",
        "grading_criteria",
        "budgets",
        "fee_structures",
        "grade_streams",
        "grade_subjects",
        "stream_subjects",
        "club_activities",
        "report_cards",
        "zscore_calculations",
        "user_set_users",
        "user_permissions",
        "sessions",
        "library_issues",
        "students",
        "staff",
        "academic_years",
        "grade_levels",
        "streams",
        "subjects",
        "exam_types",
        "terms",
        "exams",
        "fee_categories",
        "asset_categories",
        "inventory_items",
        "uniform_items",
        "budget_categories",
        "income_sources",
        "expense_categories",
        "salary_components",
        "grading_schemes",
        "clubs",
        "sports",
        "sport_teams",
        "competitions",
        "cultural_events",
        "library_categories",
        "library_books",
        "library_settings",
        "user_sets",
        "users",
        "staff_departments",
        "seeds",
    ];

    for table_name in deletion_order {
        if let Some(ids) = records_by_table.get(table_name) {
            let _delete_count = match table_name {
                // Tables with INTEGER primary keys
                "library_books" | "library_categories" | "library_issues" | "library_settings" => {
                    let int_ids: Vec<i32> =
                        ids.iter().filter_map(|s| s.parse::<i32>().ok()).collect();
                    match table_name {
                        "library_books" => diesel::delete(
                            library_books::table.filter(library_books::id.eq_any(&int_ids)),
                        )
                        .execute(conn)?,
                        "library_categories" => diesel::delete(
                            library_categories::table
                                .filter(library_categories::id.eq_any(&int_ids)),
                        )
                        .execute(conn)?,
                        "library_issues" => diesel::delete(
                            library_issues::table.filter(library_issues::id.eq_any(&int_ids)),
                        )
                        .execute(conn)?,
                        "library_settings" => diesel::delete(
                            library_settings::table.filter(library_settings::id.eq_any(&int_ids)),
                        )
                        .execute(conn)?,
                        _ => 0, // Should not happen
                    }
                }
                // Tables with VARCHAR/TEXT primary keys
                "academic_years" => {
                    diesel::delete(academic_years::table.filter(academic_years::id.eq_any(ids)))
                        .execute(conn)?
                }
                "al_exams" => diesel::delete(al_exams::table.filter(al_exams::id.eq_any(ids)))
                    .execute(conn)?,
                "asset_allocations" => diesel::delete(
                    asset_allocations::table.filter(asset_allocations::id.eq_any(ids)),
                )
                .execute(conn)?,
                "asset_categories" => {
                    diesel::delete(asset_categories::table.filter(asset_categories::id.eq_any(ids)))
                        .execute(conn)?
                }
                "budget_categories" => diesel::delete(
                    budget_categories::table.filter(budget_categories::id.eq_any(ids)),
                )
                .execute(conn)?,
                "budgets" => {
                    diesel::delete(budgets::table.filter(budgets::id.eq_any(ids))).execute(conn)?
                }
                "class_subject_teachers" => diesel::delete(
                    class_subject_teachers::table
                        .filter(class_subject_teachers::class_id.eq_any(ids)),
                )
                .execute(conn)?,
                "classes" => {
                    diesel::delete(classes::table.filter(classes::id.eq_any(ids))).execute(conn)?
                }
                "club_activities" => {
                    diesel::delete(club_activities::table.filter(club_activities::id.eq_any(ids)))
                        .execute(conn)?
                }
                "club_members" => {
                    diesel::delete(club_members::table.filter(club_members::club_id.eq_any(ids)))
                        .execute(conn)?
                }
                "clubs" => {
                    diesel::delete(clubs::table.filter(clubs::id.eq_any(ids))).execute(conn)?
                }
                "competition_participants" => diesel::delete(
                    competition_participants::table
                        .filter(competition_participants::competition_id.eq_any(ids)),
                )
                .execute(conn)?,
                "competitions" => {
                    diesel::delete(competitions::table.filter(competitions::id.eq_any(ids)))
                        .execute(conn)?
                }
                "cultural_event_participants" => diesel::delete(
                    cultural_event_participants::table
                        .filter(cultural_event_participants::event_id.eq_any(ids)),
                )
                .execute(conn)?,
                "cultural_events" => {
                    diesel::delete(cultural_events::table.filter(cultural_events::id.eq_any(ids)))
                        .execute(conn)?
                }
                "exam_subjects" => {
                    diesel::delete(exam_subjects::table.filter(exam_subjects::exam_id.eq_any(ids)))
                        .execute(conn)?
                }
                "exam_types" => {
                    diesel::delete(exam_types::table.filter(exam_types::id.eq_any(ids)))
                        .execute(conn)?
                }
                "exams" => {
                    diesel::delete(exams::table.filter(exams::id.eq_any(ids))).execute(conn)?
                }
                "expense_categories" => diesel::delete(
                    expense_categories::table.filter(expense_categories::id.eq_any(ids)),
                )
                .execute(conn)?,
                "expense_transactions" => diesel::delete(
                    expense_transactions::table.filter(expense_transactions::id.eq_any(ids)),
                )
                .execute(conn)?,
                "fee_categories" => {
                    diesel::delete(fee_categories::table.filter(fee_categories::id.eq_any(ids)))
                        .execute(conn)?
                }
                "fee_payments" => {
                    diesel::delete(fee_payments::table.filter(fee_payments::id.eq_any(ids)))
                        .execute(conn)?
                }
                "fee_structures" => {
                    diesel::delete(fee_structures::table.filter(fee_structures::id.eq_any(ids)))
                        .execute(conn)?
                }
                "grade_levels" => {
                    diesel::delete(grade_levels::table.filter(grade_levels::id.eq_any(ids)))
                        .execute(conn)?
                }
                "grade_streams" => {
                    diesel::delete(grade_streams::table.filter(grade_streams::grade_id.eq_any(ids)))
                        .execute(conn)?
                }
                "grade_subjects" => diesel::delete(
                    grade_subjects::table.filter(grade_subjects::grade_id.eq_any(ids)),
                )
                .execute(conn)?,
                "grading_criteria" => {
                    diesel::delete(grading_criteria::table.filter(grading_criteria::id.eq_any(ids)))
                        .execute(conn)?
                }
                "grading_schemes" => {
                    diesel::delete(grading_schemes::table.filter(grading_schemes::id.eq_any(ids)))
                        .execute(conn)?
                }
                "income_sources" => {
                    diesel::delete(income_sources::table.filter(income_sources::id.eq_any(ids)))
                        .execute(conn)?
                }
                "income_transactions" => diesel::delete(
                    income_transactions::table.filter(income_transactions::id.eq_any(ids)),
                )
                .execute(conn)?,
                "inventory_items" => {
                    diesel::delete(inventory_items::table.filter(inventory_items::id.eq_any(ids)))
                        .execute(conn)?
                }
                "maintenance_requests" => diesel::delete(
                    maintenance_requests::table.filter(maintenance_requests::id.eq_any(ids)),
                )
                .execute(conn)?,
                "ol_exams" => diesel::delete(ol_exams::table.filter(ol_exams::id.eq_any(ids)))
                    .execute(conn)?,
                "petty_cash_transactions" => diesel::delete(
                    petty_cash_transactions::table.filter(petty_cash_transactions::id.eq_any(ids)),
                )
                .execute(conn)?,
                "report_card_marks" => diesel::delete(
                    report_card_marks::table.filter(report_card_marks::id.eq_any(ids)),
                )
                .execute(conn)?,
                "report_cards" => {
                    diesel::delete(report_cards::table.filter(report_cards::id.eq_any(ids)))
                        .execute(conn)?
                }
                "salary_components" => diesel::delete(
                    salary_components::table.filter(salary_components::id.eq_any(ids)),
                )
                .execute(conn)?,
                "salary_payments" => diesel::delete(
                    salary_payments::table.filter(salary_payments::staff_id.eq_any(ids)),
                )
                .execute(conn)?,
                "scholarship_exams" => diesel::delete(
                    scholarship_exams::table.filter(scholarship_exams::id.eq_any(ids)),
                )
                .execute(conn)?,
                "sessions" => diesel::delete(sessions::table.filter(sessions::id.eq_any(ids)))
                    .execute(conn)?,
                "sport_event_participants" => diesel::delete(
                    sport_event_participants::table
                        .filter(sport_event_participants::event_id.eq_any(ids)),
                )
                .execute(conn)?,
                "sport_events" => {
                    diesel::delete(sport_events::table.filter(sport_events::id.eq_any(ids)))
                        .execute(conn)?
                }
                "sport_team_members" => diesel::delete(
                    sport_team_members::table.filter(sport_team_members::team_id.eq_any(ids)),
                )
                .execute(conn)?,
                "sport_teams" => {
                    diesel::delete(sport_teams::table.filter(sport_teams::id.eq_any(ids)))
                        .execute(conn)?
                }
                "sports" => {
                    diesel::delete(sports::table.filter(sports::id.eq_any(ids))).execute(conn)?
                }
                "staff" => {
                    diesel::delete(staff::table.filter(staff::id.eq_any(ids))).execute(conn)?
                }
                "staff_attendance" => {
                    diesel::delete(staff_attendance::table.filter(staff_attendance::id.eq_any(ids)))
                        .execute(conn)?
                }
                "staff_departments" => diesel::delete(
                    staff_departments::table.filter(staff_departments::id.eq_any(ids)),
                )
                .execute(conn)?,
                "staff_employment_history" => diesel::delete(
                    staff_employment_history::table
                        .filter(staff_employment_history::id.eq_any(ids)),
                )
                .execute(conn)?,
                "staff_leaves" => {
                    diesel::delete(staff_leaves::table.filter(staff_leaves::id.eq_any(ids)))
                        .execute(conn)?
                }
                "staff_qualifications" => diesel::delete(
                    staff_qualifications::table.filter(staff_qualifications::id.eq_any(ids)),
                )
                .execute(conn)?,
                "staff_salaries" => diesel::delete(
                    staff_salaries::table.filter(staff_salaries::staff_id.eq_any(ids)),
                )
                .execute(conn)?,
                "staff_subjects" => diesel::delete(
                    staff_subjects::table.filter(staff_subjects::staff_id.eq_any(ids)),
                )
                .execute(conn)?,
                "stream_subjects" => diesel::delete(
                    stream_subjects::table.filter(stream_subjects::stream_id.eq_any(ids)),
                )
                .execute(conn)?,
                "streams" => {
                    diesel::delete(streams::table.filter(streams::id.eq_any(ids))).execute(conn)?
                }
                "student_achievements" => diesel::delete(
                    student_achievements::table.filter(student_achievements::id.eq_any(ids)),
                )
                .execute(conn)?,
                "student_attendance" => diesel::delete(
                    student_attendance::table.filter(student_attendance::id.eq_any(ids)),
                )
                .execute(conn)?,
                "student_class_assignments" => diesel::delete(
                    student_class_assignments::table
                        .filter(student_class_assignments::id.eq_any(ids)),
                )
                .execute(conn)?,
                "student_emergency_contacts" => diesel::delete(
                    student_emergency_contacts::table
                        .filter(student_emergency_contacts::id.eq_any(ids)),
                )
                .execute(conn)?,
                "student_fees" => {
                    diesel::delete(student_fees::table.filter(student_fees::id.eq_any(ids)))
                        .execute(conn)?
                }
                "student_guardians" => diesel::delete(
                    student_guardians::table.filter(student_guardians::id.eq_any(ids)),
                )
                .execute(conn)?,
                "student_marks" => {
                    diesel::delete(student_marks::table.filter(student_marks::id.eq_any(ids)))
                        .execute(conn)?
                }
                "student_medical_info" => diesel::delete(
                    student_medical_info::table.filter(student_medical_info::id.eq_any(ids)),
                )
                .execute(conn)?,
                "student_previous_schools" => diesel::delete(
                    student_previous_schools::table
                        .filter(student_previous_schools::id.eq_any(ids)),
                )
                .execute(conn)?,
                "student_zscores" => diesel::delete(
                    student_zscores::table.filter(student_zscores::student_id.eq_any(ids)),
                )
                .execute(conn)?,
                "students" => diesel::delete(students::table.filter(students::id.eq_any(ids)))
                    .execute(conn)?,
                "subjects" => diesel::delete(subjects::table.filter(subjects::id.eq_any(ids)))
                    .execute(conn)?,
                "teacher_class_assignments" => diesel::delete(
                    teacher_class_assignments::table
                        .filter(teacher_class_assignments::id.eq_any(ids)),
                )
                .execute(conn)?,
                "teacher_subject_assignments" => diesel::delete(
                    teacher_subject_assignments::table
                        .filter(teacher_subject_assignments::id.eq_any(ids)),
                )
                .execute(conn)?,
                "terms" => {
                    diesel::delete(terms::table.filter(terms::id.eq_any(ids))).execute(conn)?
                }
                "timetable" => diesel::delete(timetable::table.filter(timetable::id.eq_any(ids)))
                    .execute(conn)?,
                "uniform_issues" => {
                    diesel::delete(uniform_issues::table.filter(uniform_issues::id.eq_any(ids)))
                        .execute(conn)?
                }
                "uniform_items" => {
                    diesel::delete(uniform_items::table.filter(uniform_items::id.eq_any(ids)))
                        .execute(conn)?
                }
                "user_permissions" => diesel::delete(
                    user_permissions::table.filter(user_permissions::user_id.eq_any(ids)),
                )
                .execute(conn)?,
                "user_set_users" => diesel::delete(
                    user_set_users::table.filter(user_set_users::user_id.eq_any(ids)),
                )
                .execute(conn)?,
                "user_sets" => diesel::delete(user_sets::table.filter(user_sets::id.eq_any(ids)))
                    .execute(conn)?,
                "users" => {
                    diesel::delete(users::table.filter(users::id.eq_any(ids))).execute(conn)?
                }
                "zscore_calculations" => diesel::delete(
                    zscore_calculations::table.filter(zscore_calculations::exam_id.eq_any(ids)),
                )
                .execute(conn)?,
                "seeds" => {
                    diesel::delete(seeds::table.filter(seeds::id.eq_any(ids))).execute(conn)?
                }
                _ => 0,
            };
        }
    }

    // Clear the tracking table itself
    diesel::delete(seeds::table).execute(conn)?;

    diesel::sql_query("PRAGMA foreign_keys = ON;").execute(conn)?;

    Ok(())
}
