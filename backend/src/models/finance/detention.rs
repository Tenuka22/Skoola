use crate::schema::detention_balances;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::services::admin_db::{AdminQuery, AsAdminQuery};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, AsChangeset, Clone, ApiComponent)]
#[diesel(table_name = detention_balances)]
#[diesel(primary_key(student_id))]
pub struct DetentionBalance {
    pub student_id: String,
    pub total_hours_assigned: f32,
    pub total_hours_served: f32,
    pub remaining_hours: f32,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct DetentionBalanceResponse {
    pub student_id: String,
    pub total_hours_assigned: f32,
    pub total_hours_served: f32,
    pub remaining_hours: f32,
    pub updated_at: NaiveDateTime,
}

impl From<DetentionBalance> for DetentionBalanceResponse {
    fn from(d: DetentionBalance) -> Self {
        Self {
            student_id: d.student_id,
            total_hours_assigned: d.total_hours_assigned,
            total_hours_served: d.total_hours_served,
            remaining_hours: d.remaining_hours,
            updated_at: d.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateDetentionBalanceRequest {
    pub student_id: String,
    pub total_hours_assigned: f32,
    pub total_hours_served: f32,
    pub remaining_hours: f32,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = detention_balances)]
pub struct UpdateDetentionBalanceRequest {
    pub total_hours_assigned: Option<f32>,
    pub total_hours_served: Option<f32>,
    pub remaining_hours: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct DetentionBalanceQuery {
    pub student_id: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for DetentionBalanceQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}
