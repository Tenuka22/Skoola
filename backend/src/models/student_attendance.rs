use crate::schema::student_attendance;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use apistos::ApiComponent;
use crate::database::enums::AttendanceStatus;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema)]
#[diesel(table_name = student_attendance)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentAttendance {
    pub id: String,
    pub student_id: String,
    pub class_id: String,
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub marked_by: String, // User ID of the staff member marking attendance
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_locked: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct MarkStudentAttendanceRequest {
    pub student_id: String,
    pub class_id: String,
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub marked_by: String,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct BulkMarkStudentAttendanceRequest {
    pub attendance_records: Vec<MarkStudentAttendanceRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = student_attendance)]
pub struct UpdateStudentAttendanceRequest {
    pub date: Option<NaiveDate>,
    pub status: Option<AttendanceStatus>,
    pub marked_by: Option<String>,
    pub remarks: Option<String>,
    pub is_locked: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct StudentAttendanceResponse {
    pub id: String,
    pub student_id: String,
    pub class_id: String,
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub marked_by: String,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_locked: bool,
}

impl From<StudentAttendance> for StudentAttendanceResponse {
    fn from(attendance: StudentAttendance) -> Self {
        StudentAttendanceResponse {
            id: attendance.id,
            student_id: attendance.student_id,
            class_id: attendance.class_id,
            date: attendance.date,
            status: attendance.status,
            marked_by: attendance.marked_by,
            remarks: attendance.remarks,
            created_at: attendance.created_at,
            updated_at: attendance.updated_at,
            is_locked: attendance.is_locked,
        }
    }
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct GetAttendanceByClassAndDatePath {
    pub class_id: String,
    pub date: NaiveDate,
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct GetAttendanceByStudentPath {
    pub student_id: String,
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct GenerateAttendanceReportRequest {
    pub class_id: String,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
}

#[derive(Debug, Serialize, JsonSchema, ApiComponent)]
pub struct StudentAttendanceReportResponse {
    pub student_id: String,
    pub student_name: String, // Assuming student name can be retrieved or joined
    pub total_days: i64,
    pub days_present: i64,
    pub days_absent: i64,
    pub days_late: i64,
    pub percentage: f64,
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct LowAttendanceStudentQuery {
    pub class_id: String,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub threshold_percentage: f64,
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]

pub struct SendAbsenceNotificationRequest {

    pub class_id: String,

    pub date: NaiveDate,

}



#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]



pub struct InitiateEmergencyRollCallRequest {



    pub event_name: String,



}







#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]



pub struct IssueExitPassRequest {



    pub student_id: String,



    pub exit_time: NaiveTime,



    pub reason: crate::database::enums::ExitReason,



}







#[derive(Debug, Serialize, JsonSchema, ApiComponent)]



pub struct ExitPassResponse {



    pub id: String,



    pub student_id: String,



    pub date: NaiveDate,



    pub exit_time: NaiveTime,



    pub reason_type: crate::database::enums::ExitReason,



    pub remarks: Option<String>,



    pub approved_by: String,



    pub guardian_notified: bool,



}




