use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::database::enums::GradingSchemeType;

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::grading_schemes)]
pub struct GradingScheme {
    pub id: String,
    pub name: String,
    pub scheme_type: GradingSchemeType,
    pub grade_level_id: Option<String>,
    pub scale_definition: String,
    pub pass_mark: Option<i32>,
    pub is_default: bool,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::grading_schemes)]
pub struct NewGradingScheme {
    pub id: String,
    pub name: String,
    pub scheme_type: GradingSchemeType,
    pub grade_level_id: Option<String>,
    pub scale_definition: String,
    pub pass_mark: Option<i32>,
    pub is_default: bool,
    pub description: Option<String>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize, Default, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::grading_schemes)]
pub struct UpdateGradingScheme {
    pub name: Option<String>,
    pub scheme_type: Option<GradingSchemeType>,
    pub grade_level_id: Option<String>,
    pub scale_definition: Option<String>,
    pub pass_mark: Option<i32>,
    pub is_default: Option<bool>,
    pub description: Option<String>,
}
