use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::staff_contracts;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Clone,
)]
#[diesel(table_name = staff_contracts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffContract {
    pub id: String,
    pub staff_id: String,
    pub contract_type: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub salary_amount: Option<f32>,
    pub currency: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StaffContractQuery {
    pub search: Option<String>,
    pub staff_id: Option<String>,
    pub contract_type: Option<String>,
    pub status: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StaffContractQuery {
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

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CreateStaffContractRequest {
    pub staff_id: String,
    pub contract_type: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub salary_amount: Option<f32>,
    pub currency: String,
    pub status: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, JsonSchema, ApiComponent, AsChangeset)]
#[diesel(table_name = staff_contracts)]
pub struct UpdateStaffContractRequest {
    pub contract_type: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub salary_amount: Option<f32>,
    pub currency: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StaffContractResponse {
    pub id: String,
    pub staff_id: String,
    pub contract_type: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub salary_amount: Option<f32>,
    pub currency: String,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StaffContract> for StaffContractResponse {
    fn from(contract: StaffContract) -> Self {
        Self {
            id: contract.id,
            staff_id: contract.staff_id,
            contract_type: contract.contract_type,
            start_date: contract.start_date,
            end_date: contract.end_date,
            salary_amount: contract.salary_amount,
            currency: contract.currency,
            status: contract.status,
            created_at: contract.created_at,
            updated_at: contract.updated_at,
        }
    }
}
