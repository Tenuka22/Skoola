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

impl From<CreateStaffCvRequest> for StaffCv {
    fn from(req: CreateStaffCvRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            staff_id: req.staff_id,
            file_name: req.file_name,
            file_url: req.file_url,
            file_type: req.file_type,
            uploaded_at: now,
            created_at: now,
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

impl From<CreateStaffDocumentRequest> for StaffDocument {
    fn from(req: CreateStaffDocumentRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            staff_id: req.staff_id,
            doc_type: req.doc_type,
            file_url: req.file_url,
            issued_date: req.issued_date,
            expiry_date: req.expiry_date,
            created_at: now,
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

impl From<CreateStaffNoteRequest> for StaffNote {
    fn from(req: CreateStaffNoteRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            staff_id: req.staff_id,
            note_type: req.note_type,
            note_text: req.note_text,
            created_by: req.created_by,
            created_at: now,
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

impl From<CreateStaffOvertimeRequest> for StaffOvertime {
    fn from(req: CreateStaffOvertimeRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            staff_id: req.staff_id,
            date: req.date,
            hours: req.hours,
            reason: req.reason,
            approved_by: req.approved_by,
            reward_points: req.reward_points,
            is_paid: req.is_paid,
            created_at: now,
            updated_at: now,
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

impl From<CreateStaffSkillRequest> for StaffSkill {
    fn from(req: CreateStaffSkillRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            staff_id: req.staff_id,
            skill_name: req.skill_name,
            proficiency_level: req.proficiency_level,
            notes: req.notes,
            created_at: now,
            updated_at: now,
        }
    }
}
