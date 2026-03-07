use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::staff::staff::Staff;

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
#[diesel(table_name = crate::schema::staff_cvs)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffCv {
    pub id: String,
    pub staff_id: String,
    pub file_name: String,
    pub file_url: String,
    pub file_type: String,
    pub uploaded_at: NaiveDateTime,
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
    Associations,
    ApiComponent,
)]
#[diesel(table_name = crate::schema::staff_documents)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffDocument {
    pub id: String,
    pub staff_id: String,
    pub doc_type: String,
    pub file_url: String,
    pub issued_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
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
    Associations,
    ApiComponent,
)]
#[diesel(table_name = crate::schema::staff_notes)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffNote {
    pub id: String,
    pub staff_id: String,
    pub note_type: String,
    pub note_text: String,
    pub created_by: Option<String>,
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
    Associations,
    ApiComponent,
)]
#[diesel(table_name = crate::schema::staff_overtime)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffOvertime {
    pub id: String,
    pub staff_id: String,
    pub date: NaiveDate,
    pub hours: f32,
    pub reason: Option<String>,
    pub approved_by: Option<String>,
    pub reward_points: i32,
    pub is_paid: bool,
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
#[diesel(table_name = crate::schema::staff_skills)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffSkill {
    pub id: String,
    pub staff_id: String,
    pub skill_name: String,
    pub proficiency_level: String,
    pub notes: Option<String>,
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
#[diesel(table_name = crate::schema::staff_subject_expertise)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(staff_id, subject_id))]
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
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    Clone,
    Associations,
    ApiComponent,
)]
#[diesel(table_name = crate::schema::staff_event_participants)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(event_id, staff_id))]
pub struct StaffEventParticipant {
    pub event_id: String,
    pub staff_id: String,
    pub participation_status: String,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
}
