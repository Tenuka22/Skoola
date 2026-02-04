use actix_web::{HttpRequest, web};
use apistos::api_operation;
use schemars::JsonSchema;
use uuid::Uuid;
use chrono::{Utc, Duration};
use diesel::prelude::*;

use tracing::{info, warn}; // Added warn for logging errors

use crate::{AppState, database::tables::{Role, RoleEnum, User, UserRole}, errors::APIError,
    services::{auth::{create_token_pair, hash_password}, oauth::{get_github_user_info, get_google_user_info}, session::SessionService},
    schema::{roles, user_roles, users},
};

use apistos::ApiComponent;
use actix_web::web::Json; // Added Json here
use crate::models::auth::TokenResponse; // Added TokenResponse here

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
    req: HttpRequest,
) -> Result<Json<TokenResponse>, APIError> {
    let ip_address = req.connection_info().realip_remote_addr().map(|s| s.to_string());
    let user_agent = req.headers().get("User-Agent").and_then(|v| v.to_str().ok()).map(|s| s.to_string());

    let user_info = get_google_user_info(&query.code, &data.config).await?;

    let mut conn = data.db_pool.get()?;

    let existing_user_result: Option<User> = users::table
        .filter(users::email.eq(&user_info.email))
        .select(User::as_select())
        .first(&mut conn)
        .optional()?;

    let user = match existing_user_result {
        Some(mut user) => {
            user.google_id = Some(user_info.id.clone());
            diesel::update(users::table.find(&user.id))
                .set(users::google_id.eq(&user.google_id))
                .execute(&mut conn)?;
            
            info!("ACTION: Existing user logged in via Google OAuth | user_id: {} | email: {} | google_id: {}", user.id, user.email, user_info.id);
            users::table
                .find(&user.id)
                .select(User::as_select())
                .first(&mut conn)?
        },
        None => {
            let new_user = User {
                id: Uuid::new_v4().to_string(),
                email: user_info.email.clone(),
                password_hash: "".to_string(),
                google_id: Some(user_info.id.clone()),
                github_id: None,
                is_verified: true,
                verification_token: None,
                verification_sent_at: Some(Utc::now().naive_utc()),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                password_reset_token: None,
                password_reset_sent_at: None,
                failed_login_attempts: 0,
                lockout_until: None,
            };
            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&mut conn)?;

            let student_role = roles::table
                .filter(roles::name.eq(RoleEnum::Student.to_string()))
                .select(Role::as_select())
                .first::<Role>(&mut conn)?;

            let new_user_role = UserRole {
                user_id: new_user.id.clone(),
                role_id: student_role.id,
            };

            diesel::insert_into(user_roles::table)
                .values(&new_user_role)
                .execute(&mut conn)?;
            
            info!("ACTION: New user registered via Google OAuth | user_id: {} | email: {} | google_id: {}", new_user.id, new_user.email, user_info.id);
            users::table
                .filter(users::email.eq(&new_user.email))
                .select(User::as_select())
                .first(&mut conn)?
        }
    };
    let (token, refresh_token, _access_token_expiration) = create_token_pair(&user, &data.config, &data.db_pool)?;
    let hashed_refresh_token = hash_password(&refresh_token)?;
    let session_service = SessionService::new(data.db_pool.clone());

    let expires_at = Utc::now()
        .checked_add_signed(Duration::days(data.config.jwt_expiration as i64))
        .ok_or_else(|| APIError::internal("Failed to calculate session expiration"))?
        .naive_utc();

    session_service.create_session(
        user.id.clone(),
        hashed_refresh_token,
        user_agent.clone(),
        ip_address.clone(),
        expires_at,
    ).await?;

    info!(
        "ACTION: Google OAuth successful | user_id: {} | email: {} | ip_address: {:?} | user_agent: {:?}",
        user.id, user.email, ip_address, user_agent
    );
    Ok(Json(TokenResponse {
        token,
        refresh_token,
    }))
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
    req: HttpRequest,
) -> Result<Json<TokenResponse>, APIError> {
    let ip_address = req.connection_info().realip_remote_addr().map(|s| s.to_string());
    let user_agent = req.headers().get("User-Agent").and_then(|v| v.to_str().ok()).map(|s| s.to_string());

    let user_info = get_github_user_info(&query.code, &data.config).await?;

    let email = user_info
        .email
        .ok_or_else(|| {
            warn!("ACTION: GitHub OAuth failed | reason: GitHub user email is private | ip_address: {:?} | user_agent: {:?}", ip_address, user_agent);
            APIError::bad_request("GitHub user email is private.")
        })?;

    let mut conn = data.db_pool.get()?;

    let existing_user_result: Option<User> = users::table
        .filter(users::email.eq(&email))
        .select(User::as_select())
        .first(&mut conn)
        .optional()?;

    let user = match existing_user_result {
        Some(mut user) => {
            user.github_id = Some(user_info.id.to_string());
            diesel::update(users::table.find(&user.id))
                .set(users::github_id.eq(&user.github_id))
                .execute(&mut conn)?;
            
            info!("ACTION: Existing user logged in via GitHub OAuth | user_id: {} | email: {} | github_id: {}", user.id, user.email, user_info.id);
            users::table
                .find(&user.id)
                .select(User::as_select())
                .first(&mut conn)?
        }
        None => {
            let new_user = User {
                id: Uuid::new_v4().to_string(),
                email: email.clone(),
                password_hash: "".to_string(),
                google_id: None,
                github_id: Some(user_info.id.to_string()),
                is_verified: true,
                verification_token: None,
                verification_sent_at: Some(Utc::now().naive_utc()),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                password_reset_token: None,
                password_reset_sent_at: None,
                failed_login_attempts: 0,
                lockout_until: None,
            };
            diesel::insert_into(users::table)
                .values(&new_user)
                .execute(&mut conn)?;

            let student_role = roles::table
                .filter(roles::name.eq(RoleEnum::Student.to_string()))
                .select(Role::as_select())
                .first::<Role>(&mut conn)?;

            let new_user_role = UserRole {
                user_id: new_user.id.clone(),
                role_id: student_role.id,
            };

            diesel::insert_into(user_roles::table)
                .values(&new_user_role)
                .execute(&mut conn)?;
            
            info!("ACTION: New user registered via GitHub OAuth | user_id: {} | email: {} | github_id: {}", new_user.id, new_user.email, user_info.id);
            users::table
                .filter(users::email.eq(&new_user.email))
                .select(User::as_select())
                .first(&mut conn)?
        }
    };

    let (token, refresh_token, _access_token_expiration) = create_token_pair(&user, &data.config, &data.db_pool)?;
    let hashed_refresh_token = hash_password(&refresh_token)?;

    let session_service = SessionService::new(data.db_pool.clone());

    let expires_at = Utc::now()
        .checked_add_signed(Duration::days(data.config.jwt_expiration as i64))
        .ok_or_else(|| APIError::internal("Failed to calculate session expiration"))?
        .naive_utc();

    session_service.create_session(
        user.id.clone(),
        hashed_refresh_token,
        user_agent.clone(),
        ip_address.clone(),
        expires_at,
    ).await?;

    info!(
        "ACTION: GitHub OAuth successful | user_id: {} | email: {} | ip_address: {:?} | user_agent: {:?}",
        user.id, user.email, ip_address, user_agent
    );
    Ok(Json(TokenResponse {
        token,
        refresh_token,
    }))
}
