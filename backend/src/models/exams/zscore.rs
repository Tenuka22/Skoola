use crate::schema::{student_zscores, zscore_calculations};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, ApiComponent, JsonSchema)]
#[diesel(table_name = zscore_calculations)]
pub struct ZScoreCalculation {
    pub exam_id: String,
    pub subject_id: String,
    pub mean: f32,
    pub std_deviation: f32,
    pub calculated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ApiComponent, JsonSchema)]
#[diesel(table_name = zscore_calculations)]
pub struct CreateZScoreCalculation {
    pub exam_id: String,
    pub subject_id: String,
    pub mean: f32,
    pub std_deviation: f32,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = student_zscores)]
pub struct StudentZScore {
    pub student_id: String,
    pub exam_id: String,
    pub subject_id: String,
    pub zscore: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CalculateZScoreRequest {
    pub exam_id: String,
}
