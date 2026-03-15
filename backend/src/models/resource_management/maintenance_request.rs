use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::database::enums::MaintenanceStatus;
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::maintenance_requests)]
pub struct MaintenanceRequest {
    pub id: String,
    pub item_id: String,
    pub issue_description: String,
    pub reported_by: String, // user_id
    pub reported_date: NaiveDateTime,
    pub status: MaintenanceStatus,
    pub assigned_to: Option<String>, // staff_id
    pub resolved_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateMaintenanceRequest {
    pub item_id: String,
    pub issue_description: String,
    pub reported_by: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = crate::schema::maintenance_requests)]
pub struct UpdateMaintenanceStatusRequest {
    pub status: MaintenanceStatus,
    pub assigned_to: Option<String>,
    pub resolved_date: Option<NaiveDateTime>,
}
