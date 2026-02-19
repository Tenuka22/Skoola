use crate::schema::chart_of_accounts;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::NaiveDateTime;
use apistos::ApiComponent;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = chart_of_accounts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ChartOfAccount {
    pub id: String,
    pub account_name: String,
    pub account_type: String,
    pub normal_balance: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = chart_of_accounts)]
pub struct NewChartOfAccount {
    pub id: String,
    pub account_name: String,
    pub account_type: String,
    pub normal_balance: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = chart_of_accounts)]
pub struct ChartOfAccountChangeset {
    pub account_name: Option<String>,
    pub account_type: Option<String>,
    pub normal_balance: Option<String>,
    pub description: Option<String>,
    pub updated_at: NaiveDateTime,
}