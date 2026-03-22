use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;
use crate::schema::behavior_incident_details;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_details)]
pub struct BehaviorIncidentDetail {
    pub incident_id: String,
    pub description: String,
    pub points_awarded: i32,
    pub severity_id: Option<String>,
    pub status: String,
    pub resolved_by: Option<String>,
    pub resolved_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_details)]
pub struct NewBehaviorIncidentDetail {
    pub incident_id: String,
    pub description: String,
    pub points_awarded: i32,
    pub severity_id: Option<String>,
    pub status: String,
}
