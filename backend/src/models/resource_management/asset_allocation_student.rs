use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::asset_allocations_students)]
#[diesel(primary_key(asset_allocation_id, student_id))]

pub struct AssetAllocationStudent {
    pub asset_allocation_id: String,
    pub student_id: String,
    pub created_at: NaiveDateTime,
}
