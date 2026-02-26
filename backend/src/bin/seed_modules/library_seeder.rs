use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::models::resources::library::{NewLibraryBook, NewLibraryCategory, NewLibraryIssue};
use backend::schema::*;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;

#[derive(Insertable)]
#[diesel(table_name = library_settings)]
pub struct NewLibrarySettings {
    pub id: i32,
    pub max_books_per_student: i32,
    pub max_books_per_staff: i32,
    pub issue_duration_days_student: i32,
    pub issue_duration_days_staff: i32,
    pub fine_per_day: f32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

pub struct LibrarySeeder;

impl LibrarySeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for LibrarySeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
        seed_count_config: &crate::SeedCountConfig, // Add SeedCountConfig here
    ) -> Result<()> {
        println!("Seeding Library module...");

        let mut rng = rand::thread_rng();

        // 1. Library Categories
        let categories_data: Vec<NewLibraryCategory> = (0..seed_count_config.library_categories)
            .map(|i| NewLibraryCategory {
                category_name: format!("Category {}", i + 1),
                description: Some(format!("Description for Category {}", i + 1)),
            })
            .collect();
        insert_into(library_categories::table)
            .values(&categories_data)
            .execute(conn)?;

        // Fetch generated IDs
        let category_ids: Vec<i32> = library_categories::table
            .select(library_categories::id)
            .load(conn)?;
        context.library_category_ids = category_ids;
        println!(
            "Seeded {} library categories.",
            context.library_category_ids.len()
        );

        // 2. Library Books
        if !context.library_category_ids.is_empty() {
            let mut books_data = Vec::new();
            for i in 0..seed_count_config.library_books {
                books_data.push(NewLibraryBook {
                    isbn: Some(format!("978-0-123456-{:02}", i + 1)),
                    title: format!("Library Book {}", i + 1),
                    author: generate_random_name(),
                    publisher: Some(format!("Publisher {}", i + 1)),
                    category_id: *context.library_category_ids.choose(&mut rng).unwrap(),
                    quantity: rng.gen_range(1..=10),
                    available_quantity: rng.gen_range(1..=10),
                    rack_number: Some(format!("R-{}", i + 1)),
                    added_date: random_date_in_past(2),
                });
            }
            insert_into(library_books::table)
                .values(&books_data)
                .execute(conn)?;

            let book_ids: Vec<i32> = library_books::table.select(library_books::id).load(conn)?;
            context.library_book_ids = book_ids;
            println!("Seeded {} library books.", context.library_book_ids.len());
        }

        // 3. Library Settings
        let settings = NewLibrarySettings {
            id: 1,
            max_books_per_student: 2,
            max_books_per_staff: 5,
            issue_duration_days_student: 14,
            issue_duration_days_staff: 30,
            fine_per_day: 5.0,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        insert_into(library_settings::table)
            .values(&settings)
            .execute(conn)?;
        println!("Seeded library settings.");

        // 4. Library Issues
        if !context.library_book_ids.is_empty()
            && (!context.student_ids.is_empty() || !context.staff_ids.is_empty())
        {
            let mut issues = Vec::new();
            for _ in 0..seed_count_config.library_issues {
                let book_id = *context.library_book_ids.choose(&mut rng).unwrap();
                let is_student_issue = rng.gen_bool(0.7); // 70% chance to be a student issue

                let (student_id, staff_id) = if is_student_issue && !context.student_ids.is_empty()
                {
                    (Some(get_random_id(&context.student_ids)), None)
                } else if !context.staff_ids.is_empty() {
                    (None, Some(get_random_id(&context.staff_ids)))
                } else {
                    (None, None)
                };

                if student_id.is_some() || staff_id.is_some() {
                    // Only create if assigned to someone
                    issues.push(NewLibraryIssue {
                        book_id,
                        student_id,
                        staff_id,
                        issue_date: random_date_in_past(0),
                        due_date: random_date_in_past(0) + chrono::Duration::days(14),
                        issued_by: get_random_id(&context.user_ids), // Use a random user as issuer
                        status: "Issued".to_string(),
                        remarks: None,
                    });
                }
            }
            insert_into(library_issues::table)
                .values(&issues)
                .execute(conn)?;
            println!("Seeded {} library issues.", issues.len());
        }

        Ok(())
    }
}
