use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;

use crate::database::enums::{EmploymentStatus, StaffType};
use crate::schema::{staff, staff_subjects};

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
#[diesel(table_name = staff)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Staff {
    pub id: String,
    pub employee_id: String,
    pub name: String,
    pub nic: String,
    pub dob: NaiveDate,
    pub gender: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub employment_status: EmploymentStatus,
    pub staff_type: StaffType,
    pub photo_url: Option<String>,
    pub profile_id: Option<String>,
}

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
    ApiComponent
)]
#[diesel(table_name = staff_subjects)]
#[diesel(belongs_to(Staff))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(staff_id, subject_id))]
pub struct StaffSubject {
    pub staff_id: String,
    pub subject_id: String,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct StaffQuery {
    pub search: Option<String>,
    pub employment_status: Option<EmploymentStatus>,
    pub staff_type: Option<StaffType>,
    pub created_after: Option<String>,
    pub created_before: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
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

impl From<crate::database::tables::Staff> for StaffResponse {
    fn from(staff: crate::database::tables::Staff) -> Self {
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

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedStaffResponse {
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub data: Vec<StaffResponse>,
}
