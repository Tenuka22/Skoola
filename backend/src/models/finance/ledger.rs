use crate::schema::general_ledger;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::{NaiveDate, NaiveDateTime};
use apistos::ApiComponent;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = general_ledger)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GeneralLedgerEntry {
    pub id: String,
    pub transaction_date: NaiveDate,
    pub description: Option<String>,
    pub debit_account_id: String,
    pub credit_account_id: String,
    pub amount: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = general_ledger)]
pub struct NewGeneralLedgerEntry {
    pub id: String,
    pub transaction_date: NaiveDate,
    pub description: Option<String>,
    pub debit_account_id: String,
    pub credit_account_id: String,
    pub amount: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = general_ledger)]
pub struct GeneralLedgerEntryChangeset {
    pub transaction_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub debit_account_id: Option<String>,
    pub credit_account_id: Option<String>,
    pub amount: Option<f32>,
    pub updated_at: NaiveDateTime,
}