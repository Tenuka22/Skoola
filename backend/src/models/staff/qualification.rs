use crate::models::staff::staff::Staff;
use crate::schema::staff_qualifications;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    Clone,
    Associations,
    ApiComponent,
)]
#[diesel(table_name = staff_qualifications)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffQualification {
    pub id: String,
    pub staff_id: String,
    pub degree: String,
    pub institution: String,
    pub year_of_completion: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
