use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use apistos::ApiComponent;
use chrono::NaiveDate;

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct MarkPeriodAttendanceRequest {
    pub student_id: String,
    pub class_id: String,
    pub timetable_id: String,
    pub date: NaiveDate,
    pub status: String,
    pub minutes_late: Option<i32>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
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

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateLessonProgressRequest {
    pub class_id: String,
    pub subject_id: String,
    pub timetable_id: String,
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
