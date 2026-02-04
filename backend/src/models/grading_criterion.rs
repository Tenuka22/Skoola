use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = crate::schema::grading_criteria)]
pub struct GradingCriterion {
    pub id: Uuid,
    pub scheme_id: Uuid,
    pub min_marks: f64,
    pub max_marks: f64,
    pub grade: String,
    pub grade_point: f64,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateGradingCriterionRequest {
    pub scheme_id: Uuid,
    pub min_marks: f64,
    pub max_marks: f64,
    pub grade: String,
    pub grade_point: f64,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateGradingCriterionRequest {
    pub scheme_id: Option<Uuid>,
    pub min_marks: Option<f64>,
    pub max_marks: Option<f64>,
    pub grade: Option<String>,
    pub grade_point: Option<f64>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GradingCriterionResponse {
    pub id: Uuid,
    pub scheme_id: Uuid,
    pub min_marks: f64,
    pub max_marks: f64,
    pub grade: String,
    pub grade_point: f64,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}