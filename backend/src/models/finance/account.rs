use crate::database::enums::{AccountTypeEnum, NormalBalanceType};
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
    pub account_type: AccountTypeEnum,
    pub normal_balance: NormalBalanceType,
    pub description: Option<String>,
    pub parent_account_id: Option<String>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
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

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
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
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct ChartOfAccountQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for ChartOfAccountQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ChartOfAccountResponse {
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

impl From<ChartOfAccount> for ChartOfAccountResponse {
    fn from(account: ChartOfAccount) -> Self {
        Self {
            id: account.id,
            account_code: account.account_code,
            account_name: account.account_name,
            account_type: account.account_type,
            normal_balance: account.normal_balance,
            description: account.description,
            parent_account_id: account.parent_account_id,
            is_active: account.is_active,
            currency: account.currency,
            created_at: account.created_at,
            updated_at: account.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateChartOfAccountRequest {
    pub account_code: String,
    pub account_name: String,
    pub account_type: AccountTypeEnum,
    pub normal_balance: NormalBalanceType,
    pub description: Option<String>,
    pub parent_account_id: Option<String>,
    pub is_active: bool,
    pub currency: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = chart_of_accounts)]
pub struct UpdateChartOfAccountRequest {
    pub account_code: Option<String>,
    pub account_name: Option<String>,
    pub account_type: Option<AccountTypeEnum>,
    pub normal_balance: Option<NormalBalanceType>,
    pub description: Option<String>,
    pub parent_account_id: Option<String>,
    pub is_active: Option<bool>,
    pub currency: Option<String>,
}

impl From<CreateChartOfAccountRequest> for NewChartOfAccount {
    fn from(req: CreateChartOfAccountRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            account_code: req.account_code,
            account_name: req.account_name,
            account_type: req.account_type,
            normal_balance: req.normal_balance,
            description: req.description,
            parent_account_id: req.parent_account_id,
            is_active: req.is_active,
            currency: req.currency,
            created_at: now,
            updated_at: now,
        }
    }
}
