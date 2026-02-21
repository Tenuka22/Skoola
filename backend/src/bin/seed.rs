use backend::config::Config;
use backend::database::connection::establish_connection;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    // Future arguments can be added here
}

fn main() {
    dotenvy::dotenv().ok();

    let config = Config::from_env().expect("Failed to load config");
    let _pool = establish_connection(&config.database_url).expect("Failed to establish connection");

    let args = Args::parse();
    println!("Seeding the database with args: {:?}", args);
    println!("Database connection established.");
}
