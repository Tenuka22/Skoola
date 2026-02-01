use anyhow::Result;
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rand::{RngCore, thread_rng};
use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::remote::ws::Client};

use crate::database::constants::USER_TABLE;
use crate::{config::Config, database::tables::User, errors::APIError};

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
    let mut rng = thread_rng();
    let mut token_bytes = vec![0u8; 32];
    rng.fill_bytes(&mut token_bytes);
    base64_url::encode(&token_bytes)
}

pub async fn refresh_jwt(
    refresh_token: &str,
    config: &Config,
    db: &Surreal<Client>,
) -> Result<(String, String), APIError> {
    let hashed_refresh_token = hash_password(refresh_token)?;

    let mut response = db
        .query("SELECT * FROM type::table($table) WHERE refresh_token = $token")
        .bind(("table", USER_TABLE))
        .bind(("token", hashed_refresh_token.clone()))
        .await
        .map_err(|_| APIError::internal("Internal Server Error"))?;

    let users_option: Option<Vec<User>> = response
        .take(0)
        .map_err(|_| APIError::internal("Internal Server Error"))?;

    let mut users: Vec<User> = users_option.unwrap_or_default();

    let user = users
        .pop()
        .ok_or_else(|| APIError::unauthorized("Invalid refresh token"))?;

    if !verify_password(refresh_token, &user.refresh_token.clone().unwrap_or_default())? {
        return Err(APIError::unauthorized("Invalid refresh token"));
    }

    create_token_pair(&user, config)
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
