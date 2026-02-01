use actix_web::{HttpRequest, web::Json};
use actix_web::{HttpResponse, web};
use apistos::api_operation;
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::database::constants::{SESSION_TABLE, USER_TABLE};
use crate::{
    AppState,
    database::tables::{Role, Session, User},
    errors::APIError,
    models::auth::{
        LoginRequest, RefreshTokenRequest, RegisterRequest, TokenResponse, UserResponse,
    },
    services::auth::{create_token_pair, hash_password, refresh_jwt, verify_password},
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
    let mut response = data
        .database
        .query("SELECT * FROM type::table($table) WHERE email = $email")
        .bind(("table", USER_TABLE))
        .bind(("email", body.email.clone()))
        .await?;
    let users_option: Option<Vec<User>> = response.take(0)?;
    let users: Vec<User> = users_option.unwrap_or_default();

    if !users.is_empty() {
        return Err(APIError::conflict("User with this email already exists"));
    }

    let password_hash = hash_password(&body.password)?;

    let new_user = User {
        id: Uuid::new_v4(),
        email: body.email.clone(),
        password_hash,
        role: Role::Guest, // Default role
        google_id: None,
        github_id: None,
        created_at: chrono::Utc::now().into(),
        updated_at: chrono::Utc::now().into(),
    };

    let created_user: User = data
        .database
        .create(USER_TABLE)
        .content(new_user)
        .await?
        .ok_or_else(|| APIError::internal("Failed to create user"))?;

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
    let mut response = data
        .database
        .query("SELECT * FROM type::table($table) WHERE email = $email")
        .bind(("table", USER_TABLE))
        .bind(("email", body.email.clone()))
        .await?;
    let users_option: Option<Vec<User>> = response.take(1)?;
    let mut users: Vec<User> =
        users_option.ok_or_else(|| APIError::unauthorized("Invalid email or password"))?;

    let user = users
        .pop()
        .ok_or_else(|| APIError::unauthorized("Invalid email or password"))?;

    if !verify_password(&body.password, &user.password_hash)? {
        return Err(APIError::unauthorized("Invalid email or password"));
    }

    let (token, refresh_token) = create_token_pair(&user, &data.config)?;

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

    let new_session = Session {
        id: Uuid::new_v4(),

        user_id: user.id,

        refresh_token_hash: hashed_refresh_token,

        user_agent,

        ip_address,

        created_at: Utc::now(),

        expires_at: Utc::now()
            .checked_add_signed(Duration::days(data.config.jwt_expiration))
            .expect("valid timestamp"),
    };

    let _: Option<Session> = data
        .database
        .create(SESSION_TABLE)
        .content(new_session)
        .await
        .map_err(|_| APIError::internal("Failed to create session"))?;

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
        &data.database,
        ip_address,
        user_agent,
    )
    .await?;

    Ok(HttpResponse::Ok().json(TokenResponse {
        token: new_token,
        refresh_token: new_refresh_token,
    }))
}
