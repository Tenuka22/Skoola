use crate::errors::APIError;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand::RngCore;
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{
    config::Config,
    database::{connection::DbPool, tables::User},
    schema::{users},
    services::session::SessionService,
};

pub use crate::utils::security::{hash_password, verify_password};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<String>,
    pub exp: i64,
}

pub fn create_token_pair(user: &User, config: &Config, _db_pool: &DbPool) -> Result<(String, String, i64), APIError> {
    let roles = vec![user.role.to_string()];

    let expiration = Utc::now()
        .checked_add_signed(Duration::days(config.jwt_expiration as i64))
        .ok_or_else(|| APIError::internal("Failed to calculate JWT expiration timestamp"))?
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
    )
    ?;

    let refresh_token = generate_refresh_token();
    info!(
        "Access token and refresh token created for user_id: {}",
        user.id
    );
    Ok((access_token, refresh_token, expiration))
}

#[allow(deprecated)]
pub fn generate_refresh_token() -> String {
    let mut rng = rand::thread_rng();
    let mut token_bytes = vec![0u8; 32];
    rng.fill_bytes(&mut token_bytes);
    base64_url::encode(&token_bytes)
}

pub async fn refresh_jwt(
    refresh_token: &str,
    config: &Config,
    db_pool: &DbPool,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<(String, String), APIError> {
    let session_service = SessionService::new(db_pool.clone());

    let hashed_refresh_token = hash_password(refresh_token)?;
    info!("Attempting to refresh JWT for a session.");

    let session = session_service
        .find_session_by_refresh_token_hash(&hashed_refresh_token)
        .await?
        .ok_or_else(|| APIError::unauthorized("Invalid refresh token or session expired"))?;

    info!(
        "Found session {} for user_id: {}",
        session.id, session.user_id
    );

    // Delete the old session
    session_service.delete_session(&session.id).await?;
    info!(
        "Old session {} for user_id {} deleted.",
        session.id, session.user_id
    );

    let user_result: Option<User> = users::table
        .find(&session.user_id)
        .select(User::as_select())
        .first(&mut session_service.db_pool.get()?)
        .optional()
        ?;

    let user = user_result.ok_or_else(|| APIError::internal("User not found for session"))?;
    info!("User {} found for session refresh.", user.id);

    let (access_token, new_refresh_token, _access_token_expiration) =
        create_token_pair(&user, config, db_pool)?;
    let hashed_new_refresh_token = hash_password(&new_refresh_token)?;

    let expires_at = Utc::now()
        .checked_add_signed(Duration::days(config.jwt_expiration as i64))
        .ok_or_else(|| APIError::internal("Failed to calculate new session expiration timestamp"))?
        .naive_utc();

    let created_session = session_service
        .create_session(
            user.id.clone(),
            hashed_new_refresh_token,
            user_agent,
            ip_address,
            expires_at,
        )
        .await?;
    info!(
        "New session {} created for user_id: {}. Refresh successful.",
        created_session.id, user.id
    );

    Ok((access_token, new_refresh_token))
}

pub fn decode_jwt(token: &str, config: &Config) -> Result<Claims, APIError> {
    Ok(decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS512),
    )
    .map_err(|e| APIError::from(e))?
    .claims)
}