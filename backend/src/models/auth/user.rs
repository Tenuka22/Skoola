use crate::database::enums::{Gender, RoleEnum, StaffType};
use crate::schema::users;
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    pub role: RoleEnum,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub role: RoleEnum,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct UserQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for UserQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub is_verified: Option<bool>,
    pub role: RoleEnum,
    pub auth_method: String,
    pub lockout_until: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            is_verified: None,
            role: user.role,
            auth_method: "Password".to_string(),
            lockout_until: None,
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
            is_verified: None,
            role: user.role,
            auth_method: "Password".to_string(),
            lockout_until: None,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct UserProfileResponse {
    pub id: String,
    pub email: String,
    pub is_verified: Option<bool>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub roles: Vec<RoleEnum>,
    // Profile related fields
    pub profile_id: Option<String>,
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub photo_url: Option<String>,
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

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CreateUserRequest {
    pub email: String,
    pub password: String,
    pub role: RoleEnum,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct UpdateUserRequest {
    // users
    pub email: Option<String>,
    pub password: Option<String>,
    pub role: Option<RoleEnum>,

    // user_status
    pub is_verified: Option<bool>,
    pub is_active: Option<bool>,
    pub disabled_reason: Option<String>,

    // From Profile
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub photo_url: Option<String>,

    // staff-specific
    pub staff_type: Option<StaffType>,

    // student-specific
    pub name_sinhala: Option<String>,
    pub name_tamil: Option<String>,

    pub lockout_until: Option<NaiveDateTime>,
    // common for student and staff
    pub gender: Option<Gender>,
    pub dob: Option<NaiveDate>,
}


#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct UserPermissionsResponse {
    pub permissions: Vec<String>,
}
