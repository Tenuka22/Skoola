use crate::schema::student_medical_info;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema,
)]
#[diesel(table_name = student_medical_info)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentMedicalInfo {
    pub id: String,
    pub student_id: String,
    pub blood_group: Option<String>,
    pub medical_risk_level: Option<String>,
    pub has_allergies: bool,
    pub has_medications: bool,
    pub has_chronic_conditions: bool,
    pub requires_emergency_plan: bool,
    pub emergency_plan_details: Option<String>,
    pub allergies: Option<String>,
    pub medical_conditions: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub primary_physician_name: Option<String>,
    pub primary_physician_phone: Option<String>,
    pub insurance_provider: Option<String>,
    pub insurance_policy_number: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CreateStudentMedicalInfoRequest {
    pub student_id: String,
    pub blood_group: Option<String>,
    pub medical_risk_level: Option<String>,
    pub has_allergies: bool,
    pub has_medications: bool,
    pub has_chronic_conditions: bool,
    pub requires_emergency_plan: bool,
    pub emergency_plan_details: Option<String>,
    pub allergies: Option<String>,
    pub medical_conditions: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub primary_physician_name: Option<String>,
    pub primary_physician_phone: Option<String>,
    pub insurance_provider: Option<String>,
    pub insurance_policy_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema)]
#[diesel(table_name = student_medical_info)]
pub struct UpdateStudentMedicalInfoRequest {
    pub blood_group: Option<String>,
    pub medical_risk_level: Option<String>,
    pub has_allergies: bool,
    pub has_medications: bool,
    pub has_chronic_conditions: bool,
    pub requires_emergency_plan: bool,
    pub emergency_plan_details: Option<String>,
    pub allergies: Option<String>,
    pub medical_conditions: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub primary_physician_name: Option<String>,
    pub primary_physician_phone: Option<String>,
    pub insurance_provider: Option<String>,
    pub insurance_policy_number: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct StudentMedicalInfoResponse {
    pub id: String,
    pub student_id: String,
    pub blood_group: Option<String>,
    pub medical_risk_level: Option<String>,
    pub has_allergies: bool,
    pub has_medications: bool,
    pub has_chronic_conditions: bool,
    pub requires_emergency_plan: bool,
    pub emergency_plan_details: Option<String>,
    pub allergies: Option<String>,
    pub medical_conditions: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub primary_physician_name: Option<String>,
    pub primary_physician_phone: Option<String>,
    pub insurance_provider: Option<String>,
    pub insurance_policy_number: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StudentMedicalInfo> for StudentMedicalInfoResponse {
    fn from(info: StudentMedicalInfo) -> Self {
        StudentMedicalInfoResponse {
            id: info.id,
            student_id: info.student_id,
            blood_group: info.blood_group,
            medical_risk_level: info.medical_risk_level,
            has_allergies: info.has_allergies,
            has_medications: info.has_medications,
            has_chronic_conditions: info.has_chronic_conditions,
            requires_emergency_plan: info.requires_emergency_plan,
            emergency_plan_details: info.emergency_plan_details,
            allergies: info.allergies,
            medical_conditions: info.medical_conditions,
            emergency_contact_name: info.emergency_contact_name,
            emergency_contact_phone: info.emergency_contact_phone,
            primary_physician_name: info.primary_physician_name,
            primary_physician_phone: info.primary_physician_phone,
            insurance_provider: info.insurance_provider,
            insurance_policy_number: info.insurance_policy_number,
            created_at: info.created_at,
            updated_at: info.updated_at,
        }
    }
}
