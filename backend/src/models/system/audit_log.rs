use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::audit_log)]
#[diesel(treat_compound_current_type_as_possible_option_type = "true")]
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
