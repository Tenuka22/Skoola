use backend::config::Config;
use backend::database::connection::establish_connection;
use backend::handlers::seed_runner::seed_data;
use std::thread;
use std::time::Duration;
use tracing::{info, error};
use tracing_subscriber::FmtSubscriber;

fn main() {
    // Initialize tracing
    FmtSubscriber::builder().init();

    info!("🚀 Starting Database Seeder...");

    dotenvy::dotenv().ok();

    let config = match Config::from_env() {
        Ok(cfg) => cfg,
        Err(e) => {
            error!("❌ Failed to load configuration: {}", e);
            std::process::exit(1);
        }
    };

    let pool = match establish_connection(&config.database_url) {
        Ok(p) => p,
        Err(e) => {
            error!("❌ Failed to establish database connection: {}", e);
            std::process::exit(1);
        }
    };

    let mut retry_count = 0;
    loop {
        retry_count += 1;
        info!("🌱 Seeding attempt #{}...", retry_count);

        match pool.get() {
            Ok(mut conn) => {
                match seed_data(&mut conn, &config) {
                    Ok(_) => {
                        info!("✅ Database seeded successfully!");
                        break;
                    }
                    Err(e) => {
                        error!("❌ Seeding failed: {}. Retrying in 5 seconds...", e);
                    }
                }
            }
            Err(e) => {
                error!("❌ Failed to get connection from pool: {}. Retrying in 5 seconds...", e);
            }
        }

        thread::sleep(Duration::from_secs(5));
    }
}
