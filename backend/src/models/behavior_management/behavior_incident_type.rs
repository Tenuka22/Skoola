use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_types)]
pub struct BehaviorIncidentType {
    pub id: String,
    pub type_name: String,
    pub default_points: i32,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_types)]
pub struct NewBehaviorIncidentType {
    pub id: String,
    pub type_name: String,
    pub default_points: i32,
    pub description: Option<String>,
}
