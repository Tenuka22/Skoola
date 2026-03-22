use crate::schema::exam_types;
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
#[diesel(table_name = exam_types)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ExamType {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub weightage: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset, JsonSchema, ApiComponent,
)]
#[diesel(table_name = exam_types)]
pub struct CreateExamTypeRequest {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub weightage: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = exam_types)]
pub struct UpdateExamTypeRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub weightage: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct ExamTypeQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for ExamTypeQuery {
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
pub struct ExamTypeResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub weightage: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ExamType> for ExamTypeResponse {
    fn from(exam_type: ExamType) -> Self {
        ExamTypeResponse {
            id: exam_type.id,
            name: exam_type.name,
            description: exam_type.description,
            weightage: exam_type.weightage,
            created_at: exam_type.created_at,
            updated_at: exam_type.updated_at,
        }
    }
}

impl From<CreateExamTypeRequest> for ExamType {
    fn from(req: CreateExamTypeRequest) -> Self {
        Self {
            id: req.id,
            name: req.name,
            description: req.description,
            weightage: req.weightage.unwrap_or(0.0),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
