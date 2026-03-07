use crate::database::enums::*;
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
    pub education_level: EducationLevel,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = al_streams)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AlStream {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
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
    pub academic_year_id: String,
    pub class_teacher_id: Option<String>,
    pub medium: Medium,
    pub room_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = school_rooms)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SchoolRoom {
    pub id: String,
    pub name: Option<String>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = al_stream_grade_levels)]
#[diesel(primary_key(stream_id, grade_level_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AlStreamGradeLevel {
    pub stream_id: String,
    pub grade_level_id: String,
    pub created_at: NaiveDateTime,
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
#[diesel(table_name = al_stream_required_subjects)]
#[diesel(primary_key(stream_id, subject_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AlStreamRequiredSubject {
    pub stream_id: String,
    pub subject_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = al_stream_optional_groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AlStreamOptionalGroup {
    pub id: String,
    pub stream_id: String,
    pub group_name: String,
    pub min_select: i32,
    pub max_select: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = al_stream_optional_subjects)]
#[diesel(primary_key(group_id, subject_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AlStreamOptionalSubject {
    pub group_id: String,
    pub subject_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = timetable)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Timetable {
    pub id: String,
    pub class_id: String,
    pub day_of_week: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub room: String,
    pub academic_year_id: String,
    pub grade_period_id: Option<String>,
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub role: RoleEnum,
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub role: RoleEnum,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = user_security)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserSecurity {
    pub user_id: String,
    pub google_id: Option<String>,
    pub github_id: Option<String>,
    pub verification_token: Option<String>,
    pub verification_sent_at: Option<NaiveDateTime>,
    pub password_reset_token: Option<String>,
    pub password_reset_sent_at: Option<NaiveDateTime>,
    pub failed_login_attempts: i32,
    pub lockout_until: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = user_status)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserStatus {
    pub user_id: String,
    pub is_verified: bool,
    pub is_active: bool,
    pub disabled_at: Option<NaiveDateTime>,
    pub disabled_reason: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = auth_tokens)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AuthToken {
    pub id: String,
    pub user_id: String,
    pub token_hash: String,
    pub token_type: AuthTokenType,
    pub issued_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub revoked_at: Option<NaiveDateTime>,
    pub is_active: bool,
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = verification_tokens)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct VerificationToken {
    pub id: String,
    pub user_id: String,
    pub token_hash: String,
    pub purpose: VerificationPurpose,
    pub issued_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub consumed_at: Option<NaiveDateTime>,
    pub is_active: bool,
    pub metadata: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = profiles)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = profile_contacts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ProfileContact {
    pub profile_id: String,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = profile_media)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ProfileMedia {
    pub profile_id: String,
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
    pub dob: NaiveDate,
    pub gender: Gender,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub staff_type: StaffType,
    pub profile_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = staff_identity)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffIdentity {
    pub staff_id: String,
    pub nic: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = staff_contacts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffContact {
    pub staff_id: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub address_latitude: Option<f32>,
    pub address_longitude: Option<f32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = staff_employment_status)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffEmploymentStatus {
    pub staff_id: String,
    pub employment_status: EmploymentStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = staff_media)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffMedia {
    pub staff_id: String,
    pub photo_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = staff_reward_snapshots)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffRewardSnapshot {
    pub staff_id: String,
    pub reward_points_balance: i32,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, Associations, ApiComponent)]
#[diesel(table_name = staff_leaves)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffLeave {
    pub id: String,
    pub staff_id: String,
    pub leave_type: StaffLeaveType,
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = students)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Student {
    pub id: String,
    pub admission_number: String,
    pub name_english: String,
    pub name_sinhala: Option<String>,
    pub name_tamil: Option<String>,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub profile_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = student_contacts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentContact {
    pub student_id: String,
    pub address: String,
    pub address_latitude: Option<f32>,
    pub address_longitude: Option<f32>,
    pub phone: String,
    pub email: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = student_demographics)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentDemographics {
    pub student_id: String,
    pub religion: Option<Religion>,
    pub ethnicity: Option<Ethnicity>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = student_status)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentStatusRow {
    pub student_id: String,
    pub status: StudentStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = student_media)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentMedia {
    pub student_id: String,
    pub photo_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
    pub curriculum_topic_id: Option<String>,
    pub date: NaiveDate,
    pub lesson_summary: String,
    pub homework_assigned: Option<String>,
    pub resources_used: Option<String>,
    pub progress_percentage: Option<i32>,
    pub delivery_mode: LessonDeliveryMode,
    pub planned_duration_minutes: Option<i32>,
    pub actual_duration_minutes: Option<i32>,
    pub created_at: NaiveDateTime,
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
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = fee_structure_pricing)]
#[diesel(primary_key(fee_structure_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FeeStructurePricing {
    pub fee_structure_id: String,
    pub amount: f32,
    pub currency: String,
    pub amount_type: FeeAmountType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = fee_structure_schedule)]
#[diesel(primary_key(fee_structure_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FeeStructureSchedule {
    pub fee_structure_id: String,
    pub due_date: Option<NaiveDate>,
    pub frequency: FeeFrequency,
    pub fee_type: FeeTypeEnum,
    pub effective_from: Option<NaiveDate>,
    pub effective_to: Option<NaiveDate>,
    pub due_day_of_month: Option<i32>,
    pub is_refundable: bool,
    pub late_fee_type: Option<LateFeeTypeEnum>,
    pub late_fee_value: Option<f32>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
#[diesel(table_name = fee_payments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FeePayment {
    pub id: String,
    pub student_fee_id: String,
    pub amount_paid: f32,
    pub payment_date: NaiveDateTime,
    pub collected_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = fee_payment_details)]
#[diesel(primary_key(payment_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct FeePaymentDetail {
    pub payment_id: String,
    pub payment_method: PaymentMethod,
    pub payment_channel: Option<String>,
    pub payment_status: PaymentStatusType,
    pub receipt_number: String,
    pub transaction_reference: Option<String>,
    pub remarks: Option<String>,
    pub recorded_by: Option<String>,
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
#[diesel(table_name = library_issues)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LibraryIssue {
    pub id: i32,
    pub book_id: i32,
    pub student_id: Option<String>,
    pub staff_id: Option<String>,
    pub issue_date: NaiveDate,
    pub due_date: NaiveDate,
    pub return_date: Option<NaiveDate>,
    pub issued_by: String,
    pub fine_amount: Option<f32>,
    pub fine_paid: bool,
    pub status: LibraryIssueStatus,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
    pub auth_token_id: Option<String>,
    pub verification_token_id: Option<String>,
    pub user_agent: Option<String>,
    pub ip_address: Option<String>,
    pub created_at: NaiveDateTime,
    pub expires_at: NaiveDateTime,
    pub is_active: bool,
    pub disabled_at: Option<NaiveDateTime>,
    pub disabled_reason: Option<String>,
    pub last_seen_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = crate::schema::attendance_policies)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AttendancePolicy {
    pub id: String,
    pub name: String,
    pub rule_type: PolicyRuleType,
    pub threshold: i32,
    pub consequence_type: ConsequenceType,
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
    pub bulk_pass_id: Option<String>,
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
    pub status: String,
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
    pub participant_type: String,
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
    pub stream_id: Option<String>,
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
#[diesel(table_name = teacher_reward_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeacherRewardHistory {
    pub id: String,
    pub teacher_id: String,
    pub points: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = teacher_reward_balances)]
#[diesel(primary_key(teacher_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeacherRewardBalance {
    pub teacher_id: String,
    pub total_points: i32,
    pub updated_at: NaiveDateTime,
    pub lifetime_points: i32,
    pub last_updated: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = teacher_reward_details)]
#[diesel(primary_key(reward_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeacherRewardDetail {
    pub reward_id: String,
    pub reason_type: RewardReasonType,
    pub reference_id: Option<String>,
    pub reward_type_id: Option<String>,
    pub awarded_by: Option<String>,
    pub status: String,
    pub effective_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub reference_type: Option<String>,
    pub balance_after: Option<i32>,
    pub created_at: NaiveDateTime,
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
#[diesel(table_name = crate::schema::seeds)]
pub struct Seed {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub created_at: NaiveDateTime,
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
#[diesel(table_name = staff_departments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffDepartment {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = staff_qualifications)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffQualification {
    pub id: String,
    pub staff_id: String,
    pub degree: String,
    pub institution: String,
    pub year_of_completion: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub file_name: Option<String>,
    pub file_url: Option<String>,
    pub file_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = staff_employment_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffEmploymentHistory {
    pub id: String,
    pub staff_id: String,
    pub previous_school: String,
    pub position: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub reason_for_leaving: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub workplace_address: Option<String>,
    pub workplace_contact_number: Option<String>,
    pub workplace_email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = student_previous_schools)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentPreviousSchool {
    pub id: String,
    pub student_id: String,
    pub school_name: String,
    pub grade_left: Option<String>,
    pub date_left: Option<NaiveDate>,
    pub reason_for_leaving: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = student_class_assignments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentClassAssignment {
    pub id: String,
    pub student_id: String,
    pub academic_year_id: String,
    pub grade_id: String,
    pub class_id: String,
    pub from_date: NaiveDate,
    pub to_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = student_class_assignments_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentClassAssignmentHistory {
    pub id: String,
    pub student_id: String,
    pub academic_year_id: String,
    pub grade_id: String,
    pub class_id: String,
    pub from_date: NaiveDate,
    pub to_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = crate::schema::student_guardians)]
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
#[diesel(table_name = resources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Resource {
    pub id: String,
    pub resource_name: String,
    pub resource_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = conversations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Conversation {
    pub id: String,
    pub subject: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = messages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub sender_user_id: String,
    pub content: String,
    pub sent_at: NaiveDateTime,
    pub read_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = student_allergies)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentAllergy {
    pub id: String,
    pub student_id: String,
    pub allergen_type: String,
    pub allergen_name: String,
    pub reaction_severity: String,
    pub reaction_description: Option<String>,
    pub requires_epipen: bool,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = student_medical_conditions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentMedicalCondition {
    pub id: String,
    pub student_id: String,
    pub condition_type: String,
    pub condition_name: String,
    pub severity: String,
    pub diagnosis_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = student_medications)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentMedication {
    pub id: String,
    pub student_id: String,
    pub medication_name: String,
    pub dosage: Option<String>,
    pub frequency: Option<String>,
    pub is_emergency_med: bool,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = staff_contracts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffContract {
    pub id: String,
    pub staff_id: String,
    pub contract_type: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub salary_amount: Option<f32>,
    pub currency: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
