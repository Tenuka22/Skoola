use chrono::{NaiveDateTime, NaiveDate};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::asset_maintenance_logs)]
pub struct AssetMaintenanceLog {
    pub id: String,
    pub item_id: String,
    pub maintenance_date: NaiveDate,
    pub maintenance_type: String,
    pub notes: Option<String>,
    pub cost: Option<f32>,
    pub performed_by: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::asset_maintenance_logs)]
pub struct CreateAssetMaintenanceLogRequest {
    pub item_id: String,
    pub maintenance_date: NaiveDate,
    pub maintenance_type: String,
    pub notes: Option<String>,
    pub cost: Option<f32>,
    pub performed_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::asset_maintenance_logs)]
pub struct UpdateAssetMaintenanceLogRequest {
    pub maintenance_date: Option<NaiveDate>,
    pub maintenance_type: Option<String>,
    pub notes: Option<String>,
    pub cost: Option<f32>,
    pub performed_by: Option<String>,
}
