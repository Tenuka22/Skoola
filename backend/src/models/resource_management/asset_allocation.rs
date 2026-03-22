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

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::asset_allocations)]
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

impl From<AllocateAssetRequest> for AssetAllocation {
    fn from(req: AllocateAssetRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        // Determine allocated_to_type and allocated_to_id from staff_id or student_id
        let (allocated_to_type, allocated_to_id) = if let Some(staff_id) = req.staff_id {
            ("staff".to_string(), staff_id)
        } else if let Some(student_id) = req.student_id {
            ("student".to_string(), student_id)
        } else if let Some(allocated_to_id) = req.allocated_to_id {
            (req.allocated_to_type.unwrap_or_else(|| "unknown".to_string()), allocated_to_id)
        } else {
            ("unknown".to_string(), String::new())
        };

        Self {
            id: uuid::Uuid::new_v4().to_string(),
            item_id: req.item_id,
            allocated_to_type,
            allocated_to_id,
            quantity: req.quantity,
            allocation_date: req.allocation_date,
            return_date: None,
            allocated_by: req.allocated_by,
            created_at: now,
            updated_at: now,
        }
    }
}
