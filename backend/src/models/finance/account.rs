use crate::schema::chart_of_accounts;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::database::enums::{AccountTypeEnum, NormalBalanceType};

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
    pub account_type: AccountTypeEnum,
    pub normal_balance: NormalBalanceType,
    pub description: Option<String>,
    pub parent_account_id: Option<String>,
    pub is_active: bool,
    pub currency: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = chart_of_accounts)]
pub struct NewChartOfAccount {
    pub id: String,
    pub account_code: String,
    pub account_name: String,
    pub account_type: AccountTypeEnum,
    pub normal_balance: NormalBalanceType,
    pub description: Option<String>,
    pub parent_account_id: Option<String>,
    pub is_active: bool,
    pub currency: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = chart_of_accounts)]
pub struct ChartOfAccountChangeset {
    pub account_code: Option<String>,
    pub account_name: Option<String>,
    pub account_type: Option<AccountTypeEnum>,
    pub normal_balance: Option<NormalBalanceType>,
    pub description: Option<String>,
    pub parent_account_id: Option<String>,
    pub is_active: Option<bool>,
    pub currency: Option<String>,
    pub updated_at: NaiveDateTime,
}
