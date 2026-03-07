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
    Associations,
    ApiComponent,
)]
#[diesel(table_name = teacher_teaching_history)]
#[diesel(belongs_to(Staff))]
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
