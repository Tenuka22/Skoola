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

impl From<CreateBehaviorIncidentSeverityLevelRequest> for BehaviorIncidentSeverityLevel {
    fn from(req: CreateBehaviorIncidentSeverityLevelRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: req.name,
            points: req.points,
            description: req.description,
            created_at: now,
        }
    }
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

impl From<CreateBehaviorIncidentActionRequest> for BehaviorIncidentAction {
    fn from(req: CreateBehaviorIncidentActionRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            incident_id: req.incident_id,
            action_type: req.action_type,
            action_details: req.action_details,
            assigned_to: req.assigned_to,
            due_date: req.due_date,
            status: req.status,
            created_at: now,
            updated_at: now,
        }
    }
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

impl From<CreateBehaviorIncidentEvidenceRequest> for BehaviorIncidentEvidence {
    fn from(req: CreateBehaviorIncidentEvidenceRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            incident_id: req.incident_id,
            file_url: req.file_url,
            file_type: req.file_type,
            uploaded_by: req.uploaded_by,
            created_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_evidence)]
pub struct UpdateBehaviorIncidentEvidenceRequest {
    pub file_url: Option<String>,
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

impl From<CreateBehaviorIncidentFollowupRequest> for BehaviorIncidentFollowup {
    fn from(req: CreateBehaviorIncidentFollowupRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            incident_id: req.incident_id,
            followup_date: req.followup_date,
            notes: req.notes,
            recorded_by: req.recorded_by,
            created_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_followups)]
pub struct UpdateBehaviorIncidentFollowupRequest {
    pub followup_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub recorded_by: Option<String>,
}

// Behavior Incident Details
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_details)]
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

impl From<CreateBehaviorIncidentDetailsRequest> for BehaviorIncidentDetail {
    fn from(req: CreateBehaviorIncidentDetailsRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            incident_id: req.incident_id,
            description: req.description,
            points_awarded: req.points_awarded,
            severity_id: req.severity_id,
            status: req.status,
            resolved_by: req.resolved_by,
            resolved_at: req.resolved_at,
            created_at: now,
            updated_at: now,
        }
    }
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

// Behavior Incident Type Request Types
#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_types)]
pub struct CreateBehaviorIncidentTypeRequest {
    pub type_name: String,
    pub default_points: i32,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incident_types)]
pub struct UpdateBehaviorIncidentTypeRequest {
    pub type_name: Option<String>,
    pub default_points: Option<i32>,
    pub description: Option<String>,
}

// Behavior Incident Request Types
#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incidents)]
pub struct RecordBehaviorIncidentRequest {
    pub student_id: String,
    pub incident_type_id: String,
    pub incident_date: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::behavior_incidents)]
pub struct UpdateBehaviorIncidentRequest {
    pub student_id: Option<String>,
    pub reported_by_user_id: Option<String>,
    pub incident_type_id: Option<String>,
    pub incident_date: Option<NaiveDateTime>,
}
