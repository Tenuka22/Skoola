use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::database::enums::Medium;
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, ApiComponent, JsonSchema)]
#[diesel(table_name = crate::schema::substitution_plans)]
pub struct SubstitutionPlan {
    pub id: String,
    pub subject_id: String,
    pub medium: Medium,
    pub plan_name: String,
    pub content_link: Option<String>,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CreateSubstitutionPlanRequest {
    pub subject_id: String,
    pub medium: Medium,
    pub plan_name: String,
    pub content_link: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema, AsChangeset)]
#[diesel(table_name = crate::schema::substitution_plans)]
pub struct UpdateSubstitutionPlanRequest {
    pub subject_id: Option<String>,
    pub medium: Option<Medium>,
    pub plan_name: Option<String>,
    pub content_link: Option<String>,
    pub description: Option<String>,
}
