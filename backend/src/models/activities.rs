use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use apistos::ApiComponent;

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateActivityRequest {
    pub activity_type_id: String,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub is_mandatory: bool,
    pub academic_year_id: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct ActivityResponse {
    pub id: String,
    pub activity_type_id: String,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub is_mandatory: bool,
    pub created_by: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct EnrollParticipantRequest {
    pub user_id: String,
    pub participant_type: String, // Changed from ParticipantType to String
    pub enrollment_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct ActivityTypeResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateActivityTypeRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct MarkActivityAttendanceRequest {
    pub user_id: String,
    pub status: String, // String to be parsed to AttendanceStatus
}
