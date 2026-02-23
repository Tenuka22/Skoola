use crate::database::enums::{PaymentMethod, TransactionType};
use crate::schema::{income_transactions, expense_transactions};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use apistos::ApiComponent;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = income_transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct IncomeTransaction {
    pub id: String,
    pub source_id: String,
    pub amount: f32,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub received_by: String,
    pub receipt_number: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = expense_transactions)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ExpenseTransaction {
    pub id: String,
    pub category_id: String,
    pub amount: f32,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub vendor: Option<String>,
    pub payment_method: PaymentMethod,
    pub approved_by: Option<String>,
    pub receipt_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct RecordIncomeRequest {
    pub source_id: String,
    pub amount: f32,
    pub date: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub received_by: String,
    pub receipt_number: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct IncomeTransactionResponse {
    pub id: String,
    pub source_id: String,
    pub amount: f32,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub received_by: String,
    pub receipt_number: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<IncomeTransaction> for IncomeTransactionResponse {
    fn from(t: IncomeTransaction) -> Self {
        Self {
            id: t.id,
            source_id: t.source_id,
            amount: t.amount,
            date: t.date,
            description: t.description,
            received_by: t.received_by,
            receipt_number: t.receipt_number,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}



#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct RecordExpenseRequest {
    pub category_id: String,
    pub amount: f32,
    pub date: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub vendor: Option<String>,
    pub payment_method: PaymentMethod,
    pub approved_by: Option<String>,
    pub receipt_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ExpenseTransactionResponse {
    pub id: String,
    pub category_id: String,
    pub amount: f32,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub vendor: Option<String>,
    pub payment_method: PaymentMethod,
    pub approved_by: Option<String>,
    pub receipt_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ExpenseTransaction> for ExpenseTransactionResponse {
    fn from(t: ExpenseTransaction) -> Self {
        Self {
            id: t.id,
            category_id: t.category_id,
            amount: t.amount,
            date: t.date,
            description: t.description,
            vendor: t.vendor,
            payment_method: t.payment_method,
            approved_by: t.approved_by,
            receipt_url: t.receipt_url,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}



#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ReconcilePettyCashRequest {
    pub physical_balance: f32,
    pub remarks: Option<String>,
    pub handled_by: String,
}
