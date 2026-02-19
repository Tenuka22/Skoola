use crate::schema::student_medical_info;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema)]
#[diesel(table_name = student_medical_info)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentMedicalInfo {
    pub id: String,
    pub student_id: String,
    pub blood_group: Option<String>,
    pub allergies: Option<String>,
    pub medical_conditions: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CreateStudentMedicalInfoRequest {
    pub student_id: String,
    pub blood_group: Option<String>,
    pub allergies: Option<String>,
    pub medical_conditions: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema)]
#[diesel(table_name = student_medical_info)]
pub struct UpdateStudentMedicalInfoRequest {
    pub blood_group: Option<String>,
    pub allergies: Option<String>,
    pub medical_conditions: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct StudentMedicalInfoResponse {
    pub id: String,
    pub student_id: String,
    pub blood_group: Option<String>,
    pub allergies: Option<String>,
    pub medical_conditions: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StudentMedicalInfo> for StudentMedicalInfoResponse {
    fn from(info: StudentMedicalInfo) -> Self {
        StudentMedicalInfoResponse {
            id: info.id,
            student_id: info.student_id,
            blood_group: info.blood_group,
            allergies: info.allergies,
            medical_conditions: info.medical_conditions,
            emergency_contact_name: info.emergency_contact_name,
            emergency_contact_phone: info.emergency_contact_phone,
            created_at: info.created_at,
            updated_at: info.updated_at,
        }
    }
}
