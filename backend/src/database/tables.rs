use crate::database::enums::{
    AllocationType, AppealStatus, AttendanceStatus, ComponentType, DayType, DetailedStatus,
    EmergencyStatus, Ethnicity, ExcuseType, ExitReason, FeeFrequency, Gender, LeaveStatus,
    LessonMaterialType, MaintenanceStatus, Medium, MissedLessonStatus, ParticipantType,
    PaymentMethod, PolicyRuleType, PreApprovedReason, Religion, RoleEnum, StudentStatus,
    SubstitutionStatus, SuspicionFlag, TeacherPeriodStatus, TransactionType, ReviewerType, RewardReasonType,
    ActivityAttendanceStatus
};
use crate::schema::*;
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = academic_years)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AcademicYear {
    pub id: String,
    pub year_start: i32,
    pub year_end: i32,
    pub name: String,
    pub current: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = grade_levels)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GradeLevel {
    pub id: String,
    pub grade_number: i32,
    pub grade_name: String,
    pub education_level: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = streams)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Stream {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = subjects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Subject {
    pub id: String,
    pub subject_code: String,
    pub subject_name_en: String,
    pub subject_name_si: Option<String>,
    pub subject_name_ta: Option<String>,
    pub is_core: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = terms)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Term {
    pub id: String,
    pub academic_year_id: String,
    pub term_number: i32,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = classes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Class {
    pub id: String,
    pub grade_id: String,
    pub section_name: String,
    pub academic_year_id: String,
    pub class_teacher_id: Option<String>,
    pub medium: Medium,
    pub room_number: Option<String>,
    pub max_capacity: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = grade_streams)]
#[diesel(primary_key(grade_id, stream_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GradeStream {
    pub grade_id: String,
    pub stream_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = grade_subjects)]
#[diesel(primary_key(grade_id, subject_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GradeSubject {
    pub grade_id: String,
    pub subject_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = stream_subjects)]
#[diesel(primary_key(stream_id, subject_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StreamSubject {
    pub stream_id: String,
    pub subject_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = timetable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Timetable {
    pub id: String,
    pub class_id: String,
    pub day_of_week: String,
    pub period_number: i32,
    pub subject_id: String,
    pub teacher_id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub room: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = class_subject_teachers)]
#[diesel(primary_key(class_id, subject_id, teacher_id, academic_year_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ClassSubjectTeacher {
    pub class_id: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub google_id: Option<String>,
    pub github_id: Option<String>,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub verification_sent_at: Option<NaiveDateTime>,
    pub password_reset_token: Option<String>,
    pub password_reset_sent_at: Option<NaiveDateTime>,
    pub failed_login_attempts: i32,
    pub lockout_until: Option<NaiveDateTime>,
    pub role: RoleEnum,
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub google_id: Option<String>,
    pub github_id: Option<String>,
    pub is_verified: bool,
    pub verification_token: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub role: RoleEnum,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = profiles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub photo_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = user_profiles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(user_id, profile_id))]
pub struct UserProfile {
    pub user_id: String,
    pub profile_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = staff)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Staff {
    pub id: String,
    pub employee_id: String,
    pub name: String,
    pub nic: String,
    pub dob: NaiveDate,
    pub gender: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub employment_status: crate::database::enums::EmploymentStatus,
    pub staff_type: crate::database::enums::StaffType,
    pub photo_url: Option<String>,
    pub profile_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations, ApiComponent)]
#[diesel(table_name = staff_leaves)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffLeave {
    pub id: String,
    pub staff_id: String,
    pub leave_type: String,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub reason: String,
    pub status: LeaveStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations, ApiComponent)]
#[diesel(table_name = staff_attendance)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffAttendance {
    pub id: String,
    pub staff_id: String,
    pub date: NaiveDate,
    pub status: AttendanceStatus,
    pub time_in: Option<NaiveTime>,
    pub time_out: Option<NaiveTime>,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub is_locked: bool,
    pub marked_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = students)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Student {
    pub id: String,
    pub admission_number: String,
    pub name_english: String,
    pub name_sinhala: Option<String>,
    pub name_tamil: Option<String>,
    pub nic_or_birth_certificate: String,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub address: String,
    pub phone: String,
    pub email: Option<String>,
    pub religion: Option<Religion>,
    pub ethnicity: Option<Ethnicity>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: StudentStatus,
    pub photo_url: Option<String>,
    pub profile_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations, ApiComponent)]
#[diesel(table_name = student_guardians)]
#[diesel(belongs_to(Student))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentGuardian {
    pub id: String,
    pub student_id: String,
    pub name: String,
    pub relationship: String,
    pub phone: String,
    pub email: Option<String>,
    pub address: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub user_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
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
    pub verified_by: Option<String>,
    pub verified_at: Option<NaiveDateTime>,
    pub is_skipped: bool,
    pub priority_level: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = teacher_class_assignments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeacherClassAssignment {
    pub id: String,
    pub teacher_id: String,
    pub class_id: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = teacher_subject_assignments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeacherSubjectAssignment {
    pub id: String,
    pub teacher_id: String,
    pub subject_id: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub medium: Medium,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = user_sets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserSet {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = user_set_users)]
#[diesel(primary_key(user_set_id, user_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserSetUser {
    pub user_set_id: String,
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = role_permissions)]
#[diesel(primary_key(role_id, permission))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RolePermission {
    pub role_id: String,
    pub permission: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = role_sets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RoleSet {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = role_set_roles)]
#[diesel(primary_key(role_set_id, role_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RoleSetRole {
    pub role_set_id: String,
    pub role_id: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = user_permissions)]
#[diesel(primary_key(user_id, permission))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserPermission {
    pub user_id: String,
    pub permission: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = user_set_permissions)]
#[diesel(primary_key(user_set_id, permission))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserSetPermission {
    pub user_set_id: String,
    pub permission: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = fee_categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FeeCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_mandatory: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = fee_structures)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FeeStructure {
    pub id: String,
    pub grade_id: String,
    pub academic_year_id: String,
    pub category_id: String,
    pub amount: f32,
    pub due_date: NaiveDate,
    pub frequency: FeeFrequency,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = asset_categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AssetCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = inventory_items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InventoryItem {
    pub id: String,
    pub category_id: String,
    pub item_name: String,
    pub description: Option<String>,
    pub unit: String,
    pub quantity: i32,
    pub reorder_level: i32,
    pub unit_price: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = library_books)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LibraryBook {
    pub id: i32,
    pub isbn: Option<String>,
    pub title: String,
    pub author: String,
    pub publisher: Option<String>,
    pub category_id: i32,
    pub quantity: i32,
    pub available_quantity: i32,
    pub rack_number: Option<String>,
    pub added_date: NaiveDate,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = library_categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LibraryCategory {
    pub id: i32,
    pub category_name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = subject_enrollments)]
#[diesel(primary_key(student_id, subject_id, academic_year_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SubjectEnrollment {
    pub student_id: String,
    pub subject_id: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = sessions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub refresh_token_hash: String,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
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
#[diesel(table_name = detention_balances)]
#[diesel(primary_key(student_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct DetentionBalance {
    pub student_id: String,
    pub total_hours_assigned: f32,
    pub total_hours_served: f32,
    pub remaining_hours: f32,
    pub updated_at: NaiveDateTime,
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
#[diesel(table_name = activity_attendance)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ActivityAttendance {
    pub id: String,
    pub activity_id: String,
    pub user_id: String,
    pub status: ActivityAttendanceStatus,
    pub check_in_time: Option<NaiveDateTime>,
    pub check_out_time: Option<NaiveDateTime>,
    pub remarks: Option<String>,
    pub marked_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = activity_participants)]
#[diesel(primary_key(activity_id, user_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ActivityParticipant {
    pub activity_id: String,
    pub user_id: String,
    pub participant_type: ParticipantType,
    pub enrollment_reason: Option<String>,
    pub created_at: NaiveDateTime,
}

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
#[diesel(table_name = school_settings)]
#[diesel(primary_key(setting_key))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SchoolSetting {
    pub setting_key: String,
    pub setting_value: String,
    pub description: Option<String>,
    pub updated_at: NaiveDateTime,
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
#[diesel(table_name = student_fees)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentFee {
    pub id: String,
    pub student_id: String,
    pub fee_structure_id: String,
    pub amount: f32,
    pub is_exempted: bool,
    pub exemption_reason: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = student_marks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentMark {
    pub id: String,
    pub student_id: String,
    pub exam_id: String,
    pub subject_id: String,
    pub marks_obtained: i32,
    pub is_absent: bool,
    pub remarks: Option<String>,
    pub entered_by: String,
    pub entered_at: NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = curriculum_standards)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct CurriculumStandard {
    pub id: String,
    pub subject_id: String,
    pub grade_level_id: String,
    pub standard_code: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub medium: Medium,
    pub version_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = syllabus)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Syllabus {
    pub id: String,
    pub curriculum_standard_id: String,
    pub topic_name: String,
    pub suggested_duration_hours: Option<i32>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub parent_id: Option<String>,
    pub is_practical: bool,
    pub required_periods: i32,
    pub buffer_periods: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = lesson_progress_periods)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(lesson_progress_id, timetable_id, date))]
pub struct LessonProgressPeriod {
    pub lesson_progress_id: String,
    pub timetable_id: String,
    pub date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = student_missed_lessons)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentMissedLesson {
    pub id: String,
    pub student_id: String,
    pub lesson_progress_id: String,
    pub status: MissedLessonStatus,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub notified_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = lesson_progress_attachments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LessonProgressAttachment {
    pub id: String,
    pub lesson_progress_id: String,
    pub file_name: String,
    pub file_url: String,
    pub file_type: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = syllabus_unit_allocations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SyllabusUnitAllocation {
    pub id: String,
    pub class_id: String,
    pub syllabus_id: String,
    pub planned_periods: i32,
    pub buffer_periods: i32,
    pub target_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations, ApiComponent)]
#[diesel(table_name = teacher_period_attendance)]
#[diesel(belongs_to(Staff, foreign_key = teacher_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = substitution_plans)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SubstitutionPlan {
    pub id: String,
    pub subject_id: String,
    pub medium: Medium,
    pub plan_name: String,
    pub content_link: Option<String>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = teacher_reward_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeacherRewardHistory {
    pub id: String,
    pub teacher_id: String,
    pub points: i32,
    pub reason_type: RewardReasonType,
    pub reference_id: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = teacher_reward_balances)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeacherRewardBalance {
    pub teacher_id: String,
    pub total_points: i32,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = lesson_reviews)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LessonReview {
    pub id: String,
    pub lesson_progress_id: String,
    pub reviewer_type: ReviewerType,
    pub reviewer_id: String,
    pub clarity_rating: i32,
    pub feedback_text: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = lesson_materials)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LessonMaterial {
    pub id: String,
    pub lesson_progress_id: String,
    pub uploader_id: String,
    pub file_name: String,
    pub file_url: String,
    pub file_type: LessonMaterialType,
    pub is_processed_by_ai: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = ai_processed_notes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AiProcessedNote {
    pub id: String,
    pub material_id: String,
    pub structured_json: String,
    pub summary: Option<String>,
    pub key_takeaways: Option<String>,
    pub suggested_questions: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = practical_lesson_appeals)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct PracticalLessonAppeal {
    pub id: String,
    pub lesson_progress_id: String,
    pub appeal_reason: String,
    pub evidence_image_url: Option<String>,
    pub status: AppealStatus,
    pub reviewed_by: Option<String>,
    pub reviewed_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = seeds)]
pub struct Seed {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = library_settings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LibrarySettings {
    pub id: i32,
    pub max_books_per_student: i32,
    pub max_books_per_staff: i32,
    pub issue_duration_days_student: i32,
    pub issue_duration_days_staff: i32,
    pub fine_per_day: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = substitutions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Substitution {
    pub id: String,
    pub original_teacher_id: String,
    pub substitute_teacher_id: String,
    pub timetable_id: String,
    pub date: NaiveDate,
    pub status: String, 
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
}
