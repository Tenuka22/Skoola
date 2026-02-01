use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware::Logger, web::Data};
use apistos::{
    app::{BuildConfig, OpenApiWrapper},
    spec::Spec,
};
use apistos_models::info::Info;
use apistos_scalar::ScalarConfig;
use config::Config;
use env_logger::Env;
use log::{error, info};
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::database::init_db;

mod config;
mod database;
mod errors;
mod handlers;
mod models;
mod routes;
mod services;
mod utils;

struct AppState {
    config: Config,
    database: Surreal<Client>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    env_logger::Builder::from_env(Env::default().default_filter_or("info"))
        .format_timestamp_secs()
        .init();

    info!("╔════════════════════════════════════════╗");
    info!("║   🚀 Skoola Backend Starting Up       ║");
    info!("╚════════════════════════════════════════╝");

    let config = Config::from_env().map_err(|e| {
        error!("❌ Configuration error: {}", e);
        std::io::Error::new(std::io::ErrorKind::InvalidInput, e)
    })?;

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

    let database = init_db(&config).await?;

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&allowed_origin)
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Authorization", "Content-Type", "Accept"])
            .supports_credentials()
            .max_age(3600);

        App::new()
            .document(spec.clone())
            .app_data(Data::new(AppState {
                config: config.clone(),
                database: database.clone(),
            }))
            .wrap(cors)
            .wrap(Logger::new(
                r#"%a "%r" %s %b "%{Referer}i" "%{User-Agent}i" %T"#,
            ))
            .configure(routes::configure)
            .build_with(
                "/openapi.json",
                BuildConfig::default().with(ScalarConfig::new(&"/docs")),
            )
    })
    .bind(bind_address)
    .map_err(|e| {
        error!("❌ Failed to bind - {}", e);
        e
    })?
    .run()
    .await?;

    info!("👋 Server shutting down gracefully");
    Ok(())
}
