use crate::models::staff::staff::Staff;
use crate::schema::{teacher_class_assignments, teacher_subject_assignments};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::database::enums::Medium;

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
#[diesel(table_name = teacher_class_assignments)]
#[diesel(belongs_to(Staff, foreign_key = teacher_id))]
pub struct TeacherClassAssignment {
    pub id: String,
    pub teacher_id: String,
    pub class_id: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = teacher_class_assignments)]
pub struct CreateTeacherClassAssignmentRequest {
    pub teacher_id: String,
    pub class_id: String,
    pub academic_year_id: String,
}

impl From<CreateTeacherClassAssignmentRequest> for TeacherClassAssignment {
    fn from(req: CreateTeacherClassAssignmentRequest) -> Self {
        TeacherClassAssignment {
            id: uuid::Uuid::new_v4().to_string(),
            teacher_id: req.teacher_id,
            class_id: req.class_id,
            academic_year_id: req.academic_year_id,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
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
#[diesel(table_name = teacher_subject_assignments)]
#[diesel(belongs_to(Staff, foreign_key = teacher_id))]
pub struct TeacherSubjectAssignment {
    pub id: String,
    pub teacher_id: String,
    pub subject_id: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub medium: Medium,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = teacher_subject_assignments)]
pub struct CreateTeacherSubjectAssignmentRequest {
    pub teacher_id: String,
    pub subject_id: String,
    pub academic_year_id: String,
    pub medium: Medium,
}

impl From<CreateTeacherSubjectAssignmentRequest> for TeacherSubjectAssignment {
    fn from(req: CreateTeacherSubjectAssignmentRequest) -> Self {
        TeacherSubjectAssignment {
            id: uuid::Uuid::new_v4().to_string(),
            teacher_id: req.teacher_id,
            subject_id: req.subject_id,
            academic_year_id: req.academic_year_id,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            medium: req.medium,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct AssignClassToTeacherRequest {
    pub class_id: String,
    pub academic_year_id: String,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct AssignSubjectToTeacherRequest {
    pub subject_id: String,
    pub academic_year_id: String,
    pub medium: Medium,
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
    pub medium: Medium,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct TeacherWorkloadResponse {
    pub teacher_id: String,
    pub total_classes_assigned: i64,
    pub total_subjects_assigned: i64,
}
