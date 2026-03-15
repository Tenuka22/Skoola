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
    AsChangeset,
    Clone,
    Associations,
    ApiComponent,
)]
#[diesel(table_name = staff_qualifications)]
#[diesel(belongs_to(Staff))]
pub struct StaffQualification {
    pub id: String,
    pub staff_id: String,
    pub degree: String,
    pub institution: String,
    pub year_of_completion: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub file_name: Option<String>,
    pub file_url: Option<String>,
    pub file_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = staff_qualifications)]
pub struct CreateStaffQualificationRequest {
    pub staff_id: String,
    pub degree: String,
    pub institution: String,
    pub year_of_completion: i32,
    pub file_name: Option<String>,
    pub file_url: Option<String>,
    pub file_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = staff_qualifications)]
pub struct UpdateStaffQualificationRequest {
    pub degree: Option<String>,
    pub institution: Option<String>,
    pub year_of_completion: Option<i32>,
    pub file_name: Option<String>,
    pub file_url: Option<String>,
    pub file_type: Option<String>,
}
