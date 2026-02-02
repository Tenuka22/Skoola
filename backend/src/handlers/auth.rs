use actix_web::{HttpRequest, web::Json};
use actix_web::{HttpResponse, web};
use apistos::api_operation;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use rand::distributions::{Alphanumeric, DistString};
use tracing::{info, warn};

use crate::{
    AppState,
    database::tables::{Role, User},
    errors::APIError,
    models::auth::{
        LoginRequest, RefreshTokenRequest, RegisterRequest, TokenResponse, UserResponse,
    },
    schema::users,
    services::auth::{create_token_pair, hash_password, refresh_jwt, verify_password},
    services::email::EmailService,
    services::session::SessionService,
};

#[api_operation(
    summary = "Register a new user",
    description = "Creates a new user account.",
    tag = "auth"
)]
pub async fn register(
    data: web::Data<AppState>,
    body: web::Json<RegisterRequest>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;

    let existing_user: Option<User> = users::table
        .filter(users::email.eq(&body.email))
        .select(User::as_select())
        .first(&mut conn)
        .optional()?;

    if existing_user.is_some() {
        warn!(
            "ACTION: User registration failed | reason: email already exists | email: {}",
            body.email
        );
        return Err(APIError::conflict("User with this email already exists"));
    }

    let password_hash = hash_password(&body.password)?;

    let verification_token: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 30);

    let new_user = User {
        id: Uuid::new_v4().to_string(),
        email: body.email.clone(),
        password_hash,
        role: Role::Guest.clone(),
        google_id: None,
        github_id: None,
        is_verified: false,
        verification_token: Some(verification_token.clone()),
        verification_sent_at: Some(Utc::now().naive_utc()),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)?;

    let email_service = EmailService::new(data.config.clone());
    email_service
        .send_verification_email(&new_user.email, &verification_token)
        .await
        .unwrap_or_else(|e| {
            warn!(
                "ACTION: Failed to send verification email to {} | error: {:?}",
                new_user.email, e
            );
            false
        });

    let created_user: User = users::table
        .filter(users::email.eq(&body.email))
        .select(User::as_select())
        .first(&mut conn)?;

    info!(
        "ACTION: User registered successfully | user_id: {} | email: {}",
        created_user.id, created_user.email
    );
    Ok(HttpResponse::Created().json(UserResponse::from(created_user)))
}

#[api_operation(
    summary = "User login",
    description = "Authenticates a user and returns a JWT.",
    tag = "auth"
)]
pub async fn login(
    data: web::Data<AppState>,
    body: web::Json<LoginRequest>,
    req: HttpRequest,
) -> Result<Json<TokenResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

    let user: User = users::table
        .filter(users::email.eq(&body.email))
        .select(User::as_select())
        .first(&mut conn)
        .map_err(|_| {
            warn!(
                "ACTION: User login failed | reason: user not found | email: {}",
                body.email
            );
            APIError::unauthorized("Invalid email or password")
        })?;

    if !user.is_verified {
        warn!(
            "ACTION: User login failed | reason: email not verified | user_id: {} | email: {}",
            user.id, user.email
        );
        return Err(APIError::unauthorized(
            "Email not verified. Please check your inbox for a verification link.",
        ));
    }

    if !verify_password(&body.password, &user.password_hash)? {
        warn!(
            "ACTION: User login failed | reason: invalid password | user_id: {} | email: {}",
            user.id, user.email
        );
        return Err(APIError::unauthorized("Invalid email or password"));
    }

    let (token, refresh_token, _access_token_expiration) = create_token_pair(&user, &data.config)?;

    let hashed_refresh_token = hash_password(&refresh_token)?;

    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|s| s.to_string());

    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    let session_service = SessionService::new(data.db_pool.clone());

    let expires_at = Utc::now()
        .checked_add_signed(Duration::days(data.config.jwt_expiration as i64))
        .ok_or_else(|| APIError::internal("Failed to calculate session expiration"))?
        .naive_utc();

    session_service
        .create_session(
            user.id.clone(),
            hashed_refresh_token,
            user_agent.clone(),
            ip_address.clone(),
            expires_at,
        )
        .await?;

    info!(
        "ACTION: User logged in successfully | user_id: {} | email: {} | ip_address: {:?} | user_agent: {:?}",
        user.id, user.email, ip_address, user_agent
    );
    Ok(Json(TokenResponse {
        token,
        refresh_token,
    }))
}

#[api_operation(
    summary = "Refresh JWT",
    description = "Provides a new JWT by using a refresh token.",
    tag = "auth"
)]
pub async fn refresh(
    data: web::Data<AppState>,
    body: web::Json<RefreshTokenRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, APIError> {
    let ip_address = req
        .connection_info()
        .realip_remote_addr()
        .map(|s: &str| s.to_string());
    let user_agent = req
        .headers()
        .get("User-Agent")
        .and_then(|v: &actix_web::http::header::HeaderValue| v.to_str().ok())
        .map(|s: &str| s.to_string());

    let (new_token, new_refresh_token) = refresh_jwt(
        &body.refresh_token,
        &data.config,
        &data.db_pool,
        ip_address.clone(),
        user_agent.clone(),
    )
    .await
    .map_err(|e| {
        warn!(
            "ACTION: JWT refresh failed | reason: {:?} | ip_address: {:?} | user_agent: {:?}",
            e, ip_address, user_agent
        );
        e
    })?;

    info!(
        "ACTION: JWT refreshed successfully | ip_address: {:?} | user_agent: {:?}",
        ip_address, user_agent
    );
    Ok(HttpResponse::Ok().json(TokenResponse {
        token: new_token,
        refresh_token: new_refresh_token,
    }))
}
