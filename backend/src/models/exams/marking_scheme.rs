use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::marking_schemes)]
pub struct MarkingScheme {
    pub id: String,
    pub name: String,
    pub subject_id: String,
    pub grade_level_id: Option<String>,
    pub curriculum_standard_id: Option<String>,
    pub stream_id: Option<String>,
    pub description: Option<String>,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
    pub calculation_fn: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::marking_schemes)]
pub struct NewMarkingScheme {
    pub id: String,
    pub name: String,
    pub subject_id: String,
    pub grade_level_id: Option<String>,
    pub curriculum_standard_id: Option<String>,
    pub stream_id: Option<String>,
    pub description: Option<String>,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
    pub calculation_fn: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::marking_scheme_parts)]
pub struct MarkingSchemePart {
    pub id: String,
    pub scheme_id: String,
    pub paper_label: String,
    pub part_label: String,
    pub question_label: Option<String>,
    pub max_marks: f32,
    pub weight_ratio: Option<f32>,
    pub structure_json: Option<String>,
    pub order_index: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::marking_scheme_parts)]
pub struct NewMarkingSchemePart {
    pub id: String,
    pub scheme_id: String,
    pub paper_label: String,
    pub part_label: String,
    pub question_label: Option<String>,
    pub max_marks: f32,
    pub weight_ratio: Option<f32>,
    pub structure_json: Option<String>,
    pub order_index: i32,
}
