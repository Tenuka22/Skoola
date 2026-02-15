use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::NaiveDateTime;
use apistos::ApiComponent;
use crate::schema::subjects;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = subjects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Subject {
    pub id: String,
    pub subject_code: String,
    pub subject_name_en: String,
    pub subject_name_si: Option<String>,
    pub subject_name_ta: Option<String>,
    pub is_core: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = subjects)]
pub struct CreateSubjectRequest {
    pub subject_code: String,
    pub subject_name_en: String,
    pub subject_name_si: Option<String>,
    pub subject_name_ta: Option<String>,
    pub is_core: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = subjects)]
pub struct UpdateSubjectRequest {
    pub subject_code: Option<String>,
    pub subject_name_en: Option<String>,
    pub subject_name_si: Option<String>,
    pub subject_name_ta: Option<String>,
    pub is_core: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct AssignSubjectToGradeRequest {
    pub grade_id: String,
    pub subject_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct AssignSubjectToStreamRequest {
    pub stream_id: String,
    pub subject_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct EnrollStudentInSubjectRequest {
    pub student_id: String,
    pub subject_id: String,
    pub academic_year_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct SubjectEnrollmentResponse {
    pub student_id: String,
    pub subject_id: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct SubjectResponse {
    pub id: String,
    pub subject_code: String,
    pub subject_name_en: String,
    pub subject_name_si: Option<String>,
    pub subject_name_ta: Option<String>,
    pub is_core: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Subject> for SubjectResponse {
    fn from(subject: Subject) -> Self {
        SubjectResponse {
            id: subject.id,
            subject_code: subject.subject_code,
            subject_name_en: subject.subject_name_en,
            subject_name_si: subject.subject_name_si,
            subject_name_ta: subject.subject_name_ta,
            is_core: subject.is_core,
            created_at: subject.created_at,
            updated_at: subject.updated_at,
        }
    }
}
