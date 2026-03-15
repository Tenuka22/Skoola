use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::{
    activity_participants_staff, activity_participants_students, behavior_incident_participants,
    exit_passes_bulk, reward_adjustments, reward_types, role_set_roles, sport_events,
    staff_event_participants, staff_leave_balances, staff_subject_expertise, staff_subjects,
    student_demographics, teacher_reward_balances, teacher_reward_details, teacher_reward_history,
    user_security, user_status,
};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = activity_participants_staff)]
#[diesel(primary_key(activity_id, staff_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ActivityParticipantStaff {
    pub activity_id: String,
    pub staff_id: String,
    pub participant_type: String,
    pub enrollment_reason: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = activity_participants_students)]
#[diesel(primary_key(activity_id, student_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ActivityParticipantStudent {
    pub activity_id: String,
    pub student_id: String,
    pub participant_type: String,
    pub enrollment_reason: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = behavior_incident_participants)]
#[diesel(primary_key(incident_id, participant_type, participant_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct BehaviorIncidentParticipant {
    pub incident_id: String,
    pub participant_type: String,
    pub participant_id: String,
    pub role: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = exit_passes_bulk)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ExitPassBulk {
    pub id: String,
    pub target_type: String,
    pub target_id: String,
    pub start_time: NaiveDateTime,
    pub end_time: Option<NaiveDateTime>,
    pub reason: Option<String>,
    pub issued_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = reward_adjustments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RewardAdjustment {
    pub id: String,
    pub teacher_id: String,
    pub adjustment_points: i32,
    pub reason: Option<String>,
    pub approved_by: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = reward_types)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RewardType {
    pub id: String,
    pub name: String,
    pub category: String,
    pub default_points: i32,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = role_set_roles)]
#[diesel(primary_key(role_set_id, role_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct RoleSetRole {
    pub role_set_id: String,
    pub role_id: String,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = sport_events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SportEvent {
    pub id: String,
    pub sport_id: String,
    pub event_name: String,
    pub event_date: NaiveDateTime,
    pub venue: String,
    pub organizer: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = staff_event_participants)]
#[diesel(primary_key(event_id, staff_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffEventParticipant {
    pub event_id: String,
    pub staff_id: String,
    pub participation_status: String,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = staff_leave_balances)]
#[diesel(primary_key(staff_id, leave_type_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffLeaveBalance {
    pub staff_id: String,
    pub leave_type_id: String,
    pub balance_days: f32,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = staff_subject_expertise)]
#[diesel(primary_key(staff_id, subject_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffSubjectExpertise {
    pub staff_id: String,
    pub subject_id: String,
    pub expertise_level: String,
    pub years_experience: Option<i32>,
    pub evidence: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = staff_subjects)]
#[diesel(primary_key(staff_id, subject_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffSubject {
    pub staff_id: String,
    pub subject_id: String,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = student_demographics)]
#[diesel(primary_key(student_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentDemographic {
    pub student_id: String,
    pub religion: Option<String>,
    pub ethnicity: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
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

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = teacher_reward_details)]
#[diesel(primary_key(reward_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeacherRewardDetail {
    pub reward_id: String,
    pub reason_type: String,
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

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = teacher_reward_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeacherRewardHistory {
    pub id: String,
    pub teacher_id: String,
    pub points: i32,
    pub created_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = user_security)]
#[diesel(primary_key(user_id))]
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

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = user_status)]
#[diesel(primary_key(user_id))]
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

