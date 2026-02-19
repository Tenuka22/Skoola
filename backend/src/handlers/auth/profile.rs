use crate::models::auth::user::{UserResponse, UserProfileResponse};
use crate::schema::users::dsl::*;
use crate::{
    AppState,
    database::tables::{User},
    errors::APIError,
    handlers::auth::oauth::OAuthQuery,
    models::auth::profile::{ChangeEmailRequest, ChangePasswordRequest, UpdateProfileRequest},
    schema::{users},
    services::{
        auth::auth::{hash_password, verify_password},
        auth::oauth::{get_github_user_info, get_google_user_info},
        auth::session::invalidate_sessions_for_user,
        auth::user_service
    },
    utils::jwt::UserId,
};
use actix_web::web::{self, Json};
use apistos::api_operation;
use diesel::prelude::*;
use tracing::{info, warn};
use actix_web::HttpMessage; 

#[api_operation(
    summary = "Get user profile",
    description = "Retrieves the profile of the currently authenticated user.",
    tag = "profile",
    operation_id = "get_profile"
)]
pub async fn get_profile(
    data: web::Data<AppState>,
    req: actix_web::HttpRequest,
) -> Result<Json<UserProfileResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    
    let user_id = req.extensions()
        .get::<UserId>()
        .cloned()  
        .ok_or_else(|| {
            warn!("ACTION: Failed to extract UserId from request extensions.");
            APIError::unauthorized("Unauthorized")
        })?;
    
    let user: User = users::table
        .find(&user_id.0)
        .select(User::as_select())
        .first(&mut conn)
        .optional()?
        .ok_or_else(|| {
            warn!("ACTION: User profile fetch failed | reason: user not found | user_id: {}", user_id.0);
            APIError::not_found("User not found")
        })?;

    let user_profile_response = user_service::user_to_user_profile_response(user)?;

    info!("ACTION: User profile fetched | user_id: {}", user_id.0);
    Ok(Json(user_profile_response))
}

#[api_operation(
    summary = "Update user profile",
    description = "Updates the profile of the currently authenticated user.",
    tag = "profile",
    operation_id = "update_profile"
)]
pub async fn update_profile(
    data: web::Data<AppState>,
    user_id: UserId,
    body: web::Json<UpdateProfileRequest>,
) -> Result<Json<UserResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

    let mut updated_fields = Vec::new();

    if let Some(new_email) = &body.email {
        diesel::update(users::table.find(&user_id.0))
            .set(email.eq(new_email))
            .execute(&mut conn)?;
        updated_fields.push(format!("email: {}", new_email));
    }

    let updated_user: User = users::table
        .find(&user_id.0)
        .select(User::as_select())
        .first(&mut conn)
        .optional()?
        .ok_or_else(|| {
            warn!(
                "ACTION: User profile update failed | reason: user not found after update | user_id: {}",
                user_id.0
            );
            APIError::not_found("User not found")
        })?;

    let changes_summary = if updated_fields.is_empty() {
        "no changes".to_string()
    } else {
        updated_fields.join(", ")
    };
    info!(
        "ACTION: User profile updated | user_id: {} | changes: {}",
        user_id.0, changes_summary
    );
    Ok(Json(UserResponse::from(updated_user)))
}

#[api_operation(
    summary = "Change password",
    description = "Changes the password of the currently authenticated user.",
    tag = "profile",
    operation_id = "change_password"
)]
pub async fn change_password(
    data: web::Data<AppState>,
    user_id: UserId,
    body: web::Json<ChangePasswordRequest>,
) -> Result<Json<UserResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

    let user: User = users::table
        .find(&user_id.0)
        .select(User::as_select())
        .first(&mut conn)
        .optional()?
        .ok_or_else(|| {
            warn!(
                "ACTION: Change password failed | reason: user not found | user_id: {}",
                user_id.0
            );
            APIError::not_found("User not found")
        })?;

    if !verify_password(&body.old_password, &user.password_hash)? {
        warn!(
            "ACTION: Change password failed | reason: invalid old password | user_id: {}",
            user_id.0
        );
        return Err(APIError::unauthorized("Invalid old password"));
    }

    let new_password_hash = hash_password(&body.new_password)?;

    diesel::update(users::table.find(&user_id.0))
        .set(password_hash.eq(new_password_hash))
        .execute(&mut conn)?;

    // Invalidate all sessions for this user after password change
    invalidate_sessions_for_user(&mut conn, &user_id.0).map_err(APIError::from)?;

    let updated_user: User = users::table
        .find(&user_id.0)
        .select(User::as_select())
        .first(&mut conn)
        .optional()?
        .ok_or_else(|| {
            warn!(
                "ACTION: Change password failed | reason: user not found after update | user_id: {}",
                user_id.0
            );
            APIError::not_found("User not found")
        })?;

    info!(
        "ACTION: Password changed and all sessions invalidated | user_id: {}",
        user_id.0
    );
    Ok(Json(UserResponse::from(updated_user)))
}

#[api_operation(
    summary = "Change email",
    description = "Changes the email of the currently authenticated user.",
    tag = "profile",
    operation_id = "change_email"
)]
pub async fn change_email(
    data: web::Data<AppState>,
    user_id: UserId,
    body: web::Json<ChangeEmailRequest>,
) -> Result<Json<UserResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

    let user: User = users::table
        .find(&user_id.0)
        .select(User::as_select())
        .first(&mut conn)
        .optional()?
        .ok_or_else(|| {
            warn!(
                "ACTION: Change email failed | reason: user not found | user_id: {}",
                user_id.0
            );
            APIError::not_found("User not found")
        })?;

    if !verify_password(&body.password, &user.password_hash)? {
        warn!(
            "ACTION: Change email failed | reason: invalid password | user_id: {}",
            user_id.0
        );
        return Err(APIError::unauthorized("Invalid password"));
    }

    diesel::update(users::table.find(&user_id.0))
        .set(email.eq(&body.new_email))
        .execute(&mut conn)?;

    // Invalidate all sessions for this user after email change
    invalidate_sessions_for_user(&mut conn, &user_id.0).map_err(APIError::from)?;

    let updated_user: User = users::table
        .find(&user_id.0)
        .select(User::as_select())
        .first(&mut conn)
        .optional()?
        .ok_or_else(|| {
            warn!(
                "ACTION: Change email failed | reason: user not found after update | user_id: {}",
                user_id.0
            );
            APIError::not_found("User not found")
        })?;

    info!(
        "ACTION: Email changed to {} and all sessions invalidated | user_id: {}",
        updated_user.email, user_id.0
    );
    Ok(Json(UserResponse::from(updated_user)))
}

#[api_operation(
    summary = "Link Google account",
    description = "Links a Google account to the currently authenticated user.",
    tag = "profile",
    tag = "oauth",
    operation_id = "link_google"
)]
pub async fn link_google(
    data: web::Data<AppState>,
    user_id: UserId,
    query: web::Query<OAuthQuery>,
) -> Result<Json<UserResponse>, APIError> {
    let google_user_info = get_google_user_info(&query.code, &data.config)
        .await?;

    let mut conn = data.db_pool.get()?;

    diesel::update(users::table.find(&user_id.0))
        .set(google_id.eq(google_user_info.id.clone()))
        .execute(&mut conn)?;

    let updated_user: User = users::table
        .find(&user_id.0)
        .select(User::as_select())
        .first(&mut conn)
        .optional()?
        .ok_or_else(|| {
            warn!(
                "ACTION: Link Google failed | reason: user not found after update | user_id: {}",
                user_id.0
            );
            APIError::not_found("User not found")
        })?;

    info!(
        "ACTION: Google account linked | user_id: {} | google_id: {}",
        user_id.0, google_user_info.id
    );
    Ok(Json(UserResponse::from(updated_user)))
}

#[api_operation(
    summary = "Link GitHub account",
    description = "Links a GitHub account to the currently authenticated user.",
    tag = "profile",
    tag = "oauth",
    operation_id = "link_github"
)]
pub async fn link_github(
    data: web::Data<AppState>,
    user_id: UserId,
    query: web::Query<OAuthQuery>,
) -> Result<Json<UserResponse>, APIError> {
    let github_user_info = get_github_user_info(&query.code, &data.config)
        .await?;

    let mut conn = data.db_pool.get()?;

    diesel::update(users::table.find(&user_id.0))
        .set(github_id.eq(github_user_info.id.to_string()))
        .execute(&mut conn)?;

    let updated_user: User = users::table
        .find(&user_id.0)
        .select(User::as_select())
        .first(&mut conn)
        .optional()?
        .ok_or_else(|| {
            warn!(
                "ACTION: Link GitHub failed | reason: user not found after update | user_id: {}",
                user_id.0
            );
            APIError::not_found("User not found")
        })?;

    info!(
        "ACTION: GitHub account linked | user_id: {} | github_id: {}",
        user_id.0, github_user_info.id
    );
    Ok(Json(UserResponse::from(updated_user)))
}