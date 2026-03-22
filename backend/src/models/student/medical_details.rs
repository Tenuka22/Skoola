use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::{student_allergies, student_medical_conditions, student_medications};

// Student Allergies
#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Clone,
)]
#[diesel(table_name = student_allergies)]
pub struct StudentAllergy {
    pub id: String,
    pub student_id: String,
    pub allergen_type: String,
    pub allergen_name: String,
    pub reaction_severity: String,
    pub reaction_description: Option<String>,
    pub requires_epipen: bool,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StudentAllergyQuery {
    pub search: Option<String>,
    pub student_id: Option<String>,
    pub allergen_type: Option<String>,
    pub reaction_severity: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StudentAllergyQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CreateStudentAllergyRequest {
    pub student_id: String,
    pub allergen_type: String,
    pub allergen_name: String,
    pub reaction_severity: String,
    pub reaction_description: Option<String>,
    pub requires_epipen: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = student_allergies)]
pub struct UpdateStudentAllergyRequest {
    pub allergen_type: Option<String>,
    pub allergen_name: Option<String>,
    pub reaction_severity: Option<String>,
    pub reaction_description: Option<String>,
    pub requires_epipen: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StudentAllergyResponse {
    pub id: String,
    pub student_id: String,
    pub allergen_type: String,
    pub allergen_name: String,
    pub reaction_severity: String,
    pub reaction_description: Option<String>,
    pub requires_epipen: bool,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StudentAllergy> for StudentAllergyResponse {
    fn from(allergy: StudentAllergy) -> Self {
        Self {
            id: allergy.id,
            student_id: allergy.student_id,
            allergen_type: allergy.allergen_type,
            allergen_name: allergy.allergen_name,
            reaction_severity: allergy.reaction_severity,
            reaction_description: allergy.reaction_description,
            requires_epipen: allergy.requires_epipen,
            notes: allergy.notes,
            created_at: allergy.created_at,
            updated_at: allergy.updated_at,
        }
    }
}

// Student Medical Conditions
#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Clone,
)]
#[diesel(table_name = student_medical_conditions)]
pub struct StudentMedicalCondition {
    pub id: String,
    pub student_id: String,
    pub condition_type: String,
    pub condition_name: String,
    pub severity: String,
    pub diagnosis_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StudentMedicalConditionQuery {
    pub search: Option<String>,
    pub student_id: Option<String>,
    pub condition_type: Option<String>,
    pub severity: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StudentMedicalConditionQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CreateStudentMedicalConditionRequest {
    pub student_id: String,
    pub condition_type: String,
    pub condition_name: String,
    pub severity: String,
    pub diagnosis_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = student_medical_conditions)]
pub struct UpdateStudentMedicalConditionRequest {
    pub condition_type: Option<String>,
    pub condition_name: Option<String>,
    pub severity: Option<String>,
    pub diagnosis_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StudentMedicalConditionResponse {
    pub id: String,
    pub student_id: String,
    pub condition_type: String,
    pub condition_name: String,
    pub severity: String,
    pub diagnosis_date: Option<NaiveDate>,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StudentMedicalCondition> for StudentMedicalConditionResponse {
    fn from(condition: StudentMedicalCondition) -> Self {
        Self {
            id: condition.id,
            student_id: condition.student_id,
            condition_type: condition.condition_type,
            condition_name: condition.condition_name,
            severity: condition.severity,
            diagnosis_date: condition.diagnosis_date,
            notes: condition.notes,
            created_at: condition.created_at,
            updated_at: condition.updated_at,
        }
    }
}

// Student Medications
#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Clone,
)]
#[diesel(table_name = student_medications)]
pub struct StudentMedication {
    pub id: String,
    pub student_id: String,
    pub medication_name: String,
    pub dosage: Option<String>,
    pub frequency: Option<String>,
    pub is_emergency_med: bool,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StudentMedicationQuery {
    pub search: Option<String>,
    pub student_id: Option<String>,
    pub is_emergency_med: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StudentMedicationQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CreateStudentMedicationRequest {
    pub student_id: String,
    pub medication_name: String,
    pub dosage: Option<String>,
    pub frequency: Option<String>,
    pub is_emergency_med: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = student_medications)]
pub struct UpdateStudentMedicationRequest {
    pub medication_name: Option<String>,
    pub dosage: Option<String>,
    pub frequency: Option<String>,
    pub is_emergency_med: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StudentMedicationResponse {
    pub id: String,
    pub student_id: String,
    pub medication_name: String,
    pub dosage: Option<String>,
    pub frequency: Option<String>,
    pub is_emergency_med: bool,
    pub notes: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StudentMedication> for StudentMedicationResponse {
    fn from(medication: StudentMedication) -> Self {
        Self {
            id: medication.id,
            student_id: medication.student_id,
            medication_name: medication.medication_name,
            dosage: medication.dosage,
            frequency: medication.frequency,
            is_emergency_med: medication.is_emergency_med,
            notes: medication.notes,
            created_at: medication.created_at,
            updated_at: medication.updated_at,
        }
    }
}

impl From<CreateStudentAllergyRequest> for StudentAllergy {
    fn from(req: CreateStudentAllergyRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            student_id: req.student_id,
            allergen_type: req.allergen_type,
            allergen_name: req.allergen_name,
            reaction_severity: req.reaction_severity,
            reaction_description: req.reaction_description,
            requires_epipen: req.requires_epipen,
            notes: req.notes,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<CreateStudentMedicalConditionRequest> for StudentMedicalCondition {
    fn from(req: CreateStudentMedicalConditionRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            student_id: req.student_id,
            condition_type: req.condition_type,
            condition_name: req.condition_name,
            severity: req.severity,
            diagnosis_date: req.diagnosis_date,
            notes: req.notes,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<CreateStudentMedicationRequest> for StudentMedication {
    fn from(req: CreateStudentMedicationRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            student_id: req.student_id,
            medication_name: req.medication_name,
            dosage: req.dosage,
            frequency: req.frequency,
            is_emergency_med: req.is_emergency_med,
            notes: req.notes,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
