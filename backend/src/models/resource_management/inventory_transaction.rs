use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;
use crate::database::enums::TransactionType;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::inventory_transactions)]
pub struct InventoryTransaction {
    pub id: String,
    pub item_id: String,
    pub transaction_type: TransactionType,
    pub quantity: f32,
    pub unit_cost: Option<f32>,
    pub transaction_date: NaiveDateTime,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::inventory_transactions)]
pub struct CreateInventoryTransactionRequest {
    pub item_id: String,
    pub transaction_type: TransactionType,
    pub quantity: f32,
    pub unit_cost: Option<f32>,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
}

impl From<CreateInventoryTransactionRequest> for InventoryTransaction {
    fn from(req: CreateInventoryTransactionRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            item_id: req.item_id,
            transaction_type: req.transaction_type,
            quantity: req.quantity,
            unit_cost: req.unit_cost,
            transaction_date: now,
            reference_type: req.reference_type,
            reference_id: req.reference_id,
            created_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::inventory_transactions)]
pub struct UpdateInventoryTransactionRequest {
    pub transaction_type: Option<TransactionType>,
    pub quantity: Option<f32>,
    pub unit_cost: Option<f32>,
    pub reference_type: Option<Option<String>>,
    pub reference_id: Option<Option<String>>,
}
