use crate::models::staff::staff::Staff;
use crate::schema::{staff_employment_history, teacher_teaching_history};
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
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
#[diesel(table_name = staff_employment_history)]
#[diesel(belongs_to(Staff))]
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

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = staff_employment_history)]
pub struct CreateStaffEmploymentHistoryRequest {
    pub staff_id: String,
    pub previous_school: String,
    pub position: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub reason_for_leaving: Option<String>,
}

impl From<CreateStaffEmploymentHistoryRequest> for StaffEmploymentHistory {
    fn from(req: CreateStaffEmploymentHistoryRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            staff_id: req.staff_id,
            previous_school: req.previous_school,
            position: req.position,
            start_date: req.start_date,
            end_date: req.end_date,
            reason_for_leaving: req.reason_for_leaving,
            created_at: now,
            updated_at: now,
            workplace_address: None,
            workplace_contact_number: None,
            workplace_email: None,
        }
    }
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
#[diesel(table_name = teacher_teaching_history)]
#[diesel(belongs_to(Staff, foreign_key = staff_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeacherTeachingHistory {
    pub id: String,
    pub staff_id: String,
    pub school_name: String,
    pub subject_id: Option<String>,
    pub grade_level_id: Option<String>,
    pub role_title: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = teacher_teaching_history)]
pub struct CreateTeacherTeachingHistoryRequest {
    pub staff_id: String,
    pub school_name: String,
    pub subject_id: Option<String>,
    pub grade_level_id: Option<String>,
    pub role_title: Option<String>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

impl From<CreateTeacherTeachingHistoryRequest> for TeacherTeachingHistory {
    fn from(req: CreateTeacherTeachingHistoryRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            staff_id: req.staff_id,
            school_name: req.school_name,
            subject_id: req.subject_id,
            grade_level_id: req.grade_level_id,
            role_title: req.role_title,
            start_date: req.start_date,
            end_date: req.end_date,
            notes: req.notes,
            created_at: now,
            updated_at: now,
        }
    }
}
