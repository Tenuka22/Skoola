use backend::config::Config;
use backend::database::connection::establish_connection;
use clap::Parser;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use diesel::sql_query;
use diesel::sql_types::Text;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Future arguments can be added here
}

// Define a simple struct to deserialize table names from the database
#[derive(QueryableByName, Debug)]
struct TableName {
    #[sql_type = "Text"]
    name: String,
}

fn delete_all_tables(conn: &mut SqliteConnection) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Disable foreign key checks
    sql_query("PRAGMA foreign_keys = OFF;")
        .execute(conn)?;
    println!("Foreign key checks disabled.");

    // 2. Retrieve table names
    let table_names: Vec<TableName> = sql_query(
        "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' AND name NOT LIKE 'diesel_%';"
    )
    .load(conn)?;

    // 3. Drop tables
    for table in table_names {
        let drop_table_sql = format!("DROP TABLE IF EXISTS {};", table.name);
        sql_query(&drop_table_sql)
            .execute(conn)?;
        println!("Dropped table: {}", table.name);
    }

    // 4. Enable foreign key checks
    sql_query("PRAGMA foreign_keys = ON;")
        .execute(conn)?;
    println!("Foreign key checks enabled.");

    Ok(())
}

fn main() {
    dotenvy::dotenv().ok();

    let config = Config::from_env().expect("Failed to load config");
    let mut connection = establish_connection(&config.database_url).expect("Failed to establish connection");

    let args = Args::parse();
    println!("Seeding the database with args: {:?}", args);
    println!("Database connection established.");
    if let Some(password) = &config.seed_user_password {
        println!("Seed user password: {}", password);
    } else {
        println!("Seed user password not found in config.");
    }

    // Call the delete_all_tables function
    println!("Purging existing data...");
    if let Err(e) = delete_all_tables(&mut connection) {
        eprintln!("Error purging tables: {}", e);
        std::process::exit(1);
    }
    println!("Data purging complete.");
}
