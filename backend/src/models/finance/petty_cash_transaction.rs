use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema; // Added for JsonSchema derive
use serde::{Deserialize, Serialize}; // Added for ApiComponent derive

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, JsonSchema, ApiComponent)] // Added JsonSchema, ApiComponent
#[diesel(table_name = crate::schema::petty_cash_transactions)]

pub struct PettyCashTransaction {
    pub id: String,
    pub amount: f32,
    pub transaction_type: crate::database::enums::TransactionType,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub handled_by: String, // staff_id
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct RecordPettyCashRequest {
    pub amount: f32,
    pub transaction_type: crate::database::enums::TransactionType,
    pub date: Option<NaiveDateTime>,
    pub description: Option<String>,
    pub handled_by: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct PettyCashTransactionResponse {
    pub id: String,
    pub amount: f32,
    pub transaction_type: crate::database::enums::TransactionType,
    pub date: NaiveDateTime,
    pub description: Option<String>,
    pub handled_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<PettyCashTransaction> for PettyCashTransactionResponse {
    fn from(t: PettyCashTransaction) -> Self {
        Self {
            id: t.id,
            amount: t.amount,
            transaction_type: t.transaction_type,
            date: t.date,
            description: t.description,
            handled_by: t.handled_by,
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}
