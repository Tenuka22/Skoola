use crate::schema::{uniform_items, uniform_issues};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, Clone, ApiComponent, JsonSchema)]
#[diesel(table_name = uniform_items)]
pub struct UniformItem {
    pub id: String,
    pub item_name: String,
    pub size: String,
    pub gender: String,
    pub grade_level: Option<String>,
    pub price: f32,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, Clone, ApiComponent, JsonSchema)]
#[diesel(table_name = uniform_issues)]
pub struct UniformIssue {
    pub id: String,
    pub student_id: String,
    pub uniform_item_id: String,
    pub quantity: i32,
    pub issue_date: NaiveDateTime,
    pub issued_by: String,
    pub amount_collected: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// Requests live in `crate::models::resource_management::resource` to avoid duplication.
