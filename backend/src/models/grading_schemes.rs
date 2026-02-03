use diesel::{Queryable, Insertable, AsChangeset};
use serde::{Serialize, Deserialize};
use schemars::JsonSchema;
use apistos::ApiComponent;

use crate::schema::grading_schemes;

#[derive(Debug, Clone, Queryable, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct GradingScheme {
    pub id: String,
    pub name: String,
    pub grade_level: String,
    pub description: Option<String>,
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
