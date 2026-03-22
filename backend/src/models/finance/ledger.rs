use crate::schema::{general_ledger, ledger_entries, ledger_transactions};
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
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

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
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

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = general_ledger)]
pub struct GeneralLedgerEntryChangeset {
    pub transaction_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub debit_account_id: Option<String>,
    pub credit_account_id: Option<String>,
    pub amount: Option<f32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct GeneralLedgerQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for GeneralLedgerQuery {
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
pub struct GeneralLedgerResponse {
    pub id: String,
    pub transaction_date: NaiveDate,
    pub description: Option<String>,
    pub debit_account_id: String,
    pub credit_account_id: String,
    pub amount: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<GeneralLedgerEntry> for GeneralLedgerResponse {
    fn from(entry: GeneralLedgerEntry) -> Self {
        Self {
            id: entry.id,
            transaction_date: entry.transaction_date,
            description: entry.description,
            debit_account_id: entry.debit_account_id,
            credit_account_id: entry.credit_account_id,
            amount: entry.amount,
            created_at: entry.created_at,
            updated_at: entry.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateGeneralLedgerRequest {
    pub transaction_date: NaiveDate,
    pub description: Option<String>,
    pub debit_account_id: String,
    pub credit_account_id: String,
    pub amount: f32,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = general_ledger)]
pub struct UpdateGeneralLedgerRequest {
    pub transaction_date: Option<NaiveDate>,
    pub description: Option<String>,
    pub debit_account_id: Option<String>,
    pub credit_account_id: Option<String>,
    pub amount: Option<f32>,
}

impl From<CreateGeneralLedgerRequest> for NewGeneralLedgerEntry {
    fn from(req: CreateGeneralLedgerRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            transaction_date: req.transaction_date,
            description: req.description,
            debit_account_id: req.debit_account_id,
            credit_account_id: req.credit_account_id,
            amount: req.amount,
            created_at: now,
            updated_at: now,
        }
    }
}

// Ledger Transaction
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
#[diesel(table_name = ledger_transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LedgerTransaction {
    pub id: String,
    pub transaction_date: NaiveDateTime,
    pub description: Option<String>,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = ledger_transactions)]
pub struct NewLedgerTransaction {
    pub id: String,
    pub transaction_date: NaiveDateTime,
    pub description: Option<String>,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = ledger_transactions)]
pub struct LedgerTransactionChangeset {
    pub transaction_date: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct LedgerTransactionQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for LedgerTransactionQuery {
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
pub struct LedgerTransactionResponse {
    pub id: String,
    pub transaction_date: NaiveDateTime,
    pub description: Option<String>,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
    pub created_at: NaiveDateTime,
}

impl From<LedgerTransaction> for LedgerTransactionResponse {
    fn from(transaction: LedgerTransaction) -> Self {
        Self {
            id: transaction.id,
            transaction_date: transaction.transaction_date,
            description: transaction.description,
            reference_type: transaction.reference_type,
            reference_id: transaction.reference_id,
            created_at: transaction.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateLedgerTransactionRequest {
    pub transaction_date: NaiveDateTime,
    pub description: Option<String>,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = ledger_transactions)]
pub struct UpdateLedgerTransactionRequest {
    pub transaction_date: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
}

impl From<CreateLedgerTransactionRequest> for NewLedgerTransaction {
    fn from(req: CreateLedgerTransactionRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            transaction_date: req.transaction_date,
            description: req.description,
            reference_type: req.reference_type,
            reference_id: req.reference_id,
            created_at: now,
        }
    }
}

// Ledger Entry
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
#[diesel(table_name = ledger_entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct LedgerEntry {
    pub id: String,
    pub transaction_id: String,
    pub account_id: String,
    pub entry_type: String, // Debit/Credit
    pub amount: f32,
    pub memo: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = ledger_entries)]
pub struct NewLedgerEntry {
    pub id: String,
    pub transaction_id: String,
    pub account_id: String,
    pub entry_type: String,
    pub amount: f32,
    pub memo: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = ledger_entries)]
pub struct LedgerEntryChangeset {
    pub transaction_id: Option<String>,
    pub account_id: Option<String>,
    pub entry_type: Option<String>,
    pub amount: Option<f32>,
    pub memo: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct LedgerEntryQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for LedgerEntryQuery {
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
pub struct LedgerEntryResponse {
    pub id: String,
    pub transaction_id: String,
    pub account_id: String,
    pub entry_type: String,
    pub amount: f32,
    pub memo: Option<String>,
    pub created_at: NaiveDateTime,
}

impl From<LedgerEntry> for LedgerEntryResponse {
    fn from(entry: LedgerEntry) -> Self {
        Self {
            id: entry.id,
            transaction_id: entry.transaction_id,
            account_id: entry.account_id,
            entry_type: entry.entry_type,
            amount: entry.amount,
            memo: entry.memo,
            created_at: entry.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateLedgerEntryRequest {
    pub transaction_id: String,
    pub account_id: String,
    pub entry_type: String,
    pub amount: f32,
    pub memo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = ledger_entries)]
pub struct UpdateLedgerEntryRequest {
    pub transaction_id: Option<String>,
    pub account_id: Option<String>,
    pub entry_type: Option<String>,
    pub amount: Option<f32>,
    pub memo: Option<String>,
}

impl From<CreateLedgerEntryRequest> for NewLedgerEntry {
    fn from(req: CreateLedgerEntryRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            transaction_id: req.transaction_id,
            account_id: req.account_id,
            entry_type: req.entry_type,
            amount: req.amount,
            memo: req.memo,
            created_at: now,
        }
    }
}
