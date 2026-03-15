use crate::schema::sessions;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
#[diesel(table_name = sessions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub auth_token_id: Option<String>,
    pub verification_token_id: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub is_active: bool,
    pub disabled_at: Option<NaiveDateTime>,
    pub disabled_reason: Option<String>,
    pub last_seen_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct SessionResponse {
    pub id: String,
    pub user_id: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub is_active: bool,
    pub last_seen_at: Option<NaiveDateTime>,
}

impl From<Session> for SessionResponse {
    fn from(s: Session) -> Self {
        Self {
            id: s.id,
            user_id: s.user_id,
            user_agent: s.user_agent,
            ip_address: s.ip_address,
            created_at: s.created_at,
            expires_at: s.expires_at,
            is_active: s.is_active,
            last_seen_at: s.last_seen_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateSessionRequest {
    pub user_id: String,
    pub auth_token_id: Option<String>,
    pub verification_token_id: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub expires_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = sessions)]
pub struct UpdateSessionRequest {
    pub is_active: Option<bool>,
    pub disabled_at: Option<NaiveDateTime>,
    pub disabled_reason: Option<String>,
    pub last_seen_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct SessionQuery {
    pub user_id: Option<String>,
    pub is_active: Option<bool>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for SessionQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}
