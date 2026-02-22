use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::inventory_items)]
#[diesel(treat_compound_current_type_as_possible_option_type = "true")]
pub struct InventoryItem {
    pub id: String,
    pub category_id: String,
    pub item_name: String,
    pub description: Option<String>,
    pub unit: String,
    pub quantity: i32,
    pub reorder_level: i32,
    pub unit_price: f64,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
