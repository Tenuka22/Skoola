use actix_cors::Cors;
use actix_web::{App, HttpServer, web::Data};
use apistos::{
    app::{BuildConfig, OpenApiWrapper},
    spec::Spec,
};
use apistos_models::info::Info;
use apistos_scalar::ScalarConfig;
use config::Config;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, FmtSubscriber};
use tracing_actix_web::TracingLogger;
use tokio::time::{interval, Duration}; // Add this line
use crate::database::connection::{DbPool, establish_connection}; // Updated import
use crate::errors::APIError;
use crate::services::cleanup::remove_unverified_users; // Add this line
use crate::services::email::EmailService; // Add this line

mod config;
mod database;
mod errors;
mod handlers;
mod models;
mod routes;
mod services;
mod utils;
mod schema;

#[derive(Clone)] // Add Clone derive
pub struct AppState {
    config: Config,
    db_pool: DbPool,
    email_service: EmailService, // Add this line
}

#[actix_web::main]
async fn main() -> Result<(), APIError> {
    // Initialize tracing subscriber
    FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("╔════════════════════════════════════════╗");
    info!("║   🚀 Skoola Backend Starting Up       ║");
    info!("╚════════════════════════════════════════╝");

    dotenvy::dotenv().ok();

    let config = Config::from_env()?;

    info!("📍 Server will bind to {}", config.server_url());
    info!("📚 API Documentation: {}", config.docs_url());
    info!("📄 OpenAPI Spec: {}", config.openapi_url());
    info!("🔒 CORS allowed origin: {}", config.allowed_origin);

    let spec = Spec {
        info: Info {
            title: config.api_title.clone(),
            description: Some(config.api_description.clone()),
            version: config.api_version.clone(),
            ..Default::default()
        },
        ..Default::default()
    };

    let bind_address = (config.host.clone(), config.port);
    let allowed_origin = config.allowed_origin.clone();

    let pool = establish_connection(&config.database_url)
        .map_err(|e| APIError::internal(format!("Failed to establish database connection pool: {}", e).as_str()))?;
    
    let email_service = EmailService::new(config.clone()); // Initialize EmailService

    let app_data = Data::new(AppState {
        config: config.clone(),
        db_pool: pool.clone(),
        email_service: email_service.clone(), // Pass EmailService to AppState
    });

    // Spawn background task for removing unverified users
    let cleanup_app_data = app_data.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(3600)); // Every 1 hour
        loop {
            interval.tick().await;
            remove_unverified_users(cleanup_app_data.clone()).await;
        }
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&allowed_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Authorization", "Content-Type", "Accept"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .document(spec.clone())
            .app_data(app_data.clone()) // Use the cloned app_data here
            .wrap(cors)
            .wrap(TracingLogger::default()) // Replaced Logger with TracingLogger
            .configure(routes::configure)
            .build_with(
                "/openapi.json",
                BuildConfig::default().with(ScalarConfig::new(&"/docs")),
            )
    })
    .bind(bind_address)
    .map_err(|e| {
        error!("❌ Failed to bind - {}", e);
        APIError::internal(format!("Failed to bind server: {}", e).as_str())
    })?
    .run()
    .await?;

    info!("👋 Server shutting down gracefully");
    Ok(())
}
