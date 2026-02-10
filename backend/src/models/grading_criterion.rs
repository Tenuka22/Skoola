use diesel::prelude::*;
use diesel::Selectable; // Added
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use diesel::sql_types::Float; // Added

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone, Selectable)]
#[diesel(table_name = crate::schema::grading_criteria)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GradingCriterion {
    pub id: String,
    pub scheme_id: String,
    pub min_marks: i32,
    pub max_marks: i32,
    pub grade: String,
    #[diesel(sql_type = Float)] // Explicitly set SQL type
    pub grade_point: Option<f32>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateGradingCriterionRequest {
    pub scheme_id: String,
    pub min_marks: i32,
    pub max_marks: i32,
    pub grade: String,
    pub grade_point: Option<f32>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateGradingCriterionRequest {
    pub scheme_id: Option<String>,
    pub min_marks: Option<i32>,
    pub max_marks: Option<i32>,
    pub grade: Option<String>,
    pub grade_point: Option<f32>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GradingCriterionResponse {
    pub id: String,
    pub scheme_id: String,
    pub min_marks: i32,
    pub max_marks: i32,
    pub grade: String,
    pub grade_point: Option<f32>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}