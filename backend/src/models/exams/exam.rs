use crate::schema::exams;
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
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
    pub term_id: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset, JsonSchema, ApiComponent,
)]
#[diesel(table_name = exams)]
pub struct CreateExamRequest {
    pub id: Option<String>,
    pub exam_type_id: String,
    pub name: String,
    pub academic_year_id: String,
    pub term_id: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = exams)]
pub struct UpdateExamRequest {
    pub exam_type_id: Option<String>,
    pub name: Option<String>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct ExamQuery {
    pub search: Option<String>,
    pub exam_type_id: Option<String>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for ExamQuery {
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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct ExamResponse {
    pub id: String,
    pub exam_type_id: String,
    pub name: String,
    pub academic_year_id: String,
    pub term_id: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
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

impl From<CreateExamRequest> for Exam {
    fn from(req: CreateExamRequest) -> Self {
        Self {
            id: req.id.unwrap_or_else(|| uuid::Uuid::new_v4().to_string()),
            exam_type_id: req.exam_type_id,
            name: req.name,
            academic_year_id: req.academic_year_id,
            term_id: req.term_id,
            start_date: req.start_date,
            end_date: req.end_date,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
