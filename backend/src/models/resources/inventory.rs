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

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, Clone, ApiComponent, JsonSchema)]
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

impl From<crate::models::resource_management::resource::IssueUniformRequest> for UniformIssue {
    fn from(req: crate::models::resource_management::resource::IssueUniformRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            student_id: req.student_id,
            uniform_item_id: req.uniform_item_id,
            quantity: req.quantity,
            issue_date: now,
            issued_by: req.issued_by,
            amount_collected: req.amount_collected,
            created_at: now,
            updated_at: now,
        }
    }
}

// Requests live in `crate::models::resource_management::resource` to avoid duplication.
