use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::maintenance_requests)]

pub struct MaintenanceRequest {
    pub id: String,
    pub item_id: String,
    pub issue_description: String,
    pub reported_by: String, // user_id
    pub reported_date: NaiveDateTime,
    pub status: String,
    pub assigned_to: Option<String>, // staff_id
    pub resolved_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
