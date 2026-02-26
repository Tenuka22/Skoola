use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, ApiComponent, JsonSchema)]
#[diesel(table_name = crate::schema::budget_categories)]
pub struct BudgetCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateBudgetCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct BudgetCategoryResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<BudgetCategory> for BudgetCategoryResponse {
    fn from(cat: BudgetCategory) -> Self {
        Self {
            id: cat.id,
            name: cat.name,
            description: cat.description,
            created_at: cat.created_at,
            updated_at: cat.updated_at,
        }
    }
}
