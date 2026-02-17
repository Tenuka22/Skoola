use thiserror::Error;
use crate::errors::APIError;

#[derive(Debug, Error)]
pub enum IAMError {
    #[error("Authentication failed: {reason}")]
    AuthenticationFailed { reason: String },

    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found: {identifier}")]
    UserNotFound { identifier: String },

    #[error("User already exists: {identifier}")]
    UserAlreadyExists { identifier: String },

    #[error("Account is locked until {until}")]
    AccountLocked { until: String },

    #[error("Account not verified: {email}")]
    AccountNotVerified { email: String },

    #[error("Session expired or invalid: {session_id}")]
    SessionExpired { session_id: String },

    #[error("Unauthorized: {reason}")]
    Unauthorized { reason: String },

    #[error("Forbidden: {resource} - {reason}")]
    Forbidden { resource: String, reason: String },

    #[error("Internal IAM error: {message}")]
    Internal { message: String },

    #[error("Google OAuth error: {0}")]
    GoogleOAuthError(String),

    #[error("GitHub OAuth error: {0}")]
    GithubOAuthError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] diesel::result::Error),

    #[error("Database pool error: {0}")]
    PoolError(#[from] r2d2::Error),

    #[error("Password hashing error: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("Token error: {0}")]
    TokenError(#[from] jsonwebtoken::errors::Error),
}

impl From<IAMError> for APIError {
    fn from(err: IAMError) -> Self {
        match err {
            IAMError::InvalidCredentials => APIError::unauthorized("Invalid credentials"),
            IAMError::AuthenticationFailed { reason } => APIError::unauthorized(&reason),
            IAMError::UserNotFound { identifier } => APIError::not_found(&format!("User not found: {}", identifier)),
            IAMError::UserAlreadyExists { identifier } => APIError::conflict(&format!("User already exists: {}", identifier)),
            IAMError::AccountLocked { until } => APIError::forbidden(&format!("Account is locked until {}", until)),
            IAMError::AccountNotVerified { email } => APIError::forbidden(&format!("Account not verified: {}", email)),
            IAMError::SessionExpired { session_id } => APIError::unauthorized(&format!("Session expired or invalid: {}", session_id)),
            IAMError::Unauthorized { reason } => APIError::unauthorized(&reason),
            IAMError::Forbidden { resource, reason } => APIError::forbidden(&format!("{}: {}", resource, reason)),
            IAMError::Internal { message } => APIError::internal(&message),
            IAMError::GoogleOAuthError(msg) => APIError::internal(&format!("Google OAuth failed: {}", msg)),
            IAMError::GithubOAuthError(msg) => APIError::internal(&format!("GitHub OAuth failed: {}", msg)),
            IAMError::DatabaseError(e) => APIError::from(e),
            IAMError::PoolError(e) => APIError::from(e),
            IAMError::BcryptError(_) => APIError::internal("Security operation failed"),
            IAMError::TokenError(e) => APIError::unauthorized(&format!("Invalid token: {}", e)),
        }
    }
}
