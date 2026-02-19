use apistos::ApiComponent;
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use diesel::prelude::*;
use crate::database::enums::RoleEnum;
use crate::schema::users;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Clone,
)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub google_id: Option<String>,
    pub github_id: Option<String>,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub verification_sent_at: Option<NaiveDateTime>,
    pub password_reset_token: Option<String>,
    pub password_reset_sent_at: Option<NaiveDateTime>,
    pub failed_login_attempts: i32,
    pub lockout_until: Option<NaiveDateTime>,
    pub role: RoleEnum,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub google_id: Option<String>,
    pub github_id: Option<String>,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub verification_sent_at: Option<NaiveDateTime>,
    pub password_reset_token: Option<String>,
    pub password_reset_sent_at: Option<NaiveDateTime>,
    pub failed_login_attempts: i32,
    pub lockout_until: Option<NaiveDateTime>,
    pub role: RoleEnum,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub is_verified: bool,
    pub role: RoleEnum,
    pub lockout_until: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            is_verified: user.is_verified,
            role: user.role,
            lockout_until: user.lockout_until,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl From<crate::database::tables::User> for UserResponse {
    fn from(user: crate::database::tables::User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            is_verified: user.is_verified,
            role: user.role,
            lockout_until: user.lockout_until,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct UserProfileResponse {
    pub id: String,
    pub email: String,
    pub is_verified: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub roles: Vec<RoleEnum>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct TokenResponse {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct ResendVerificationEmailRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PasswordResetRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PasswordReset {
    pub new_password: String,
}
