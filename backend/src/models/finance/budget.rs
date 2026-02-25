use crate::schema::budgets;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use apistos::ApiComponent;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = budgets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Budget {
    pub id: String,
    pub academic_year_id: String,
    pub category_id: String,
    pub allocated_amount: f32,
    pub spent_amount: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SetBudgetRequest {
    pub academic_year_id: String,
    pub category_id: String,
    pub allocated_amount: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct UpdateBudgetRequest {
    pub allocated_amount: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct BudgetSummaryResponse {
    pub category_name: String,
    pub allocated: f32,
    pub spent: f32,
    pub remaining: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct BudgetResponse {
    pub id: String,
    pub academic_year_id: String,
    pub category_id: String,
    pub allocated_amount: f32,
    pub spent_amount: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Budget> for BudgetResponse {
    fn from(b: Budget) -> Self {
        Self {
            id: b.id,
            academic_year_id: b.academic_year_id,
            category_id: b.category_id,
            allocated_amount: b.allocated_amount,
            spent_amount: b.spent_amount,
            created_at: b.created_at,
            updated_at: b.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct BudgetComparisonResponse {
    pub category_name: String,
    pub allocated: f32,
    pub actual_spent: f32,
    pub variance: f32,
    pub variance_percentage: f32,
}
