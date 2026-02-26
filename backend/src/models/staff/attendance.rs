use crate::database::enums::{AttendanceStatus, SubstitutionStatus};
use crate::models::staff::staff::Staff;
use crate::schema::{lesson_progress, staff_attendance, substitutions};
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    Clone,
    Associations,
    ApiComponent,
)]
#[diesel(table_name = staff_attendance)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffAttendance {
    pub id: String,
    pub staff_id: String,
    pub date: NaiveDate,
    pub status: String,
    pub time_in: Option<NaiveTime>,
    pub time_out: Option<NaiveTime>,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_locked: bool,
    pub marked_by: Option<String>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    Clone,
    ApiComponent,
)]
#[diesel(table_name = substitutions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Substitution {
    pub id: String,
    pub original_teacher_id: String,
    pub substitute_teacher_id: String,
    pub timetable_id: String,
    pub date: NaiveDate,
    pub status: SubstitutionStatus,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    Clone,
    ApiComponent,
)]
#[diesel(table_name = lesson_progress)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LessonProgress {
    pub id: String,
    pub class_id: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub timetable_id: Option<String>,
    pub date: NaiveDate,
    pub topic_covered: String,
    pub sub_topic: Option<String>,
    pub homework_assigned: Option<String>,
    pub resources_used: Option<String>,
    pub progress_percentage: Option<i32>,
    pub is_substitution: bool,
    pub created_at: NaiveDateTime,
    pub syllabus_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct MarkStaffAttendanceRequest {
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub time_in: Option<NaiveTime>,
    pub time_out: Option<NaiveTime>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct BulkMarkStaffAttendanceItem {
    pub staff_id: String,
    pub status: AttendanceStatus,
    pub time_in: Option<NaiveTime>,
    pub time_out: Option<NaiveTime>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct BulkMarkStaffAttendanceRequest {
    pub date: NaiveDate,
    pub attendance_records: Vec<BulkMarkStaffAttendanceItem>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct StaffAttendanceResponse {
    pub id: String,
    pub staff_id: String,
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub time_in: Option<NaiveTime>,
    pub time_out: Option<NaiveTime>,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StaffAttendance> for StaffAttendanceResponse {
    fn from(attendance: StaffAttendance) -> Self {
        StaffAttendanceResponse {
            id: attendance.id,
            staff_id: attendance.staff_id,
            date: attendance.date,
            status: attendance
                .status
                .parse()
                .unwrap_or(AttendanceStatus::Absent),
            time_in: attendance.time_in,
            time_out: attendance.time_out,
            remarks: attendance.remarks,
            created_at: attendance.created_at,
            updated_at: attendance.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct StaffAttendanceDateQuery {
    pub date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct StaffAttendanceByStaffQuery {
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct MonthlyAttendancePercentageResponse {
    pub staff_id: String,
    pub month: u32,
    pub year: i32,
    pub present_days: i64,
    pub total_working_days: i64,
    pub attendance_percentage: f64,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct UpdateStaffAttendanceRequest {
    pub status: Option<AttendanceStatus>,
    pub time_in: Option<NaiveTime>,
    pub time_out: Option<NaiveTime>,
    pub remarks: Option<String>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize, ApiComponent, JsonSchema)]
#[diesel(table_name = staff_attendance)]
pub struct StaffAttendanceChangeset {
    pub status: Option<String>,
    pub time_in: Option<NaiveTime>,
    pub time_out: Option<NaiveTime>,
    pub remarks: Option<String>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct SuggestSubstituteRequest {
    pub timetable_id: String,
    pub date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CreateSubstitutionRequest {
    pub original_teacher_id: String,
    pub timetable_id: String,
    pub date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct SubstitutionResponse {
    pub id: String,
    pub original_teacher_id: String,
    pub substitute_teacher_id: String,
    pub timetable_id: String,
    pub date: NaiveDate,
    pub status: String,
    pub remarks: Option<String>,
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
    pub syllabus_id: Option<String>,
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
