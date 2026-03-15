use crate::schema::{purchase_orders, purchase_order_items, vendors};
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = vendors)]
pub struct Vendor {
    pub id: String,
    pub name: String,
    pub contact_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct VendorResponse {
    pub id: String,
    pub name: String,
    pub contact_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}

impl From<Vendor> for VendorResponse {
    fn from(v: Vendor) -> Self {
        Self {
            id: v.id,
            name: v.name,
            contact_name: v.contact_name,
            phone: v.phone,
            email: v.email,
            address: v.address,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateVendorRequest {
    pub name: String,
    pub contact_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = vendors)]
pub struct UpdateVendorRequest {
    pub name: Option<String>,
    pub contact_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, Clone, ApiComponent, JsonSchema)]
#[diesel(table_name = purchase_orders)]
pub struct PurchaseOrder {
    pub id: String,
    pub vendor_id: String,
    pub order_date: NaiveDate,
    pub status: String,
    pub total_amount: f32,
    pub created_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreatePurchaseOrderRequest {
    pub vendor_id: String,
    pub order_date: NaiveDate,
    pub total_amount: f32,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = purchase_orders)]
pub struct UpdatePurchaseOrderRequest {
    pub vendor_id: Option<String>,
    pub order_date: Option<NaiveDate>,
    pub total_amount: Option<f32>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable, Clone, ApiComponent, JsonSchema)]
#[diesel(table_name = purchase_order_items)]
pub struct PurchaseOrderItem {
    pub id: String,
    pub purchase_order_id: String,
    pub item_name: String,
    pub quantity: f32,
    pub unit_price: f32,
    pub total_price: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreatePurchaseOrderItemRequest {
    pub purchase_order_id: String,
    pub item_name: String,
    pub quantity: f32,
    pub unit_price: f32,
    pub total_price: f32,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = purchase_order_items)]
pub struct UpdatePurchaseOrderItemRequest {
    pub item_name: Option<String>,
    pub quantity: Option<f32>,
    pub unit_price: Option<f32>,
    pub total_price: Option<f32>,
}
