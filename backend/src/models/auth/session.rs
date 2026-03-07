use crate::schema::sessions;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
