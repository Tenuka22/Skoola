use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct AssignClassToTeacherRequest {
    pub class_id: String,
    pub academic_year_id: String,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct AssignSubjectToTeacherRequest {
    pub subject_id: String,
    pub academic_year_id: String,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct TeacherClassAssignmentResponse {
    pub id: String,
    pub teacher_id: String,
    pub class_id: String,
    pub academic_year_id: String,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct TeacherSubjectAssignmentResponse {
    pub id: String,
    pub teacher_id: String,
    pub subject_id: String,
    pub academic_year_id: String,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct TeacherWorkloadResponse {
    pub teacher_id: String,
    pub total_classes_assigned: i64,
    pub total_subjects_assigned: i64,
}
