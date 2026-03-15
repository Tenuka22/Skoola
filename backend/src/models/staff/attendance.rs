use crate::database::enums::{
    AttendanceStatus, SubstitutionStatus, TeacherPeriodStatus,
};
use crate::models::staff::staff::Staff;
use crate::schema::{staff_attendance, substitutions, teacher_period_attendance};
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
    AsChangeset,
    Clone,
    Associations,
    ApiComponent,
)]
#[diesel(table_name = staff_attendance)]
#[diesel(belongs_to(Staff))]
pub struct StaffAttendance {
    pub id: String,
    pub staff_id: String,
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub time_in: Option<NaiveTime>,
    pub time_out: Option<NaiveTime>,
    pub remarks: Option<String>,
    pub reason_type: Option<String>,
    pub reason_details: Option<String>,
    pub half_day_type: Option<String>,
    pub out_of_school_from: Option<NaiveTime>,
    pub out_of_school_to: Option<NaiveTime>,
    pub attendance_context: Option<String>,
    pub event_id: Option<String>,
    pub approved_by: Option<String>,
    pub approval_status: Option<String>,
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
    AsChangeset,
    Clone,
    Associations,
    ApiComponent,
)]
#[diesel(table_name = teacher_period_attendance)]
#[diesel(belongs_to(Staff, foreign_key = teacher_id))]
pub struct TeacherPeriodAttendance {
    pub id: String,
    pub teacher_id: String,
    pub timetable_id: String,
    pub date: NaiveDate,
    pub status: TeacherPeriodStatus,
    pub remarks: Option<String>,
    pub marked_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_substitution: bool,
    pub substitution_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = teacher_period_attendance)]
pub struct CreateTeacherPeriodAttendanceRequest {
    pub teacher_id: String,
    pub timetable_id: String,
    pub date: NaiveDate,
    pub status: TeacherPeriodStatus,
    pub remarks: Option<String>,
    pub marked_by: String,
    pub is_substitution: bool,
    pub substitution_id: Option<String>,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Clone,
    ApiComponent,
)]
#[diesel(table_name = substitutions)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = substitutions)]
pub struct CreateSubstitutionModelRequest {
    pub original_teacher_id: String,
    pub substitute_teacher_id: String,
    pub timetable_id: String,
    pub date: NaiveDate,
    pub status: SubstitutionStatus,
    pub remarks: Option<String>,
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
pub struct MarkTeacherPeriodAttendanceRequest {
    pub timetable_id: String,
    pub date: NaiveDate,
    pub status: TeacherPeriodStatus,
    pub remarks: Option<String>,
    pub is_substitution: bool,
    pub substitution_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct TeacherPeriodAttendanceResponse {
    pub id: String,
    pub teacher_id: String,
    pub timetable_id: String,
    pub date: NaiveDate,
    pub status: TeacherPeriodStatus,
    pub remarks: Option<String>,
    pub is_substitution: bool,
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
            status: attendance.status,
            time_in: attendance.time_in,
            time_out: attendance.time_out,
            remarks: attendance.remarks,
            created_at: attendance.created_at,
            updated_at: attendance.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct UpdateStaffAttendanceRequest {
    pub status: Option<AttendanceStatus>,
    pub time_in: Option<NaiveTime>,
    pub time_out: Option<NaiveTime>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct BulkMarkStaffAttendanceRequest {
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub staff_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct SuggestSubstituteRequest {
    pub date: NaiveDate,
    pub timetable_id: String,
}

#[derive(Debug, Serialize, JsonSchema, ApiComponent)]
pub struct SubstitutionResponse {
    pub id: String,
    pub original_teacher_id: String,
    pub substitute_teacher_id: String,
    pub timetable_id: String,
    pub date: NaiveDate,
    pub status: String,
    pub remarks: Option<String>,
    pub plan_name: Option<String>,
    pub content_link: Option<String>,
}
