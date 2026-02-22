use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::asset_allocations_staff)]
#[diesel(primary_key(asset_allocation_id, staff_id))]
#[diesel(treat_compound_current_type_as_possible_option_type = "true")]
pub struct AssetAllocationStaff {
    pub asset_allocation_id: String,
    pub staff_id: String,
    pub created_at: Option<NaiveDateTime>,
}
