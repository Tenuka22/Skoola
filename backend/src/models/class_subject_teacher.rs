use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::NaiveDateTime;
use apistos::ApiComponent;
use crate::schema::class_subject_teachers;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = class_subject_teachers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ClassSubjectTeacher {
    pub class_id: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = class_subject_teachers)]
pub struct CreateClassSubjectTeacherRequest {
    pub class_id: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub academic_year_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = class_subject_teachers)]
pub struct UpdateClassSubjectTeacherRequest {
    pub teacher_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct ClassSubjectTeacherResponse {
    pub class_id: String,
    pub subject_id: String,
    pub teacher_id: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ClassSubjectTeacher> for ClassSubjectTeacherResponse {
    fn from(assignment: ClassSubjectTeacher) -> Self {
        ClassSubjectTeacherResponse {
            class_id: assignment.class_id,
            subject_id: assignment.subject_id,
            teacher_id: assignment.teacher_id,
            academic_year_id: assignment.academic_year_id,
            created_at: assignment.created_at,
            updated_at: assignment.updated_at,
        }
    }
}
