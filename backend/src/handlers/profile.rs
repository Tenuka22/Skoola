use actix_web::{web, HttpResponse};
use apistos::api_operation;

use crate::database::constants::USER_TABLE;
use crate::{
    database::tables::User,
    errors::APIError,
    handlers::oauth::OAuthQuery,
    models::profile::{ChangeEmailRequest, ChangePasswordRequest, UpdateProfileRequest},
    services::{
        auth::{hash_password, verify_password},
        oauth::{get_github_user_info, get_google_user_info},
    },
    utils::jwt::UserId,
    AppState,
};

#[api_operation(
    summary = "Get user profile",
    description = "Retrieves the profile of the currently authenticated user.",
    tag = "profile"
)]
pub async fn get_profile(
    data: web::Data<AppState>,
    user_id: UserId,
) -> Result<HttpResponse, APIError> {
    let user: Option<User> = data.database.select((USER_TABLE, &user_id.0)).await?;

    match user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(APIError::not_found("User not found")),
    }
}

#[api_operation(
    summary = "Update user profile",
    description = "Updates the profile of the currently authenticated user.",
    tag = "profile"
)]
pub async fn update_profile(
    data: web::Data<AppState>,
    user_id: UserId,
    body: web::Json<UpdateProfileRequest>,
) -> Result<HttpResponse, APIError> {
    let updated_user: Option<User> = data
        .database
        .update((USER_TABLE, &user_id.0))
        .merge(serde_json::json!({ "email": body.email }))
        .await?;

    match updated_user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(APIError::not_found("User not found")),
    }
}

#[api_operation(
    summary = "Change password",
    description = "Changes the password of the currently authenticated user.",
    tag = "profile"
)]
pub async fn change_password(
    data: web::Data<AppState>,
    user_id: UserId,
    body: web::Json<ChangePasswordRequest>,
) -> Result<HttpResponse, APIError> {
    let user: Option<User> = data.database.select((USER_TABLE, &user_id.0)).await?;

    let user = user.ok_or_else(|| APIError::not_found("User not found"))?;

    if !verify_password(&body.old_password, &user.password_hash)? {
        return Err(APIError::unauthorized("Invalid old password"));
    }

    let new_password_hash = hash_password(&body.new_password)?;

    let updated_user: Option<User> = data
        .database
        .update((USER_TABLE, &user_id.0))
        .merge(serde_json::json!({ "password_hash": new_password_hash }))
        .await?;

    match updated_user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(APIError::not_found("User not found")),
    }
}

#[api_operation(
    summary = "Change email",
    description = "Changes the email of the currently authenticated user.",
    tag = "profile"
)]
pub async fn change_email(
    data: web::Data<AppState>,
    user_id: UserId,
    body: web::Json<ChangeEmailRequest>,
) -> Result<HttpResponse, APIError> {
    let user: Option<User> = data.database.select((USER_TABLE, &user_id.0)).await?;

    let user = user.ok_or_else(|| APIError::not_found("User not found"))?;

    if !verify_password(&body.password, &user.password_hash)? {
        return Err(APIError::unauthorized("Invalid password"));
    }

    let updated_user: Option<User> = data
        .database
        .update((USER_TABLE, &user_id.0))
        .merge(serde_json::json!({ "email": body.new_email }))
        .await?;

    match updated_user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(APIError::not_found("User not found")),
    }
}

#[api_operation(
    summary = "Link Google account",
    description = "Links a Google account to the currently authenticated user.",
    tag = "profile", tag = "oauth"
)]
pub async fn link_google(
    data: web::Data<AppState>,
    user_id: UserId,
    query: web::Query<OAuthQuery>,
) -> Result<HttpResponse, APIError> {
    let google_user_info = get_google_user_info(&query.code, &data.config).await?;

    let updated_user: Option<User> = data
        .database
        .update((USER_TABLE, &user_id.0))
        .merge(serde_json::json!({ "google_id": google_user_info.id }))
        .await?;

    match updated_user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(APIError::not_found("User not found")),
    }
}

#[api_operation(
    summary = "Link GitHub account",
    description = "Links a GitHub account to the currently authenticated user.",
    tag = "profile", tag = "oauth"
)]
pub async fn link_github(
    data: web::Data<AppState>,
    user_id: UserId,
    query: web::Query<OAuthQuery>,
) -> Result<HttpResponse, APIError> {
    let github_user_info = get_github_user_info(&query.code, &data.config).await?;

    let updated_user: Option<User> = data
        .database
        .update((USER_TABLE, &user_id.0))
        .merge(serde_json::json!({ "github_id": github_user_info.id.to_string() }))
        .await?;

    match updated_user {
        Some(user) => Ok(HttpResponse::Ok().json(user)),
        None => Err(APIError::not_found("User not found")),
    }
}