use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::resource_bookings)]
pub struct NewResourceBooking {
    pub id: String,
    pub resource_id: String,
    pub booked_by_user_id: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub related_event_id: Option<String>,
}
