use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::NaiveDateTime;
use apistos::ApiComponent;
use crate::schema::exam_types;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = exam_types)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ExamType {
    pub id: String, // Changed to String
    pub name: String,
    pub description: Option<String>,
    pub weightage: i32, // Changed to i32
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = exam_types)]
pub struct CreateExamTypeRequest {
    pub name: String,
    pub description: Option<String>,
    pub weightage: Option<i32>, // Changed to Option<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = exam_types)]
pub struct UpdateExamTypeRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub weightage: Option<i32>, // Changed to Option<i32>
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct ExamTypeResponse {
    pub id: String, // Changed to String
    pub name: String,
    pub description: Option<String>,
    pub weightage: i32, // Changed to i32
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
