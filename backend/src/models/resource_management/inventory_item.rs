use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::inventory_items)]
pub struct InventoryItem {
    pub id: String,
    pub category_id: String,
    pub item_name: String,
    pub unit: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::inventory_item_details)]
pub struct InventoryItemDetail {
    pub item_id: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub reorder_level: i32,
    pub unit_price: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateInventoryItemRequest {
    pub category_id: String,
    pub item_name: String,
    pub unit: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub reorder_level: i32,
    pub unit_price: f32,
}

impl From<CreateInventoryItemRequest> for InventoryItem {
    fn from(req: CreateInventoryItemRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            category_id: req.category_id,
            item_name: req.item_name,
            unit: req.unit,
            created_at: now,
            updated_at: now,
        }
    }
}

impl From<CreateInventoryItemRequest> for CreateInventoryItemDetailRequest {
    fn from(req: CreateInventoryItemRequest) -> Self {
        Self {
            item_id: uuid::Uuid::new_v4().to_string(),
            description: req.description,
            quantity: req.quantity,
            reorder_level: req.reorder_level,
            unit_price: req.unit_price,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::inventory_items)]
pub struct UpdateInventoryItemRequest {
    pub category_id: Option<String>,
    pub item_name: Option<String>,
    pub unit: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::inventory_item_details)]
pub struct CreateInventoryItemDetailRequest {
    pub item_id: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub reorder_level: i32,
    pub unit_price: f32,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::inventory_item_details)]
pub struct UpdateInventoryItemDetailRequest {
    pub description: Option<String>,
    pub quantity: Option<i32>,
    pub reorder_level: Option<i32>,
    pub unit_price: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct UpdateInventoryItemWithDetailsRequest {
    pub category_id: Option<String>,
    pub item_name: Option<String>,
    pub unit: Option<String>,
    pub description: Option<String>,
    pub reorder_level: Option<i32>,
    pub unit_price: Option<f32>,
}
