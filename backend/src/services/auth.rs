use anyhow::Result;
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand::{RngCore, rng};
use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::remote::ws::Client};
use uuid::Uuid;

use crate::database::constants::{SESSION_TABLE, USER_TABLE};
use crate::{
    config::Config,
    database::tables::{Session, User},
    errors::APIError,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub fn hash_password(password: &str) -> Result<String, APIError> {
    hash(password, DEFAULT_COST).map_err(|_| APIError::internal("Internal Server Error"))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, APIError> {
    verify(password, hash).map_err(|_| APIError::internal("Internal Server Error"))
}

pub fn create_token_pair(user: &User, config: &Config) -> Result<(String, String), APIError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(config.jwt_expiration))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user.id.to_string(),
        role: user.role.to_string(),
        exp: expiration as usize,
    };

    let header = Header::new(jsonwebtoken::Algorithm::HS512);
    let access_token = encode(
        &header,
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )
    .map_err(|_| APIError::internal("Internal Server Error"))?;

    let refresh_token = generate_refresh_token();
    Ok((access_token, refresh_token))
}

#[allow(deprecated)]
pub fn generate_refresh_token() -> String {
    let mut rng = rng();
    let mut token_bytes = vec![0u8; 32];
    rng.fill_bytes(&mut token_bytes);
    base64_url::encode(&token_bytes)
}

pub async fn refresh_jwt(
    refresh_token: &str,
    config: &Config,
    db: &Surreal<Client>,
    ip_address: Option<String>,
    user_agent: Option<String>,
) -> Result<(String, String), APIError> {
    let hashed_refresh_token = hash_password(refresh_token)?;

    let mut response = db
        .query("SELECT * FROM type::table($table) WHERE refresh_token_hash = $token")
        .bind(("table", SESSION_TABLE))
        .bind(("token", hashed_refresh_token.clone()))
        .await
        .map_err(|_| APIError::internal("Internal Server Error"))?;

    let sessions_option: Option<Vec<Session>> = response
        .take(0)
        .map_err(|_| APIError::internal("Internal Server Error"))?;

    let mut sessions: Vec<Session> = sessions_option.unwrap_or_default();

    let session = sessions
        .pop()
        .ok_or_else(|| APIError::unauthorized("Invalid refresh token or session expired"))?;

    if session.expires_at < Utc::now() {
        return Err(APIError::unauthorized("Refresh token expired"));
    }

    if !verify_password(refresh_token, &session.refresh_token_hash)? {
        return Err(APIError::unauthorized("Invalid refresh token"));
    }

    // Invalidate the old session
    let _: Option<Session> = db
        .update((SESSION_TABLE, session.id.to_string()))
        .merge(serde_json::json!({ "expires_at": Utc::now() }))
        .await
        .map_err(|_| APIError::internal("Failed to invalidate old session"))?;

    let user: Option<User> = db.select((USER_TABLE, session.user_id.to_string())).await?;
    let user = user.ok_or_else(|| APIError::internal("User not found for session"))?;

    let (access_token, new_refresh_token) = create_token_pair(&user, config)?;
    let hashed_new_refresh_token = hash_password(&new_refresh_token)?;

    let new_session = Session {
        id: Uuid::new_v4(),
        user_id: user.id,
        refresh_token_hash: hashed_new_refresh_token,
        user_agent,
        ip_address,
        created_at: Utc::now(),
        expires_at: Utc::now()
            .checked_add_signed(Duration::days(config.jwt_expiration))
            .expect("valid timestamp"),
    };

    let _: Option<Session> = db
        .create(SESSION_TABLE)
        .content(new_session)
        .await
        .map_err(|_| APIError::internal("Failed to create new session"))?;

    Ok((access_token, new_refresh_token))
}

pub fn decode_jwt(token: &str, config: &Config) -> Result<Claims, APIError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.jwt_secret.as_ref()),
        &Validation::new(jsonwebtoken::Algorithm::HS512),
    )
    .map(|data| data.claims)
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
            APIError::unauthorized("Token has expired")
        }
        _ => APIError::unauthorized("Invalid token"),
    })
}
