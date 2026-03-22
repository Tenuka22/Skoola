use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::schema::staff_events;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Clone,
)]
#[diesel(table_name = staff_events)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffEvent {
    pub id: String,
    pub event_name: String,
    pub event_type: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub location: Option<String>,
    pub organizer: Option<String>,
    pub counts_as_attendance: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StaffEventQuery {
    pub search: Option<String>,
    pub event_type: Option<String>,
    pub counts_as_attendance: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StaffEventQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CreateStaffEventRequest {
    pub event_name: String,
    pub event_type: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub location: Option<String>,
    pub organizer: Option<String>,
    pub counts_as_attendance: bool,
}

impl From<CreateStaffEventRequest> for StaffEvent {
    fn from(req: CreateStaffEventRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            event_name: req.event_name,
            event_type: req.event_type,
            start_date: req.start_date,
            end_date: req.end_date,
            location: req.location,
            organizer: req.organizer,
            counts_as_attendance: req.counts_as_attendance,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, JsonSchema, ApiComponent, AsChangeset)]
#[diesel(table_name = staff_events)]
pub struct UpdateStaffEventRequest {
    pub event_name: Option<String>,
    pub event_type: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub location: Option<String>,
    pub organizer: Option<String>,
    pub counts_as_attendance: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StaffEventResponse {
    pub id: String,
    pub event_name: String,
    pub event_type: String,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub location: Option<String>,
    pub organizer: Option<String>,
    pub counts_as_attendance: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StaffEvent> for StaffEventResponse {
    fn from(event: StaffEvent) -> Self {
        Self {
            id: event.id,
            event_name: event.event_name,
            event_type: event.event_type,
            start_date: event.start_date,
            end_date: event.end_date,
            location: event.location,
            organizer: event.organizer,
            counts_as_attendance: event.counts_as_attendance,
            created_at: event.created_at,
            updated_at: event.updated_at,
        }
    }
}
