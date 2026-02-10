use diesel::prelude::*;
use diesel::Selectable; // Added
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Queryable, Insertable, Serialize, Deserialize, Clone, Selectable)]
#[diesel(table_name = crate::schema::grading_schemes)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GradingScheme {
    pub id: String,
    pub name: String,
    pub grade_level: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CreateGradingSchemeRequest {
    pub name: String,
    pub grade_level: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UpdateGradingSchemeRequest {
    pub name: Option<String>,
    pub grade_level: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GradingSchemeResponse {
    pub id: String,
    pub name: String,
    pub grade_level: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}