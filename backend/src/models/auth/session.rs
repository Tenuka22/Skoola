use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use apistos::ApiComponent;
use diesel::prelude::*;
use crate::schema::sessions;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = sessions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub refresh_token_hash: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
}
