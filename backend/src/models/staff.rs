use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::AsChangeset;

use crate::database::enums::{EmploymentStatus, StaffType};
pub use crate::database::tables::Staff;
use crate::schema::staff;

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct StaffQuery {
    pub search: Option<String>,
    pub employment_status: Option<EmploymentStatus>,
    pub staff_type: Option<StaffType>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CreateStaffRequest {
    pub employee_id: String,
    pub name: String,
    pub nic: String,
    pub dob: NaiveDate,
    pub gender: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub employment_status: EmploymentStatus,
    pub staff_type: StaffType,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct UpdateStaffRequest {
    pub name: Option<String>,
    pub nic: Option<String>,
    pub dob: Option<NaiveDate>,
    pub gender: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize, ApiComponent, JsonSchema)]
#[diesel(table_name = staff)]
pub struct StaffChangeset {
    pub name: Option<String>,
    pub nic: Option<String>,
    pub dob: Option<NaiveDate>,
    pub gender: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub employment_status: Option<EmploymentStatus>,
    pub staff_type: Option<StaffType>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct StaffResponse {
    pub id: String,
    pub employee_id: String,
    pub name: String,
    pub nic: String,
    pub dob: NaiveDate,
    pub gender: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub photo_url: Option<String>,
    pub employment_status: EmploymentStatus,
    pub staff_type: StaffType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Staff> for StaffResponse {
    fn from(staff: Staff) -> Self {
        StaffResponse {
            id: staff.id,
            employee_id: staff.employee_id,
            name: staff.name,
            nic: staff.nic,
            dob: staff.dob,
            gender: staff.gender,
            address: staff.address,
            phone: staff.phone,
            email: staff.email,
            photo_url: staff.photo_url,
            employment_status: staff.employment_status,
            staff_type: staff.staff_type,
            created_at: staff.created_at,
            updated_at: staff.updated_at,
        }
    }
}
