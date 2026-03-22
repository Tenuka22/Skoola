use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;
use crate::schema::behavior_incident_severity_levels;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_severity_levels)]
pub struct BehaviorIncidentSeverityLevel {
    pub id: String,
    pub name: String,
    pub points: i32,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_severity_levels)]
pub struct CreateBehaviorIncidentSeverityLevelRequest {
    pub name: String,
    pub points: i32,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_severity_levels)]
pub struct UpdateBehaviorIncidentSeverityLevelRequest {
    pub name: Option<String>,
    pub points: Option<i32>,
    pub description: Option<String>,
}
