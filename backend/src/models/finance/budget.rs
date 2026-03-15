use crate::schema::budgets;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Clone,
    ApiComponent,
)]
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
    pub spent_amount: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = budgets)]
pub struct UpdateBudgetRequest {
    pub academic_year_id: Option<String>,
    pub category_id: Option<String>,
    pub allocated_amount: Option<f32>,
    pub spent_amount: Option<f32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BudgetQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for BudgetQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
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
