use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::asset_allocations_students)]
#[diesel(primary_key(asset_allocation_id, student_id))]
pub struct AssetAllocationStudent {
    pub asset_allocation_id: String,
    pub student_id: String,
    pub created_at: NaiveDateTime,
}
