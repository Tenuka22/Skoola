use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::{LibraryIssueStatus};
use backend::models::resources::library::{NewLibraryBook, NewLibraryCategory, NewLibraryIssue};
use backend::schema::*;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use rand::Rng;
use std::collections::HashSet;

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
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Library module...");

        let mut rng = rand::thread_rng();

        // 1. library_categories
        println!("Seeding library_categories...");
        let cats = vec![
            "Mathematics", "Physics", "Chemistry", "Biology", "Computer Science",
            "History", "Geography", "Literature", "Philosophy", "Art", "Music",
            "Fiction", "Non-Fiction", "Reference", "Encyclopedias"
        ];
        for name in cats {
            insert_into(library_categories::table)
                .values(&NewLibraryCategory {
                    category_name: name.to_string(),
                    description: Some(format!("Academic and reference books for {}", name)),
                })
                .execute(conn)?;
        }
        
        let cat_ids: Vec<i32> = library_categories::table.select(library_categories::id).load(conn)?;

        // 2. library_books
        println!("Seeding library_books...");
        for _ in 0..100 {
            insert_into(library_books::table)
                .values(&NewLibraryBook {
                    isbn: Some(format!("978-{:09}", rng.gen_range(100000000..999999999))),
                    title: generate_realistic_title(),
                    author: generate_random_name(),
                    publisher: Some("Academic Press".to_string()),
                    category_id: cat_ids[rng.gen_range(0..cat_ids.len())],
                    quantity: 5,
                    available_quantity: 5,
                    rack_number: Some(format!("R-{:02}-{:02}", rng.gen_range(1..10), rng.gen_range(1..20))),
                    added_date: Utc::now().date_naive(),
                })
                .execute(conn)?;
        }

        let book_ids: Vec<i32> = library_books::table.select(library_books::id).load(conn)?;

        // 3. library_settings
        println!("Seeding library_settings...");
        insert_into(library_settings::table)
            .values((
                library_settings::id.eq(1),
                library_settings::max_books_per_student.eq(3),
                library_settings::max_books_per_staff.eq(10),
                library_settings::issue_duration_days_student.eq(14),
                library_settings::issue_duration_days_staff.eq(30),
                library_settings::fine_per_day.eq(5.0),
                library_settings::created_at.eq(Utc::now().naive_utc()),
                library_settings::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;

        // 4. library_issues
        println!("Seeding library_issues...");
        for stu_id in context.student_ids.iter().take(50) {
            insert_into(library_issues::table)
                .values(&NewLibraryIssue {
                    book_id: book_ids[rng.gen_range(0..book_ids.len())],
                    student_id: Some(stu_id.clone()),
                    staff_id: None,
                    issue_date: Utc::now().date_naive(),
                    due_date: (Utc::now() + chrono::Duration::days(14)).date_naive(),
                    issued_by: get_random_id(&context.user_ids),
                    status: LibraryIssueStatus::Issued,
                    remarks: None,
                })
                .execute(conn)?;
        }

        Ok(())
    }
}
