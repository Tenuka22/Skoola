use crate::schema::student_emergency_contacts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema)]
#[diesel(table_name = student_emergency_contacts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentEmergencyContact {
    pub id: String,
    pub student_id: String,
    pub name: String,
    pub relationship: String,
    pub phone: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CreateStudentEmergencyContactRequest {
    pub student_id: String,
    pub name: String,
    pub relationship: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema)]
#[diesel(table_name = student_emergency_contacts)]
pub struct UpdateStudentEmergencyContactRequest {
    pub name: Option<String>,
    pub relationship: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct StudentEmergencyContactResponse {
    pub id: String,
    pub student_id: String,
    pub name: String,
    pub relationship: String,
    pub phone: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StudentEmergencyContact> for StudentEmergencyContactResponse {
    fn from(contact: StudentEmergencyContact) -> Self {
        StudentEmergencyContactResponse {
            id: contact.id,
            student_id: contact.student_id,
            name: contact.name,
            relationship: contact.relationship,
            phone: contact.phone,
            created_at: contact.created_at,
            updated_at: contact.updated_at,
        }
    }
}
