use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::database::enums::Medium;
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, ApiComponent, JsonSchema)]
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
