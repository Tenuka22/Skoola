use crate::config::Config;
use crate::database::tables::{LibraryBook, LibraryCategory, LibrarySettings};
use crate::errors::APIError;
use crate::faker::CustomFaker;
use crate::models::student::Student;
use crate::schema::{library_books, library_categories, library_settings, students};
use chrono::{Duration, Utc};
use diesel::SqliteConnection;
use diesel::prelude::*;

pub fn seed_all(
    conn: &mut SqliteConnection,
    _app_config: &Config,
    staff_ids: &[String],
) -> Result<(Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>), APIError> {
    let mut seeded_category_ids = Vec::new();
    let mut seeded_book_ids = Vec::new();
    let seeded_issue_ids = Vec::new();
    let mut seeded_setting_ids = Vec::new();

    let now = Utc::now().naive_utc();
    let two_years_ago = now - Duration::days(730);

    let students_data = students::table
        .select(Student::as_select())
        .load::<Student>(conn)?;
    let student_ids: Vec<String> = students_data.iter().map(|s| s.id.clone()).collect();

    if student_ids.is_empty() || staff_ids.is_empty() {
        return Ok((
            seeded_category_ids,
            seeded_book_ids,
            seeded_issue_ids,
            seeded_setting_ids,
        ));
    }

    // 1. Seed Library Categories
    let category_names = vec![
        "Science",
        "Mathematics",
        "Literature",
        "History",
        "Fiction",
        "Reference",
    ];
    for (i, name) in category_names.iter().enumerate() {
        let cat_id = (i + 1) as i32;
        let new_cat = LibraryCategory {
            id: cat_id,
            category_name: name.to_string(),
            description: None,
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        diesel::insert_into(library_categories::table)
            .values(&new_cat)
            .execute(conn)?;
        seeded_category_ids.push(cat_id);
    }

    // 2. Seed Library Books
    let mut book_id_counter = 1;
    for cat_id in &seeded_category_ids {
        for i in 1..=5 {
            let book_id = book_id_counter;
            book_id_counter += 1;
            let new_book = LibraryBook {
                id: book_id,
                isbn: Some(format!("ISBN-{}-{}", cat_id, i)),
                title: format!("Book {}-{}", cat_id, i),
                author: "Random Author".to_string(),
                publisher: None,
                category_id: *cat_id,
                quantity: 10,
                available_quantity: 10,
                rack_number: Some(format!("R-{}", book_id % 10)),
                added_date: now.date(),
                created_at: CustomFaker::date_time_between(two_years_ago, now),
                updated_at: CustomFaker::date_time_between(two_years_ago, now),
            };
            diesel::insert_into(library_books::table)
                .values(&new_book)
                .execute(conn)?;
            seeded_book_ids.push(book_id);
        }
    }

    // 3. Seed Library Settings
    let settings = LibrarySettings {
        id: 1,
        max_books_per_student: 3,
        max_books_per_staff: 10,
        issue_duration_days_student: 14,
        issue_duration_days_staff: 30,
        fine_per_day: 5.0,
        created_at: now,
        updated_at: now,
    };
    diesel::insert_into(library_settings::table)
        .values(&settings)
        .execute(conn)?;
    seeded_setting_ids.push(1);

    Ok((
        seeded_category_ids,
        seeded_book_ids,
        seeded_issue_ids,
        seeded_setting_ids,
    ))
}
