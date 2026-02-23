use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::behavior_incident_types)]
pub struct BehaviorIncidentType {
    pub id: String,
    pub type_name: String,
    pub default_points: i32,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::behavior_incident_types)]
pub struct NewBehaviorIncidentType {
    pub id: String,
    pub type_name: String,
    pub default_points: i32,
    pub description: Option<String>,
}
