use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use apistos::ApiComponent;
use diesel::prelude::*;
use crate::schema::{activities, activity_attendance, activity_participants, activity_types};
use crate::database::enums::{AttendanceStatus, ParticipantType};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = activity_types)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ActivityType {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = activities)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Activity {
    pub id: String,
    pub activity_type_id: String,
    pub name: String,
    pub description: Option<String>,
    pub location: Option<String>,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub is_mandatory: bool,
    pub academic_year_id: String,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = activity_participants)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(activity_id, user_id))]
pub struct ActivityParticipant {
    pub activity_id: String,
    pub user_id: String,
    pub participant_type: ParticipantType,
    pub enrollment_reason: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = activity_attendance)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ActivityAttendance {
    pub id: String,
    pub activity_id: String,
    pub user_id: String,
    pub status: AttendanceStatus,
    pub check_in_time: Option<NaiveDateTime>,
    pub check_out_time: Option<NaiveDateTime>,
    pub remarks: Option<String>,
    pub marked_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

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
    pub participant_type: String,
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
    pub status: String,
}
