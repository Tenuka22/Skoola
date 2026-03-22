use actix_web::web;
use actix_web::{HttpRequest, web::Json};
use apistos::api_operation;
use chrono::{Duration, Utc};
use diesel::prelude::*;

use rand::distributions::{Alphanumeric, DistString};
use tracing::{info, warn};

use crate::{
    AppState,
    database::enums::{RoleEnum, VerificationPurpose},
    database::tables::{User, UserSecurity, UserStatus, VerificationToken},
    errors::APIError,
    models::ids::{IdPrefix, generate_prefixed_id},
    models::auth::user::{
        LoginRequest, PasswordReset, PasswordResetRequest, RefreshTokenRequest, RegisterRequest,
        TokenResponse, UserResponse,
    },
    models::{MessageResponse, NewProfile, NewUserProfile},
    schema::{profiles, user_profiles, users, user_security, user_status, verification_tokens},
    services::auth::auth::{create_token_pair, hash_password, refresh_jwt, verify_password},
    services::auth::session::{
        create_session, delete_session, find_session_by_refresh_token_hash,
        invalidate_sessions_for_user,
    },
    services::system::email::{send_password_reset_email, send_verification_email},
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

    let new_user_id = generate_prefixed_id(&mut conn, IdPrefix::USER)?;

    let now = Utc::now().naive_utc();

    let new_user = User {
        id: new_user_id.clone(),
        email: body.email.clone(),
        password_hash,
        role: RoleEnum::Guest,
        created_at: now,
        updated_at: now,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut conn)?;

    let security = UserSecurity {
        user_id: new_user_id.clone(),
        google_id: None,
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
        .values(&security)
        .execute(&mut conn)?;

    let status = UserStatus {
        user_id: new_user_id.clone(),
        is_verified: false,
        is_active: true,
        disabled_at: None,
        disabled_reason: None,
        created_at: now,
        updated_at: now,
    };
    diesel::insert_into(user_status::table)
        .values(&status)
        .execute(&mut conn)?;

    // Create a new Profile record for the user
    let new_profile_id = generate_prefixed_id(&mut conn, IdPrefix::PROFILE)?;
    let new_profile = NewProfile {
        id: new_profile_id.clone(),
        name: new_user.email.clone(),
        created_at: now,
        updated_at: now,
    };
    diesel::insert_into(profiles::table)
        .values(&new_profile)
        .execute(&mut conn)?;

    // Create a UserProfile entry linking the User to the new Profile
    let new_user_profile = NewUserProfile {
        user_id: new_user_id.clone(),
        profile_id: new_profile_id.clone(),
        created_at: now,
        updated_at: now,
    };
    diesel::insert_into(user_profiles::table)
        .values(&new_user_profile)
        .execute(&mut conn)?;

    let email_config = data.config.clone();
    let email = new_user.email.clone();
    let token = verification_token.clone();
    let verification_expires_at = now + Duration::hours(24);
    let new_verification_token = VerificationToken {
        id: generate_prefixed_id(&mut conn, IdPrefix::VERIFICATION_TOKEN)?,
        user_id: new_user_id.clone(),
        token_hash: token.clone(),
        purpose: VerificationPurpose::EmailVerification,
        issued_at: now,
        expires_at: verification_expires_at,
        consumed_at: None,
        is_active: true,
        metadata: None,
    };
    diesel::insert_into(verification_tokens::table)
        .values(&new_verification_token)
        .execute(&mut conn)?;

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

    let security: UserSecurity = user_security::table
        .filter(user_security::user_id.eq(&user.id))
        .select(UserSecurity::as_select())
        .first(&mut conn)?;
    let status: UserStatus = user_status::table
        .filter(user_status::user_id.eq(&user.id))
        .select(UserStatus::as_select())
        .first(&mut conn)?;

    if let Some(lockout_until) = security.lockout_until {
        if lockout_until > Utc::now().naive_utc() {
            warn!(
                "ACTION: User login failed | reason: account locked | user_id: {} | email: {}",
                user.id, user.email
            );
            return Err(APIError::unauthorized(
                "Account is locked. Try again later.",
            ));
        }
    }

    if !status.is_verified {
        warn!(
            "ACTION: User login failed | reason: email not verified | user_id: {} | email: {}",
            user.id, user.email
        );
        return Err(APIError::unauthorized(
            "Email not verified. Please check your inbox for a verification link.",
        ));
    }

    if !verify_password(&body.password, &user.password_hash)? {
        let attempts = security.failed_login_attempts + 1;
        if attempts >= 5 {
            let lockout_until = Utc::now().naive_utc() + Duration::minutes(15);
            diesel::update(user_security::table.filter(user_security::user_id.eq(&user.id)))
                .set((
                    user_security::failed_login_attempts.eq(attempts),
                    user_security::lockout_until.eq(Some(lockout_until)),
                    user_security::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(&mut conn)?;
        } else {
            diesel::update(user_security::table.filter(user_security::user_id.eq(&user.id)))
                .set((
                    user_security::failed_login_attempts.eq(attempts),
                    user_security::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(&mut conn)?;
        }

        warn!(
            "ACTION: User login failed | reason: invalid password | user_id: {} | email: {}",
            user.id, user.email
        );
        return Err(APIError::unauthorized("Invalid email or password"));
    }

    diesel::update(user_security::table.filter(user_security::user_id.eq(&user.id)))
        .set((
            user_security::failed_login_attempts.eq(0),
            user_security::lockout_until.eq(None::<chrono::NaiveDateTime>),
            user_security::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    let (token, refresh_token, _access_token_expiration) =
        create_token_pair(&user, &data.config, &data.db_pool)?;


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
        &refresh_token,
        user_agent.as_deref(),
        ip_address.as_deref(),
        expires_at,
    )
    .map_err(APIError::from)?;

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

    if let Some(session) = find_session_by_refresh_token_hash(&mut conn, &body.refresh_token)
        .map_err(APIError::from)?
    {
        delete_session(&mut conn, &session.id).map_err(APIError::from)?;
        info!(
            "ACTION: User logged out successfully | user_id: {} | session_id: {}",
            session.user_id, session.id
        );
    } else {
        warn!("ACTION: Logout attempt with invalid refresh token");
    }

    Ok(Json(MessageResponse {
        message: "Logged out successfully".to_string(),
    }))
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
        diesel::update(user_security::table.filter(user_security::user_id.eq(&user.id)))
            .set((
                user_security::password_reset_token.eq(Some(token.clone())),
                user_security::password_reset_sent_at.eq(Some(Utc::now().naive_utc())),
                user_security::updated_at.eq(Utc::now().naive_utc()),
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

    Ok(Json(MessageResponse {
        message: "Password reset email sent if user exists".to_string(),
    }))
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
    let raw_token = token.into_inner();
    let user: User = users::table
        .inner_join(user_security::table.on(users::id.eq(user_security::user_id)))
        .filter(user_security::password_reset_token.eq(raw_token))
        .select(User::as_select())
        .first(&mut conn)?;

    let reset_sent_at: Option<chrono::NaiveDateTime> = user_security::table
        .filter(user_security::user_id.eq(&user.id))
        .select(user_security::password_reset_sent_at)
        .first(&mut conn)
        .optional()?
        .flatten();

    if let Some(sent_at) = reset_sent_at {
        if Utc::now().naive_utc() - sent_at > Duration::hours(1) {
            return Err(APIError::unauthorized("Password reset token has expired"));
        }
    } else {
        return Err(APIError::unauthorized(
            "Invalid or expired password reset token",
        ));
    }

    let new_password_hash = hash_password(&body.new_password)?;

    diesel::update(users::table.find(&user.id))
        .set(users::password_hash.eq(new_password_hash))
        .execute(&mut conn)?;
    diesel::update(user_security::table.filter(user_security::user_id.eq(&user.id)))
        .set((
            user_security::password_reset_token.eq(None::<String>),
            user_security::password_reset_sent_at.eq(None::<chrono::NaiveDateTime>),
            user_security::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    invalidate_sessions_for_user(&mut conn, &user.id).map_err(APIError::from)?;

    info!(
        "ACTION: User password reset successfully | user_id: {}",
        user.id
    );
    Ok(Json(MessageResponse {
        message: "Password reset successfully".to_string(),
    }))
}

