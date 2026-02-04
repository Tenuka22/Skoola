use diesel::{Queryable, Insertable, AsChangeset};
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use apistos::ApiComponent;


#[derive(Debug, Clone, Queryable, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct GradingCriterion {
    pub id: String,
    pub scheme_id: String,
    pub min_marks: i32,
    pub max_marks: i32,
    pub grade: String,
    pub grade_point: Option<f32>,
    pub description: Option<String>,
}

#[derive(Debug, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::grading_criteria)]
pub struct NewGradingCriterion {
    pub id: String,
    pub scheme_id: String,
    pub min_marks: i32,
    pub max_marks: i32,
    pub grade: String,
    pub grade_point: Option<f32>,
    pub description: Option<String>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize, Default, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::grading_criteria)]
pub struct UpdateGradingCriterion {
    pub scheme_id: Option<String>,
    pub min_marks: Option<i32>,
    pub max_marks: Option<i32>,
    pub grade: Option<String>,
    pub grade_point: Option<f32>,
    pub description: Option<String>,
}