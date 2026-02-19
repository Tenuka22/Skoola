use crate::schema::{student_attendance, attendance_policies, exit_passes, pre_approved_absences, attendance_discrepancies, detention_balances, attendance_excuses, student_period_attendance, attendance_audit_log, emergency_roll_calls, emergency_roll_call_entries};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use apistos::ApiComponent;
use crate::database::enums::{AttendanceStatus, PolicyRuleType, ExitReason, PreApprovedReason, ExcuseType, SuspicionFlag, DetailedStatus, EmergencyStatus};

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = student_attendance)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentAttendance {
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = attendance_policies)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AttendancePolicy {
    pub id: String,
    pub name: String,
    pub rule_type: PolicyRuleType,
    pub threshold: i32,
    pub consequence_type: String,
    pub consequence_value: Option<f32>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = exit_passes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ExitPass {
    pub id: String,
    pub student_id: String,
    pub date: NaiveDate,
    pub exit_time: NaiveTime,
    pub reason_type: ExitReason,
    pub remarks: Option<String>,
    pub approved_by: String,
    pub guardian_notified: bool,
    pub gate_cleared_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = pre_approved_absences)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PreApprovedAbsence {
    pub id: String,
    pub student_id: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason_type: PreApprovedReason,
    pub remarks: Option<String>,
    pub approved_by: String,
    pub document_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = emergency_roll_calls)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct EmergencyRollCall {
    pub id: String,
    pub event_name: String,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub initiated_by: String,
    pub status: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = emergency_roll_call_entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(roll_call_id, user_id))]
pub struct EmergencyRollCallEntry {
    pub roll_call_id: String,
    pub user_id: String,
    pub status: EmergencyStatus,
    pub location_found: Option<String>,
    pub marked_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = attendance_discrepancies)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AttendanceDiscrepancy {
    pub id: String,
    pub student_id: String,
    pub date: NaiveDate,
    pub discrepancy_type: String,
    pub details: Option<String>,
    pub severity: String,
    pub is_resolved: bool,
    pub resolved_by: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = detention_balances)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DetentionBalance {
    pub student_id: String,
    pub total_hours_assigned: f32,
    pub total_hours_served: f32,
    pub remaining_hours: f32,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = attendance_excuses)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AttendanceExcuse {
    pub id: String,
    pub attendance_record_id: String,
    pub excuse_type: ExcuseType,
    pub document_url: Option<String>,
    pub is_verified: bool,
    pub verified_by: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = student_period_attendance)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentPeriodAttendance {
    pub id: String,
    pub student_id: String,
    pub class_id: String,
    pub timetable_id: String,
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub minutes_late: Option<i32>,
    pub remarks: Option<String>,
    pub is_locked: bool,
    pub marked_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub suspicion_flag: Option<SuspicionFlag>,
    pub detailed_status: Option<DetailedStatus>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = attendance_audit_log)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AttendanceAuditLog {
    pub id: String,
    pub attendance_type: String,
    pub attendance_record_id: String,
    pub old_status: Option<String>,
    pub new_status: String,
    pub change_reason: String,
    pub changed_by: String,
    pub changed_at: NaiveDateTime,
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
    pub student_name: String,
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
    pub reason: ExitReason,
}

#[derive(Debug, Serialize, JsonSchema, ApiComponent)]
pub struct ExitPassResponse {
    pub id: String,
    pub student_id: String,
    pub date: NaiveDate,
    pub exit_time: NaiveTime,
    pub reason_type: ExitReason,
    pub remarks: Option<String>,
    pub approved_by: String,
    pub guardian_notified: bool,
}

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
