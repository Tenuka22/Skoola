use actix_web::{HttpRequest, web::Json};
use actix_web::web;
use apistos::api_operation;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use uuid::Uuid;

use rand::distributions::{Alphanumeric, DistString};
use tracing::{info, warn};

use crate::{
    AppState,
    database::tables::{User},
    database::enums::RoleEnum,
    errors::APIError,
    models::auth::user::{LoginRequest, PasswordReset, PasswordResetRequest, RefreshTokenRequest, RegisterRequest, TokenResponse, UserResponse},
    models::{MessageResponse, Profile, NewProfile, UserProfile, NewUserProfile},
    schema::{users, profiles, user_profiles},
    services::auth::auth::{create_token_pair, hash_password, refresh_jwt, verify_password},
    services::system::email::{send_verification_email, send_password_reset_email},
    services::auth::session::{create_session, delete_session, find_session_by_refresh_token_hash, invalidate_sessions_for_user},

};

#[api_operation(
    summary = "Register a new user",
    description = "Creates a new user account.",
    tag = "auth",
    operation_id = "register_user"
)]
pub async fn register(
    data: web::Data<AppState>,
    body: web::Json<RegisterRequest>,
) -> Result<Json<UserResponse>, APIError> {
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

    let new_user_id = Uuid::new_v4().to_string(); // Generate user ID here

    let new_user = User {
        id: new_user_id.clone(), // Use the generated user ID
        email: body.email.clone(),
        password_hash,
        role: RoleEnum::Guest,
        google_id: None,
        github_id: None,
        is_verified: false,
        verification_token: Some(verification_token.clone()),
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

    // Create a new Profile record for the user
    let new_profile_id = Uuid::new_v4().to_string();
    let new_profile = NewProfile {
        id: new_profile_id.clone(),
        name: new_user.email.clone(), // Using email as a default name for now
        address: None,
        phone: None,
        photo_url: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    diesel::insert_into(profiles::table)
        .values(&new_profile)
        .execute(&mut conn)?;

    // Create a UserProfile entry linking the User to the new Profile
    let new_user_profile = NewUserProfile {
        user_id: new_user_id.clone(),
        profile_id: new_profile_id.clone(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    diesel::insert_into(user_profiles::table)
        .values(&new_user_profile)
        .execute(&mut conn)?;

    let email_config = data.config.clone();
    let email = new_user.email.clone();
    let token = verification_token.clone();
    
    // Send verification email asynchronously
    tokio::spawn(async move {
        if let Err(e) = send_verification_email(&email_config, &email, &token).await {
            warn!(
                "ACTION: Failed to send verification email to {} | error: {:?}",
                email, e
            );
        }
    });

    let created_user: User = users::table
        .filter(users::email.eq(&body.email))
        .select(User::as_select())
        .first(&mut conn)?;

    info!(
        "ACTION: User registered successfully | user_id: {} | email: {}",
        created_user.id, created_user.email
    );
    Ok(Json(UserResponse::from(created_user)))
}

#[api_operation(
    summary = "User login",
    description = "Authenticates a user and returns a JWT.",
    tag = "auth",
    operation_id = "login_user"
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
        .first(&mut conn)?;

    if let Some(lockout_until) = user.lockout_until {
        if lockout_until > Utc::now().naive_utc() {
            warn!(
                "ACTION: User login failed | reason: account locked | user_id: {} | email: {}",
                user.id, user.email
            );
            return Err(APIError::unauthorized("Account is locked. Try again later."));
        }
    }

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
        let attempts = user.failed_login_attempts + 1;
        if attempts >= 5 {
            let lockout_until = Utc::now().naive_utc() + Duration::minutes(15);
            diesel::update(users::table.find(&user.id))
                .set((
                    users::failed_login_attempts.eq(attempts),
                    users::lockout_until.eq(Some(lockout_until)),
                ))
                .execute(&mut conn)?;
        } else {
            diesel::update(users::table.find(&user.id))
                .set(users::failed_login_attempts.eq(attempts))
                .execute(&mut conn)?;
        }

        warn!(
            "ACTION: User login failed | reason: invalid password | user_id: {} | email: {}",
            user.id, user.email
        );
        return Err(APIError::unauthorized("Invalid email or password"));
    }

    diesel::update(users::table.find(&user.id))
        .set((
            users::failed_login_attempts.eq(0),
            users::lockout_until.eq(None::<chrono::NaiveDateTime>),
        ))
        .execute(&mut conn)?;

    let (token, refresh_token, _access_token_expiration) = create_token_pair(&user, &data.config, &data.db_pool)?;

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

    let expires_at = Utc::now()
        .checked_add_signed(Duration::days(data.config.jwt_expiration as i64))
        .ok_or_else(|| APIError::internal("Failed to calculate session expiration"))?
        .naive_utc();

    create_session(
        &mut conn,
        &user.id,
        &hashed_refresh_token,
        user_agent.as_deref(),
        ip_address.as_deref(),
        expires_at,
    ).map_err(APIError::from)?;

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
    summary = "User logout",
    description = "Invalidates the user's session by deleting the refresh token.",
    tag = "auth",
    operation_id = "logout_user"
)]
pub async fn logout(
    data: web::Data<AppState>,
    body: web::Json<RefreshTokenRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let hashed_refresh_token = hash_password(&body.refresh_token)?;

    if let Some(session) = find_session_by_refresh_token_hash(&mut conn, &hashed_refresh_token).map_err(APIError::from)? {
        delete_session(&mut conn, &session.id).map_err(APIError::from)?;
        info!(
            "ACTION: User logged out successfully | user_id: {} | session_id: {}",
            session.user_id, session.id
        );
    } else {
        warn!("ACTION: Logout attempt with invalid refresh token");
    }

    Ok(Json(MessageResponse { message: "Logged out successfully".to_string() }))
}

#[api_operation(
    summary = "Refresh JWT",
    description = "Provides a new JWT by using a refresh token.",
    tag = "auth",
    operation_id = "refresh_token"
)]
pub async fn refresh(
    data: web::Data<AppState>,
    body: web::Json<RefreshTokenRequest>,
    req: HttpRequest,
) -> Result<Json<TokenResponse>, APIError> {
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
    .await?;

    info!(
        "ACTION: JWT refreshed successfully | ip_address: {:?} | user_agent: {:?}",
        ip_address, user_agent
    );
    Ok(Json(TokenResponse {
        token: new_token,
        refresh_token: new_refresh_token,
    }))
}

#[api_operation(
    summary = "Request password reset",
    description = "Sends a password reset link to the user's email.",
    tag = "auth",
    operation_id = "request_password_reset"
)]
pub async fn request_password_reset(
    data: web::Data<AppState>,
    body: web::Json<PasswordResetRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

    if let Ok(user) = users::table
        .filter(users::email.eq(&body.email))
        .select(User::as_select())
        .first::<User>(&mut conn)
    {
        let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 30);
        let hashed_token = hash_password(&token)?;

        diesel::update(users::table.find(&user.id))
            .set((
                users::password_reset_token.eq(Some(hashed_token)),
                users::password_reset_sent_at.eq(Some(Utc::now().naive_utc())),
            ))
            .execute(&mut conn)?;

        let email_config = data.config.clone();
        let email = user.email.clone();
        let token_clone = token.clone();

        tokio::spawn(async move {
            if let Err(e) = send_password_reset_email(&email_config, &email, &token_clone).await {
                warn!(
                    "ACTION: Failed to send password reset email to {} | error: {:?}",
                    email, e
                );
            }
        });
    } else {
        warn!(
            "ACTION: Password reset request for non-existent user | email: {}",
            body.email
        );
    }

    Ok(Json(MessageResponse { message: "Password reset email sent if user exists".to_string() }))
}

#[api_operation(
    summary = "Reset password",
    description = "Resets the user's password using a token.",
    tag = "auth",
    operation_id = "reset_password"
)]
pub async fn reset_password(
    data: web::Data<AppState>,
    token: web::Path<String>,
    body: web::Json<PasswordReset>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let hashed_token = hash_password(&token)?;

    let user: User = users::table
        .filter(users::password_reset_token.eq(hashed_token))
        .select(User::as_select())
        .first(&mut conn)?;

    if let Some(sent_at) = user.password_reset_sent_at {
        if Utc::now().naive_utc() - sent_at > Duration::hours(1) {
            return Err(APIError::unauthorized(
                "Password reset token has expired",
            ));
        }
    } else {
        return Err(APIError::unauthorized(
            "Invalid or expired password reset token",
        ));
    }

    let new_password_hash = hash_password(&body.new_password)?;

    diesel::update(users::table.find(&user.id))
        .set((
            users::password_hash.eq(new_password_hash),
            users::password_reset_token.eq(None::<String>),
            users::password_reset_sent_at.eq(None::<chrono::NaiveDateTime>),
        ))
        .execute(&mut conn)?;

    invalidate_sessions_for_user(&mut conn, &user.id).map_err(APIError::from)?;

    info!(
        "ACTION: User password reset successfully | user_id: {}",
        user.id
    );
    Ok(Json(MessageResponse { message: "Password reset successfully".to_string() }))
}