use crate::config::Config;
use crate::database::enums::FeeFrequency;
use crate::database::tables::{FeeCategory, FeeStructure};
use crate::errors::APIError;
use crate::faker::CustomFaker;
pub use crate::models::finance::IncomeSource;
use crate::models::student::Student;
use crate::schema::{fee_categories, fee_structures, income_sources, students};
use chrono::{Duration, Utc};
use diesel::SqliteConnection;
use diesel::prelude::*;
use uuid::Uuid;

pub fn seed_all(
    conn: &mut SqliteConnection,
    _app_config: &Config,
    academic_year_ids: &[String],
    grade_level_ids: &[String],
    _staff_ids: &[String],
) -> Result<
    (
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    ),
    APIError,
> {
    let seeded_budget_category_ids = Vec::new();
    let seeded_budget_ids = Vec::new();
    let mut seeded_income_source_ids = Vec::new();
    let seeded_income_transaction_ids = Vec::new();
    let seeded_expense_category_ids = Vec::new();
    let seeded_expense_transaction_ids = Vec::new();
    let mut seeded_fee_category_ids = Vec::new();
    let mut seeded_fee_structure_ids = Vec::new();
    let seeded_student_fee_ids = Vec::new();
    let seeded_fee_payment_ids = Vec::new();
    let seeded_petty_cash_transaction_ids = Vec::new();
    let seeded_salary_component_ids = Vec::new();
    let seeded_salary_payment_ids = Vec::new();

    let now = Utc::now().naive_utc();
    let two_years_ago = now - Duration::days(730);

    let students_data = students::table
        .select(Student::as_select())
        .load::<Student>(conn)?;
    let _student_ids: Vec<String> = students_data.iter().map(|s| s.id.clone()).collect();

    // 1. Seed Fee Categories
    let fee_cat_names = vec![
        "Tuition Fee",
        "Library Fee",
        "Sports Fee",
        "Facility Fee",
        "Examination Fee",
    ];
    let mut fee_cats_to_insert = Vec::new();
    for name in fee_cat_names {
        let cat_id = Uuid::new_v4().to_string();
        let new_cat = FeeCategory {
            id: cat_id.clone(),
            name: name.to_string(),
            description: None,
            is_mandatory: true,
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        fee_cats_to_insert.push(new_cat);
        seeded_fee_category_ids.push(cat_id);
    }
    diesel::insert_into(fee_categories::table)
        .values(&fee_cats_to_insert)
        .execute(conn)?;

    // 2. Seed Fee Structures
    let mut fee_structures_to_insert = Vec::new();
    for ay_id in academic_year_ids {
        for grade_id in grade_level_ids {
            for cat_id in &seeded_fee_category_ids {
                let fs_id = Uuid::new_v4().to_string();
                let new_fs = FeeStructure {
                    id: fs_id.clone(),
                    grade_id: grade_id.clone(),
                    academic_year_id: ay_id.clone(),
                    category_id: cat_id.clone(),
                    amount: rand::Rng::gen_range(&mut rand::thread_rng(), 1000.0..5000.0),
                    due_date: now.date() + Duration::days(30),
                    frequency: FeeFrequency::Monthly,
                    created_at: CustomFaker::date_time_between(two_years_ago, now),
                    updated_at: CustomFaker::date_time_between(two_years_ago, now),
                };
                fee_structures_to_insert.push(new_fs);
                seeded_fee_structure_ids.push(fs_id);
            }
        }
    }
    diesel::insert_into(fee_structures::table)
        .values(&fee_structures_to_insert)
        .execute(conn)?;

    // 3. Seed Income Sources
    let income_names = vec![
        "School Fees",
        "Donations",
        "Government Grant",
        "Canteen Rent",
        "Hall Rent",
    ];
    let mut income_sources_to_insert = Vec::new();
    for name in income_names {
        let source_id = Uuid::new_v4().to_string();
        let new_source = IncomeSource {
            id: source_id.clone(),
            name: name.to_string(),
            description: None,
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        income_sources_to_insert.push(new_source);
        seeded_income_source_ids.push(source_id);
    }
    diesel::insert_into(income_sources::table)
        .values(&income_sources_to_insert)
        .execute(conn)?;

    Ok((
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
    ))
}
