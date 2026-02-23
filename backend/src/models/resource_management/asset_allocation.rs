use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::asset_allocations)]

pub struct AssetAllocation {
    pub id: String,
    pub item_id: String,
    pub allocated_to_type: String,
    pub allocated_to_id: String, // This will store staff_id or student_id
    pub quantity: i32,
    pub allocation_date: NaiveDateTime,
    pub return_date: Option<NaiveDateTime>,
    pub allocated_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
