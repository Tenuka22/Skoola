use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SchoolCalendarResponse {
    pub date: NaiveDate,
    pub day_type: String,
    pub name: Option<String>,
    pub is_academic_day: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct EnrollParticipantRequest {
    pub user_id: String,
    pub participant_type: String,
    pub enrollment_reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct MarkPeriodAttendanceRequest {
    pub student_id: String,
    pub class_id: String,
    pub timetable_id: String,
    pub date: NaiveDate,
    pub status: String,
    pub minutes_late: Option<i32>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AttendanceAuditResponse {
    pub id: String,
    pub attendance_type: String,
    pub attendance_record_id: String,
    pub old_status: Option<String>,
    pub new_status: String,
    pub change_reason: String,
    pub changed_by: String,
    pub changed_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateLessonProgressRequest {
    pub class_id: String,
    pub subject_id: String,
    pub timetable_id: Option<String>,
    pub date: NaiveDate,
    pub topic_covered: String,
    pub sub_topic: Option<String>,
    pub homework_assigned: Option<String>,
    pub resources_used: Option<String>,
    pub progress_percentage: Option<i32>,
    pub is_substitution: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct LessonProgressResponse {
    pub id: String,
    pub class_id: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub date: NaiveDate,
    pub topic_covered: String,
    pub progress_percentage: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SubmitExcuseRequest {
    pub attendance_record_id: String,
    pub excuse_type: String,
    pub document_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AttendanceExcuseResponse {
    pub id: String,
    pub attendance_record_id: String,
    pub excuse_type: String,
    pub is_verified: bool,
}
