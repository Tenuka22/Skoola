use backend::config::Config;
use backend::database::connection::establish_connection;
use backend::utils::security::hash_password; // Still needed for password hashing in modules
use anyhow::Result;
use super::seed_modules::SeedModule;
use super::seed_modules::message_seeder::MessageSeeder; // New import
use super::seed_modules::resource_management::ResourceManagementSeeder; // New import
use super::seed_modules::curriculum_management::CurriculumManagementSeeder; // New import
use super::seed_modules::behavior_management::BehaviorManagementSeeder; // New import
use super::seed_modules::audit_log::AuditLogSeeder; // New import
use super::seed_modules::exams::ExamsSeeder; // New import

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
    #[sql_type = "Text"]
    pub name: String,
}

pub fn delete_all_tables(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
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

fn main() -> Result<()> {

    let config = Config::from_env().expect("Failed to load config");
    let pool = establish_connection(&config.database_url).expect("Failed to establish connection");
    let mut connection = pool.get().expect("Failed to get connection from pool");
    let mut used_emails: HashSet<String> = HashSet::new();
    let default_password_hash = hash_password(config.seed_user_password.as_deref().unwrap_or("password123"))?;
    let mut seeder_context = super::seed_modules::SeederContext::new(); // Initialize SeederContext

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
        Box::new(MessageSeeder::new()),
        Box::new(ResourceManagementSeeder::new()),
        Box::new(CurriculumManagementSeeder::new()),
        Box::new(BehaviorManagementSeeder::new()),
        Box::new(AuditLogSeeder::new()),
        Box::new(ExamsSeeder::new()), // Add other seeders here
    ];

    for seeder in seeders {
        seeder.seed(&mut connection, &config, &default_password_hash, &mut used_emails, &mut seeder_context)?;
    }

    println!("Database seeding complete!");

    Ok(())
}