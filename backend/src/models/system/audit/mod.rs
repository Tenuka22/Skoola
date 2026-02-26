use crate::schema::audit_log;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = audit_log)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AuditLog {
    pub id: String,
    pub user_id: String,
    pub action_type: String,
    pub table_name: String,
    pub record_pk: String,
    pub old_value_json: Option<String>,
    pub new_value_json: Option<String>,
    pub timestamp: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = audit_log)]
pub struct NewAuditLog {
    pub id: String,
    pub user_id: String,
    pub action_type: String,
    pub table_name: String,
    pub record_pk: String,
    pub old_value_json: Option<String>,
    pub new_value_json: Option<String>,
}
