use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::asset_allocations)]
pub struct AssetAllocation {
    pub id: String,
    pub item_id: String,
    pub allocated_to_type: String,
    pub allocated_to_id: String,
    pub quantity: i32,
    pub allocation_date: NaiveDateTime,
    pub return_date: Option<NaiveDateTime>,
    pub allocated_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct AssetAllocationResponse {
    pub id: String,
    pub item_id: String,
    pub allocated_to_type: String,
    pub allocated_to_id: String,
    pub quantity: i32,
    pub allocation_date: NaiveDateTime,
    pub return_date: Option<NaiveDateTime>,
    pub allocated_by: String,
}

impl From<AssetAllocation> for AssetAllocationResponse {
    fn from(a: AssetAllocation) -> Self {
        Self {
            id: a.id,
            item_id: a.item_id,
            allocated_to_type: a.allocated_to_type,
            allocated_to_id: a.allocated_to_id,
            quantity: a.quantity,
            allocation_date: a.allocation_date,
            return_date: a.return_date,
            allocated_by: a.allocated_by,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct ReturnAssetRequest {
    pub return_date: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct AllocateAssetRequest {
    pub item_id: String,
    pub staff_id: Option<String>,
    pub student_id: Option<String>,
    pub allocated_to_type: Option<String>,
    pub allocated_to_id: Option<String>,
    pub quantity: i32,
    pub allocation_date: NaiveDateTime,
    pub allocated_by: String,
}
