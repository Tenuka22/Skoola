use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub allowed_origin: String,
    pub api_title: String,
    pub api_description: String,
    pub api_version: String,
    pub db_passwrod: String,
    pub db_username: String,
    pub db_url: String,
    pub db_nameserver: String,
    pub db_database: String,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub google_client_id: String,
    pub google_client_secret: String,
    pub google_redirect_uri: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub github_redirect_uri: String,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        Ok(Config {
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()
                .map_err(|_| "PORT must be a valid number")?,
            allowed_origin: env::var("ALLOWED_ORIGIN")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            api_title: env::var("API_TITLE").unwrap_or_else(|_| "Skoola Backend".to_string()),
            api_description: env::var("API_DESCRIPTION")
                .unwrap_or_else(|_| "An API for the Skoola Backend.".to_string()),
            api_version: env::var("API_VERSION").unwrap_or_else(|_| "1.0.0".to_string()),
            db_passwrod: env::var("DB_PASSWORD").unwrap_or_else(|_| "secret".to_string()),
            db_url: env::var("DB_URL").unwrap_or_else(|_| "127.0.0.1:8000".to_string()),
            db_username: env::var("DB_USERNAME").unwrap_or_else(|_| "root".to_string()),
            db_nameserver: env::var("DB_NAMESERVER").unwrap_or_else(|_| "main".to_string()),
            db_database: env::var("DB_DATABASE").unwrap_or_else(|_| "main".to_string()),
            jwt_secret: env::var("JWT_SECRET").unwrap_or_else(|_| "your-secret-key".to_string()),
            jwt_expiration: env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "7".to_string())
                .parse()
                .map_err(|_| "JWT_EXPIRATION must be a valid number")?,
            google_client_id: env::var("GOOGLE_CLIENT_ID").unwrap_or_default(),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default(),
            google_redirect_uri: env::var("GOOGLE_REDIRECT_URI").unwrap_or_default(),
            github_client_id: env::var("GITHUB_CLIENT_ID").unwrap_or_default(),
            github_client_secret: env::var("GITHUB_CLIENT_SECRET").unwrap_or_default(),
            github_redirect_uri: env::var("GITHUB_REDIRECT_URI").unwrap_or_default(),
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
