use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::asset_allocations_staff)]
#[diesel(primary_key(asset_allocation_id, staff_id))]

pub struct AssetAllocationStaff {
    pub asset_allocation_id: String,
    pub staff_id: String,
    pub created_at: NaiveDateTime,
}
