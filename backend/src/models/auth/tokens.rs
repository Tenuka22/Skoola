use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::database::enums::{AuthTokenType, VerificationPurpose};

use crate::services::admin_db::{AdminQuery, AsAdminQuery};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    Clone,
    ApiComponent,
)]
#[diesel(table_name = crate::schema::auth_tokens)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AuthToken {
    pub id: String,
    pub user_id: String,
    pub token_hash: String,
    pub token_type: AuthTokenType,
    pub issued_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub revoked_at: Option<NaiveDateTime>,
    pub is_active: bool,
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct AuthTokenResponse {
    pub id: String,
    pub user_id: String,
    pub token_type: AuthTokenType,
    pub issued_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub revoked_at: Option<NaiveDateTime>,
    pub is_active: bool,
    pub metadata: Option<String>,
}

impl From<AuthToken> for AuthTokenResponse {
    fn from(t: AuthToken) -> Self {
        Self {
            id: t.id,
            user_id: t.user_id,
            token_type: t.token_type,
            issued_at: t.issued_at,
            expires_at: t.expires_at,
            revoked_at: t.revoked_at,
            is_active: t.is_active,
            metadata: t.metadata,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateAuthTokenRequest {
    pub user_id: String,
    pub token_hash: String,
    pub token_type: AuthTokenType,
    pub expires_at: NaiveDateTime,
    pub metadata: Option<String>,
}

impl From<CreateAuthTokenRequest> for AuthToken {
    fn from(req: CreateAuthTokenRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: req.user_id,
            token_hash: req.token_hash,
            token_type: req.token_type,
            issued_at: now,
            expires_at: req.expires_at,
            revoked_at: None,
            is_active: true,
            metadata: req.metadata,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = crate::schema::auth_tokens)]
pub struct UpdateAuthTokenRequest {
    pub revoked_at: Option<NaiveDateTime>,
    pub is_active: Option<bool>,
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct AuthTokenQuery {
    pub user_id: Option<String>,
    pub token_type: Option<AuthTokenType>,
    pub is_active: Option<bool>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for AuthTokenQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Clone,
    ApiComponent,
)]
#[diesel(table_name = crate::schema::verification_tokens)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct VerificationToken {
    pub id: String,
    pub user_id: String,
    pub token_hash: String,
    pub purpose: VerificationPurpose,
    pub issued_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub consumed_at: Option<NaiveDateTime>,
    pub is_active: bool,
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateVerificationTokenRequest {
    pub user_id: String,
    pub token_hash: String,
    pub purpose: VerificationPurpose,
    pub expires_at: NaiveDateTime,
    pub metadata: Option<String>,
}

impl From<CreateVerificationTokenRequest> for VerificationToken {
    fn from(req: CreateVerificationTokenRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id: req.user_id,
            token_hash: req.token_hash,
            purpose: req.purpose,
            issued_at: now,
            expires_at: req.expires_at,
            consumed_at: None,
            is_active: true,
            metadata: req.metadata,
        }
    }
}
