use crate::database::enums::BehaviorIncidentStatus;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incidents)]
pub struct BehaviorIncident {
    pub id: String,
    pub student_id: String,
    pub reported_by_user_id: String,
    pub incident_type_id: String,
    pub incident_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incidents)]
pub struct NewBehaviorIncident {
    pub id: String,
    pub student_id: String,
    pub reported_by_user_id: String,
    pub incident_type_id: String,
    pub incident_date: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_details)]
pub struct BehaviorIncidentDetail {
    pub incident_id: String,
    pub description: String,
    pub points_awarded: i32,
    pub severity_id: Option<String>,
    pub status: BehaviorIncidentStatus,
    pub resolved_by: Option<String>,
    pub resolved_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_details)]
pub struct NewBehaviorIncidentDetail {
    pub incident_id: String,
    pub description: String,
    pub points_awarded: i32,
    pub severity_id: Option<String>,
    pub status: String,
}
