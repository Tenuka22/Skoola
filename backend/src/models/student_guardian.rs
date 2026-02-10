use crate::schema::student_guardians;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::NaiveDateTime;
use apistos::ApiComponent;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema)]
#[diesel(table_name = student_guardians)]
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
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct CreateStudentGuardianRequest {
    pub student_id: String,
    pub name: String,
    pub relationship: String,
    pub phone: String,
    pub email: Option<String>,
    pub address: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = student_guardians)]
pub struct UpdateStudentGuardianRequest {
    pub name: Option<String>,
    pub relationship: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct StudentGuardianResponse {
    pub id: String,
    pub student_id: String,
    pub name: String,
    pub relationship: String,
    pub phone: String,
    pub email: Option<String>,
    pub address: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StudentGuardian> for StudentGuardianResponse {
    fn from(guardian: StudentGuardian) -> Self {
        StudentGuardianResponse {
            id: guardian.id,
            student_id: guardian.student_id,
            name: guardian.name,
            relationship: guardian.relationship,
            phone: guardian.phone,
            email: guardian.email,
            address: guardian.address,
            created_at: guardian.created_at,
            updated_at: guardian.updated_at,
        }
    }
}
