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
    AsChangeset,
    Clone,
    Associations,
    ApiComponent,
)]
#[diesel(table_name = crate::schema::staff_cvs)]
#[diesel(belongs_to(Staff))]
pub struct StaffCv {
    pub id: String,
    pub staff_id: String,
    pub file_name: String,
    pub file_url: String,
    pub file_type: String,
    pub uploaded_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::staff_cvs)]
pub struct CreateStaffCvRequest {
    pub staff_id: String,
    pub file_name: String,
    pub file_url: String,
    pub file_type: String,
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
#[diesel(table_name = crate::schema::staff_documents)]
#[diesel(belongs_to(Staff))]
pub struct StaffDocument {
    pub id: String,
    pub staff_id: String,
    pub doc_type: String,
    pub file_url: String,
    pub issued_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::staff_documents)]
pub struct CreateStaffDocumentRequest {
    pub staff_id: String,
    pub doc_type: String,
    pub file_url: String,
    pub issued_date: Option<NaiveDate>,
    pub expiry_date: Option<NaiveDate>,
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
#[diesel(table_name = crate::schema::staff_notes)]
#[diesel(belongs_to(Staff))]
pub struct StaffNote {
    pub id: String,
    pub staff_id: String,
    pub note_type: String,
    pub note_text: String,
    pub created_by: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::staff_notes)]
pub struct CreateStaffNoteRequest {
    pub staff_id: String,
    pub note_type: String,
    pub note_text: String,
    pub created_by: Option<String>,
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
#[diesel(table_name = crate::schema::staff_overtime)]
#[diesel(belongs_to(Staff))]
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

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::staff_overtime)]
pub struct CreateStaffOvertimeRequest {
    pub staff_id: String,
    pub date: NaiveDate,
    pub hours: f32,
    pub reason: Option<String>,
    pub approved_by: Option<String>,
    pub reward_points: i32,
    pub is_paid: bool,
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
#[diesel(table_name = crate::schema::staff_skills)]
#[diesel(belongs_to(Staff))]
pub struct StaffSkill {
    pub id: String,
    pub staff_id: String,
    pub skill_name: String,
    pub proficiency_level: String,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::staff_skills)]
pub struct CreateStaffSkillRequest {
    pub staff_id: String,
    pub skill_name: String,
    pub proficiency_level: String,
    pub notes: Option<String>,
}
