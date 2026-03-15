use chrono::{NaiveDateTime, NaiveDate};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

// Behavior Incident Severity Levels
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_severity_levels)]
pub struct BehaviorIncidentSeverityLevel {
    pub id: String,
    pub name: String,
    pub points: i32,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_severity_levels)]
pub struct CreateBehaviorIncidentSeverityLevelRequest {
    pub name: String,
    pub points: i32,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_severity_levels)]
pub struct UpdateBehaviorIncidentSeverityLevelRequest {
    pub name: Option<String>,
    pub points: Option<i32>,
    pub description: Option<String>,
}

// Behavior Incident Actions
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_actions)]
pub struct BehaviorIncidentAction {
    pub id: String,
    pub incident_id: String,
    pub action_type: String,
    pub action_details: Option<String>,
    pub assigned_to: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_actions)]
pub struct CreateBehaviorIncidentActionRequest {
    pub incident_id: String,
    pub action_type: String,
    pub action_details: Option<String>,
    pub assigned_to: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_actions)]
pub struct UpdateBehaviorIncidentActionRequest {
    pub action_type: Option<String>,
    pub action_details: Option<String>,
    pub assigned_to: Option<String>,
    pub due_date: Option<NaiveDate>,
    pub status: Option<String>,
}

// Behavior Incident Evidence
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_evidence)]
pub struct BehaviorIncidentEvidence {
    pub id: String,
    pub incident_id: String,
    pub file_url: String,
    pub file_type: Option<String>,
    pub uploaded_by: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_evidence)]
pub struct CreateBehaviorIncidentEvidenceRequest {
    pub incident_id: String,
    pub file_url: String,
    pub file_type: Option<String>,
    pub uploaded_by: Option<String>,
}

// Behavior Incident Followups
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_followups)]
pub struct BehaviorIncidentFollowup {
    pub id: String,
    pub incident_id: String,
    pub followup_date: NaiveDate,
    pub notes: Option<String>,
    pub recorded_by: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_followups)]
pub struct CreateBehaviorIncidentFollowupRequest {
    pub incident_id: String,
    pub followup_date: NaiveDate,
    pub notes: Option<String>,
    pub recorded_by: Option<String>,
}

// Behavior Incident Details
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_details)]
pub struct BehaviorIncidentDetails {
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

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_details)]
pub struct CreateBehaviorIncidentDetailsRequest {
    pub incident_id: String,
    pub description: String,
    pub points_awarded: i32,
    pub severity_id: Option<String>,
    pub status: String,
    pub resolved_by: Option<String>,
    pub resolved_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_details)]
pub struct UpdateBehaviorIncidentDetailsRequest {
    pub description: Option<String>,
    pub points_awarded: Option<i32>,
    pub severity_id: Option<String>,
    pub status: Option<String>,
    pub resolved_by: Option<String>,
    pub resolved_at: Option<NaiveDateTime>,
}
