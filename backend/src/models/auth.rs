use apistos::ApiComponent;
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::database::tables::User;
use crate::database::enums::RoleEnum;

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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id.to_string(),
            email: user.email,
            is_verified: user.is_verified,
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
