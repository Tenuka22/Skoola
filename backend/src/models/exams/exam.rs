use crate::schema::exams;
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
#[diesel(table_name = exams)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Exam {
    pub id: String,
    pub exam_type_id: String,
    pub name: String,
    pub academic_year_id: String,
    pub term_id: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset, JsonSchema, ApiComponent,
)]
#[diesel(table_name = exams)]
pub struct CreateExamRequest {
    pub exam_type_id: String,
    pub name: String,
    pub academic_year_id: String,
    pub term_id: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = exams)]
pub struct UpdateExamRequest {
    pub exam_type_id: Option<String>,
    pub name: Option<String>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub start_date: Option<NaiveDateTime>,
    pub end_date: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct ExamResponse {
    pub id: String,
    pub exam_type_id: String,
    pub name: String,
    pub academic_year_id: String,
    pub term_id: String,
    pub start_date: NaiveDateTime,
    pub end_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Exam> for ExamResponse {
    fn from(exam: Exam) -> Self {
        ExamResponse {
            id: exam.id,
            exam_type_id: exam.exam_type_id,
            name: exam.name,
            academic_year_id: exam.academic_year_id,
            term_id: exam.term_id,
            start_date: exam.start_date,
            end_date: exam.end_date,
            created_at: exam.created_at,
            updated_at: exam.updated_at,
        }
    }
}
