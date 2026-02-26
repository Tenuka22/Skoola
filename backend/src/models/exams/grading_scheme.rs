use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Queryable, Selectable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::grading_schemes)]
pub struct GradingScheme {
    pub id: String,
    pub name: String,
    pub grade_level: String,
    pub description: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Debug, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::grading_schemes)]
pub struct NewGradingScheme {
    pub id: String,
    pub name: String,
    pub grade_level: String,
    pub description: Option<String>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize, Default, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::grading_schemes)]
pub struct UpdateGradingScheme {
    pub name: Option<String>,
    pub grade_level: Option<String>,
    pub description: Option<String>,
}
