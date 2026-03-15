use actix_cors::Cors;
use actix_web::{
    App, HttpServer,
    middleware::{self, TrailingSlash},
    web::{self, Data},
};
use apistos::app::{BuildConfig, OpenApiWrapper};
use apistos::spec::Spec;
use apistos_models::info::Info;
use apistos_scalar::ScalarConfig;
use backend::config::{AppState, Config};
use backend::database::connection::establish_connection;
use backend::errors::APIError;
use backend::routes;
use backend::services::system::cleanup::remove_unverified_users;
use tokio::time::{Duration, interval};
use tracing::info;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::FmtSubscriber;

#[actix_web::main]
async fn main() -> Result<(), APIError> {
    // Initialize tracing subscriber
    FmtSubscriber::builder().init();

    info!("╔════════════════════════════════════════╗");
    info!("║   🚀 Skoola Backend Starting Up       ║");
    info!("╚════════════════════════════════════════╝");

    dotenvy::dotenv().ok();

    let config = Config::from_env()?;

    info!("📍 Server will bind to {}", config.server_url());
    info!("📚 API Documentation: {}", config.docs_url());
    info!("📄 OpenAPI Spec: {}", config.openapi_url());
    info!("🔒 CORS allowed origin: {}", config.allowed_origin);

    let bind_address = (config.host.clone(), config.port);
    let _allowed_origin = config.allowed_origin.clone();

    let pool = establish_connection(&config.database_url)?;

    let app_data = Data::new(AppState {
        config: config.clone(),
        db_pool: pool.clone(),
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
            // .allowed_origin(&allowed_origin)
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE", "OPTIONS"])
            .allowed_headers(vec!["Authorization", "Content-Type", "Accept"])
            .supports_credentials()
            .max_age(3600);

        let spec = Spec {
            info: Info {
                title: config.api_title.clone(),
                description: Some(config.api_description.clone()),
                version: config.api_version.clone(),
                ..Default::default()
            },
            ..Default::default()
        };

        App::new()
            .document(spec)
            .app_data(app_data.clone()) // Use the cloned app_data here
            .app_data(
                web::JsonConfig::default()
                    .error_handler(|err, _req| APIError::bad_request(&err.to_string()).into()),
            )
            .app_data(
                web::QueryConfig::default()
                    .error_handler(|err, _req| APIError::bad_request(&err.to_string()).into()),
            )
            .app_data(
                web::PathConfig::default()
                    .error_handler(|err, _req| APIError::bad_request(&err.to_string()).into()),
            )
            .wrap(cors)
            .wrap(TracingLogger::default()) // Replaced Logger with TracingLogger
            .wrap(middleware::NormalizePath::new(TrailingSlash::MergeOnly))
            .configure(routes::configure)
            .build_with(
                "/openapi.json",
                BuildConfig::default().with(ScalarConfig::new(&"/docs")),
            )
    })
    .bind(bind_address)?
    .run()
    .await?;

    info!("👋 Server shutting down gracefully");
    Ok(())
}
