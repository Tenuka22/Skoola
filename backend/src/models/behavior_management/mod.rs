use crate::schema::{behavior_incident_types, behavior_incidents};
use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use apistos::ApiComponent;

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_types)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BehaviorIncidentType {
    pub id: String,
    pub type_name: String,
    pub default_points: i32,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incidents)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BehaviorIncident {
    pub id: String,
    pub student_id: String,
    pub reported_by_user_id: String,
    pub incident_type_id: String,
    pub description: String,
    pub incident_date: NaiveDateTime,
    pub points_awarded: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incident_types)]
pub struct NewBehaviorIncidentType {
    pub id: String,
    pub type_name: String,
    pub default_points: i32,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = behavior_incidents)]
pub struct NewBehaviorIncident {
    pub id: String,
    pub student_id: String,
    pub reported_by_user_id: String,
    pub incident_type_id: String,
    pub description: String,
    pub incident_date: NaiveDateTime,
    pub points_awarded: i32,
}
