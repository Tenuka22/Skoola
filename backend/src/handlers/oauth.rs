use actix_web::{HttpResponse, web};
use apistos::api_operation;
use schemars::JsonSchema;
use uuid::Uuid;

use crate::database::constants::USER_TABLE;
use crate::{
    AppState,
    database::tables::{Role, User},
    errors::APIError,
    services::{
        auth::{create_token_pair, hash_password},
        oauth::{get_github_user_info, get_google_user_info},
    },
};

use apistos::ApiComponent;

#[derive(serde::Deserialize, ApiComponent, JsonSchema)]
pub struct OAuthQuery {
    pub code: String,
}

#[api_operation(
    summary = "Google OAuth2 callback",
    description = "Handles the callback from Google OAuth2.",
    tag = "auth",
    tag = "oauth"
)]
pub async fn google_callback(
    data: web::Data<AppState>,
    query: web::Query<OAuthQuery>,
) -> Result<HttpResponse, APIError> {
    let user_info = get_google_user_info(&query.code, &data.config).await?;

    let mut response = data
        .database
        .query("SELECT * FROM type::table($table) WHERE email = $email")
        .bind(("table", USER_TABLE))
        .bind(("email", user_info.email.clone()))
        .await?;
    let users_option: Option<Vec<User>> = response.take(0)?;
    let mut users: Vec<User> = users_option.unwrap_or_default();

    let user = match users.pop() {
        Some(mut user) => {
            user.google_id = Some(user_info.id);
            let updated_user_option: Option<User> = data
                .database
                .update((USER_TABLE, user.id.to_string()))
                .merge(serde_json::json!({ "google_id": user.google_id }))
                .await?;
            let updated_user = updated_user_option
                .ok_or_else(|| APIError::internal("Failed to update user"))?;
            updated_user
        }
        None => {
            let new_user = User {
                id: Uuid::new_v4(),
                email: user_info.email,
                password_hash: "".to_string(), // No password for OAuth users
                role: Role::Student,
                google_id: Some(user_info.id),
                github_id: None,
                refresh_token: None,
                created_at: chrono::Utc::now().into(),
                updated_at: chrono::Utc::now().into(),
            };
            let created_user: User = data
                .database
                .create(USER_TABLE)
                .content(new_user)
                .await?
                .ok_or_else(|| APIError::internal("Failed to create user"))?;
            created_user
        }
    };

    let (token, refresh_token) = create_token_pair(&user, &data.config)?;
    let hashed_refresh_token = hash_password(&refresh_token)?;

    let updated_refresh_token_user_option: Option<User> = data
        .database
        .update((USER_TABLE, user.id.to_string()))
        .merge(serde_json::json!({
            "refresh_token": hashed_refresh_token,
        }))
        .await?;
    let _: User = updated_refresh_token_user_option
        .ok_or_else(|| APIError::internal("Failed to update user"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": token,
        "refresh_token": refresh_token,
    })))
}

#[api_operation(
    summary = "GitHub OAuth2 callback",
    description = "Handles the callback from GitHub OAuth2.",
    tag = "auth",
    tag = "oauth"
)]
pub async fn github_callback(
    data: web::Data<AppState>,
    query: web::Query<OAuthQuery>,
) -> Result<HttpResponse, APIError> {
    let user_info = get_github_user_info(&query.code, &data.config).await?;

    let email = user_info
        .email
        .ok_or_else(|| APIError::bad_request("GitHub user email is private."))?;

    let mut response = data
        .database
        .query("SELECT * FROM type::table($table) WHERE email = $email")
        .bind(("table", USER_TABLE))
        .bind(("email", email.clone()))
        .await?;
    let users_option: Option<Vec<User>> = response.take(0)?;
    let mut users: Vec<User> = users_option.unwrap_or_default();

    let user = match users.pop() {
        Some(mut user) => {
            user.github_id = Some(user_info.id.to_string());
            let updated_user_option: Option<User> = data
                .database
                .update((USER_TABLE, user.id.to_string()))
                .merge(serde_json::json!({ "github_id": user.github_id }))
                .await?;
            let updated_user = updated_user_option
                .ok_or_else(|| APIError::internal("Failed to update user"))?;
            updated_user
        }
        None => {
            let new_user = User {
                id: Uuid::new_v4(),
                email,
                password_hash: "".to_string(), // No password for OAuth users
                role: Role::Student,
                google_id: None,
                github_id: Some(user_info.id.to_string()),
                refresh_token: None,
                created_at: chrono::Utc::now().into(),
                updated_at: chrono::Utc::now().into(),
            };
            let created_user: User = data
                .database
                .create(USER_TABLE)
                .content(new_user)
                .await?
                .ok_or_else(|| APIError::internal("Failed to create user"))?;
            created_user
        }
    };

    let (token, refresh_token) = create_token_pair(&user, &data.config)?;
    let hashed_refresh_token = hash_password(&refresh_token)?;

    let updated_refresh_token_user_option: Option<User> = data
        .database
        .update((USER_TABLE, user.id.to_string()))
        .merge(serde_json::json!({
            "refresh_token": hashed_refresh_token,
        }))
        .await?;
    let _: User = updated_refresh_token_user_option
        .ok_or_else(|| APIError::internal("Failed to update user"))?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": token,
        "refresh_token": refresh_token,
    })))
}
