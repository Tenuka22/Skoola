use actix_web::{HttpRequest, web};
use apistos::api_operation;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use schemars::JsonSchema;

use tracing::{info, warn};

use crate::models::auth::user::TokenResponse;
use crate::{
    AppState,
    database::enums::RoleEnum,
    database::tables::{User, UserSecurity, UserStatus},
    errors::APIError,
    models::ids::{generate_prefixed_id, IdPrefix},
    schema::{user_security, user_status, users},
    services::{
        auth::auth::create_token_pair,
        auth::oauth::{get_github_user_info, get_google_user_info},
        auth::session::create_session,
    },
};

use actix_web::web::Json;
use apistos::ApiComponent;

#[derive(serde::Deserialize, ApiComponent, JsonSchema)]
pub struct OAuthQuery {
    pub code: String,
}

#[api_operation(
    summary = "Google OAuth2 callback",
    description = "Handles the callback from Google OAuth2.",
    tag = "auth",
    tag = "oauth",
    operation_id = "google_callback"
)]
pub async fn google_callback(
    data: web::Data<AppState>,
    query: web::Query<OAuthQuery>,
    req: HttpRequest,
) -> Result<Json<TokenResponse>, APIError> {
    let ip_address = req.connection_info().realip_remote_addr().map(|s| s.to_string());
    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let user_info = get_google_user_info(&query.code, &data.config).await?;
    let mut conn = data.db_pool.get()?;
    let now = Utc::now().naive_utc();

    let user: User = if let Some(existing) = users::table
        .filter(users::email.eq(&user_info.email))
        .select(User::as_select())
        .first(&mut conn)
        .optional()?
    {
        diesel::update(user_security::table.filter(user_security::user_id.eq(&existing.id)))
            .set((
                user_security::google_id.eq(Some(user_info.id.clone())),
                user_security::updated_at.eq(now),
            ))
            .execute(&mut conn)?;
        existing
    } else {
        let uid = generate_prefixed_id(&mut conn, IdPrefix::USER)?;
        let new_user = User {
            id: uid.clone(),
            email: user_info.email.clone(),
            password_hash: "".to_string(),
            role: RoleEnum::Student,
            created_at: now,
            updated_at: now,
        };
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut conn)?;

        let sec = UserSecurity {
            user_id: uid.clone(),
            google_id: Some(user_info.id.clone()),
            github_id: None,
            verification_token: None,
            verification_sent_at: Some(now),
            password_reset_token: None,
            password_reset_sent_at: None,
            failed_login_attempts: 0,
            lockout_until: None,
            created_at: now,
            updated_at: now,
        };
        diesel::insert_into(user_security::table)
            .values(&sec)
            .execute(&mut conn)?;

        let status = UserStatus {
            user_id: uid.clone(),
            is_verified: true,
            is_active: true,
            disabled_at: None,
            disabled_reason: None,
            created_at: now,
            updated_at: now,
        };
        diesel::insert_into(user_status::table)
            .values(&status)
            .execute(&mut conn)?;
        new_user
    };

    let (token, refresh_token, _exp) = create_token_pair(&user, &data.config, &data.db_pool)?;
    let expires_at = Utc::now()
        .checked_add_signed(Duration::days(data.config.jwt_expiration as i64))
        .ok_or_else(|| APIError::internal("Failed to calculate session expiration"))?
        .naive_utc();

    create_session(
        &mut conn,
        &user.id,
        &refresh_token,
        user_agent.as_deref(),
        ip_address.as_deref(),
        expires_at,
    )
    .map_err(APIError::from)?;

    info!("Google OAuth successful | user_id: {}", user.id);
    Ok(Json(TokenResponse { token, refresh_token }))
}

#[api_operation(
    summary = "GitHub OAuth2 callback",
    description = "Handles the callback from GitHub OAuth2.",
    tag = "auth",
    tag = "oauth",
    operation_id = "github_callback"
)]
pub async fn github_callback(
    data: web::Data<AppState>,
    query: web::Query<OAuthQuery>,
    req: HttpRequest,
) -> Result<Json<TokenResponse>, APIError> {
    let ip_address = req.connection_info().realip_remote_addr().map(|s| s.to_string());
    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let user_info = get_github_user_info(&query.code, &data.config).await?;
    let email = user_info.email.ok_or_else(|| {
        warn!("GitHub OAuth failed: private email");
        APIError::bad_request("GitHub user email is private.")
    })?;

    let mut conn = data.db_pool.get()?;
    let now = Utc::now().naive_utc();
    let user: User = if let Some(existing) = users::table
        .filter(users::email.eq(&email))
        .select(User::as_select())
        .first(&mut conn)
        .optional()?
    {
        diesel::update(user_security::table.filter(user_security::user_id.eq(&existing.id)))
            .set((
                user_security::github_id.eq(Some(user_info.id.to_string())),
                user_security::updated_at.eq(now),
            ))
            .execute(&mut conn)?;
        existing
    } else {
        let uid = generate_prefixed_id(&mut conn, IdPrefix::USER)?;
        let new_user = User {
            id: uid.clone(),
            email: email.clone(),
            password_hash: "".to_string(),
            role: RoleEnum::Student,
            created_at: now,
            updated_at: now,
        };
        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&mut conn)?;

        let sec = UserSecurity {
            user_id: uid.clone(),
            google_id: None,
            github_id: Some(user_info.id.to_string()),
            verification_token: None,
            verification_sent_at: Some(now),
            password_reset_token: None,
            password_reset_sent_at: None,
            failed_login_attempts: 0,
            lockout_until: None,
            created_at: now,
            updated_at: now,
        };
        diesel::insert_into(user_security::table)
            .values(&sec)
            .execute(&mut conn)?;

        let status = UserStatus {
            user_id: uid.clone(),
            is_verified: true,
            is_active: true,
            disabled_at: None,
            disabled_reason: None,
            created_at: now,
            updated_at: now,
        };
        diesel::insert_into(user_status::table)
            .values(&status)
            .execute(&mut conn)?;
        new_user
    };

    let (token, refresh_token, _exp) = create_token_pair(&user, &data.config, &data.db_pool)?;
    let expires_at = Utc::now()
        .checked_add_signed(Duration::days(data.config.jwt_expiration as i64))
        .ok_or_else(|| APIError::internal("Failed to calculate session expiration"))?
        .naive_utc();
    create_session(
        &mut conn,
        &user.id,
        &refresh_token,
        user_agent.as_deref(),
        ip_address.as_deref(),
        expires_at,
    )
    .map_err(APIError::from)?;

    info!("GitHub OAuth successful | user_id: {}", user.id);
    Ok(Json(TokenResponse { token, refresh_token }))
}
