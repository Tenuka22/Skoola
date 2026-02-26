use crate::models::staff::staff::Staff;
use crate::schema::{teacher_class_assignments, teacher_subject_assignments};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
#[diesel(table_name = teacher_class_assignments)]
#[diesel(belongs_to(Staff, foreign_key = teacher_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeacherClassAssignment {
    pub id: String,
    pub teacher_id: String,
    pub class_id: String,
    pub academic_year_id: String,
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
#[diesel(table_name = teacher_subject_assignments)]
#[diesel(belongs_to(Staff, foreign_key = teacher_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct TeacherSubjectAssignment {
    pub id: String,
    pub teacher_id: String,
    pub subject_id: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
