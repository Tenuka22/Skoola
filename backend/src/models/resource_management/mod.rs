use crate::schema::{resources, resource_bookings};
use diesel::{Queryable, Insertable};
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use apistos::ApiComponent;

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = resources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Resource {
    pub id: String,
    pub resource_name: String,
    pub resource_type: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Queryable, Selectable, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = resource_bookings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
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

#[derive(Debug, Clone, PartialEq, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = resources)]
pub struct NewResource {
    pub id: String,
    pub resource_name: String,
    pub resource_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = resource_bookings)]
pub struct NewResourceBooking {
    pub id: String,
    pub resource_id: String,
    pub booked_by_user_id: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub related_event_id: Option<String>,
}
