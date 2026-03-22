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

impl From<CreateAssetMaintenanceLogRequest> for AssetMaintenanceLog {
    fn from(req: CreateAssetMaintenanceLogRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            item_id: req.item_id,
            maintenance_date: req.maintenance_date,
            maintenance_type: req.maintenance_type,
            notes: req.notes,
            cost: req.cost,
            performed_by: req.performed_by,
            created_at: now,
        }
    }
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
