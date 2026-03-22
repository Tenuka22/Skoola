use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, ApiComponent, JsonSchema)]
#[diesel(table_name = crate::schema::resource_bookings)]
pub struct ResourceBooking {
    pub id: String,
    pub resource_id: String,
    pub booked_by_user_id: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub related_event_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ApiComponent, JsonSchema)]
#[diesel(table_name = crate::schema::resource_bookings)]
pub struct NewResourceBooking {
    pub id: String,
    pub resource_id: String,
    pub booked_by_user_id: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub related_event_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct BookResourceRequest {
    pub resource_id: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub related_event_id: Option<String>,
}
