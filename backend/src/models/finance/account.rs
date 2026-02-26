use crate::schema::chart_of_accounts;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = chart_of_accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ChartOfAccount {
    pub id: String,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub normal_balance: String,
    pub description: Option<String>,
    pub parent_account_id: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = chart_of_accounts)]
pub struct NewChartOfAccount {
    pub id: String,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub normal_balance: String,
    pub description: Option<String>,
    pub parent_account_id: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = chart_of_accounts)]
pub struct ChartOfAccountChangeset {
    pub account_code: Option<String>,
    pub account_name: Option<String>,
    pub account_type: Option<String>,
    pub normal_balance: Option<String>,
    pub description: Option<String>,
    pub parent_account_id: Option<String>,
    pub is_active: Option<bool>,
    pub updated_at: NaiveDateTime,
}
