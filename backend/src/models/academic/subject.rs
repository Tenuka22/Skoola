use crate::schema::{subject_enrollments, subjects};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    JsonSchema,
    ApiComponent,
)]
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

#[derive(
    Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset, JsonSchema, ApiComponent,
)]
#[diesel(table_name = subjects)]
pub struct CreateSubjectRequest {
    pub id: String,
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

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct SubjectQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for SubjectQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AssignSubjectToGradeRequest {
    pub grade_id: String,
    pub subject_id: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AssignSubjectToStreamRequest {
    pub stream_id: String,
    pub subject_id: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct EnrollStudentInSubjectRequest {
    pub student_id: String,
    pub subject_id: String,
    pub academic_year_id: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SubjectEnrollmentResponse {
    pub student_id: String,
    pub subject_id: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = subject_enrollments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(student_id, subject_id, academic_year_id))]
pub struct SubjectEnrollment {
    pub student_id: String,
    pub subject_id: String,
    pub academic_year_id: String,
    pub created_at: NaiveDateTime,
}
