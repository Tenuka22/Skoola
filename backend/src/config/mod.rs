use crate::errors::APIError;
use std::env;
use crate::database::connection::DbPool;
use crate::services::email::EmailService;


#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub allowed_origin: String,
    pub api_title: String,
    pub api_description: String,
    pub api_version: String,
    pub database_url: String, // Add this line
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_redirect_uri: String,
    pub smtp_host: Option<String>,
    pub smtp_port: u16,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
    pub smtp_sender_email: Option<String>,
    pub email_verification_base_url: String,
    pub password_reset_base_url: String,
    pub send_emails: bool,
    pub test_user_password: Option<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, APIError> {
        let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
        let port = env::var("PORT")
            .unwrap_or_else(|_| "8080".to_string())
            .parse()
            ?;

        let allowed_origin =
            env::var("ALLOWED_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());

        let api_title = env::var("API_TITLE").unwrap_or_else(|_| "Skoola Backend".to_string());
        let api_description = env::var("API_DESCRIPTION")
            .unwrap_or_else(|_| "An API for the Skoola Backend.".to_string());
        let api_version = env::var("API_VERSION").unwrap_or_else(|_| "1.0.0".to_string());

        let database_url =
            env::var("DATABASE_URL").unwrap_or_else(|_| "backend.sqlite".to_string());

        let jwt_secret = env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string());
        let jwt_expiration = env::var("JWT_EXPIRATION")
            .unwrap_or_else(|_| "7".to_string())
            .parse()
            ?;

        let google_client_id = env::var("GOOGLE_CLIENT_ID").unwrap_or_default();
        let google_client_secret = env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default();
        let google_redirect_uri = env::var("GOOGLE_REDIRECT_URI").unwrap_or_default();

        let github_client_id = env::var("GITHUB_CLIENT_ID").unwrap_or_default();
        let github_client_secret = env::var("GITHUB_CLIENT_SECRET").unwrap_or_default();
        let github_redirect_uri = env::var("GITHUB_REDIRECT_URI").unwrap_or_default();

        let smtp_host = env::var("SMTP_HOST").ok();
        let smtp_port = env::var("SMTP_PORT")
            .unwrap_or_else(|_| "587".to_string())
            .parse()
            ?;
        let smtp_username = env::var("SMTP_USERNAME").ok();
        let smtp_password = env::var("SMTP_PASSWORD").ok();
        let smtp_sender_email = env::var("SMTP_SENDER_EMAIL").ok();
        let email_verification_base_url = env::var("EMAIL_VERIFICATION_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:8080/auth/verify-email".to_string());
        let password_reset_base_url = env::var("PASSWORD_RESET_BASE_URL")
            .unwrap_or_else(|_| "http://localhost:8080/auth/reset-password".to_string());

        Ok(Config {
            host,
            port,
            allowed_origin,
            api_title,
            api_description,
            api_version,
            database_url,
            jwt_secret,
            jwt_expiration,
            google_client_id,
            google_client_secret,
            google_redirect_uri,
            github_client_id,
            github_client_secret,
            github_redirect_uri,
            smtp_host: smtp_host.clone(),
            smtp_port,
            smtp_username,
            smtp_password,
            smtp_sender_email: smtp_sender_email.clone(),
            email_verification_base_url,
            password_reset_base_url,
            send_emails: smtp_host.as_deref().is_some_and(|s| !s.is_empty())
                && smtp_sender_email.as_deref().is_some_and(|s| !s.is_empty()),
            test_user_password: env::var("TEST_USER_PASSWORD").ok(),
        })
    }

    pub fn server_url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    pub fn docs_url(&self) -> String {
        format!("{}/docs", self.server_url())
    }

    pub fn openapi_url(&self) -> String {
        format!("{}/openapi.json", self.server_url())
    }
}

#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub db_pool: DbPool,
    pub email_service: EmailService,
}
