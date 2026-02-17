use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand::RngCore;
use serde::{Deserialize, Serialize};

use crate::{
    config::Config,
    database::{connection::DbPool, tables::User},
    errors::iam::IAMError,
    errors::APIError,
    services::auth::session,
    utils::logging::{log_auth_success, log_auth_failure, log_iam_error},
    services::auth::user_service,
};

pub use crate::utils::security::{hash_password, verify_password};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<String>,
    pub exp: i64,
}

/// Creates a new access and refresh token pair for a user.
pub fn create_token_pair(
    user: &User,
    config: &Config,
    _db_pool: &DbPool, // Kept for signature compatibility if needed, but unused in logic
) -> Result<(String, String, i64), IAMError> {
    let roles = vec![user.role.to_string()];

    let expiration = Utc::now()
        .checked_add_signed(Duration::days(config.jwt_expiration as i64))
        .ok_or_else(|| IAMError::Internal { message: "Failed to calculate JWT expiration timestamp".to_string() })?
        .timestamp();

    let claims = Claims {
        sub: user.id.to_string(),
        roles,
        exp: expiration,
    };

    let header = Header::new(jsonwebtoken::Algorithm::HS512);
    let access_token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )?;

    let refresh_token = generate_refresh_token();
    
    log_auth_success(&user.id, "token_creation");
    Ok((access_token, refresh_token, expiration))
}

#[allow(deprecated)]
pub fn generate_refresh_token() -> String {
    let mut rng = rand::thread_rng();
    let mut token_bytes = vec![0u8; 32];
    rng.fill_bytes(&mut token_bytes);
    base64_url::encode(&token_bytes)
}

/// Refreshes a JWT using a valid refresh token.
pub async fn refresh_jwt(
    refresh_token: &str,
    config: &Config,
    db_pool: &DbPool,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<(String, String), APIError> {
    // This function keeps the async signature for handler compatibility but uses synchronous logic internally where possible
    // or wraps async DB calls if we were using an async DB driver (which we aren't really with Diesel+Sqlite here directly)
    // However, since Diesel is blocking, we should ideally run this in web::block.
    // For now, we'll keep the logic here but use the pool to get a connection.

    let mut conn = db_pool.get().map_err(|e| {
        log_iam_error("db_pool_connection", &e);
        APIError::from(e)
    })?;

    let hashed_refresh_token = hash_password(refresh_token).map_err(|e| {
        log_iam_error("password_hash", &e);
        APIError::from(e) // hash_password returns APIError currently
    })?;

    let session = session::find_session_by_refresh_token_hash(&mut conn, &hashed_refresh_token)
        .map_err(APIError::from)?
        .ok_or_else(|| {
            log_auth_failure("unknown_session", "Invalid refresh token or session expired");
            APIError::unauthorized("Invalid refresh token or session expired")
        })?;

    // Delete the old session
    session::delete_session(&mut conn, &session.id).map_err(APIError::from)?;

    let user = user_service::get_user_by_id(&mut conn, &session.user_id)
        .map_err(|e| {
             log_auth_failure(&session.user_id, "User not found during refresh");
             APIError::from(e)
        })?;

    let (access_token, new_refresh_token, _access_token_expiration) =
        create_token_pair(&user, config, db_pool).map_err(APIError::from)?;
    
    let hashed_new_refresh_token = hash_password(&new_refresh_token).map_err(APIError::from)?;

    let expires_at = Utc::now()
        .checked_add_signed(Duration::days(config.jwt_expiration as i64))
        .ok_or_else(|| APIError::internal("Failed to calculate new session expiration timestamp"))?
        .naive_utc();

    session::create_session(
        &mut conn,
        &user.id,
        &hashed_new_refresh_token,
        user_agent.as_deref(),
        ip_address.as_deref(),
        expires_at,
    ).map_err(APIError::from)?;

    Ok((access_token, new_refresh_token))
}

pub fn decode_jwt(token: &str, config: &Config) -> Result<Claims, IAMError> {
    Ok(decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS512),
    )?
    .claims)
}
