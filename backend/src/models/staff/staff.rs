use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::database::enums::{EmploymentStatus, Gender, StaffType};
use crate::schema::{
    staff, staff_contacts, staff_employment_status, staff_identity, staff_media,
    staff_reward_snapshots, staff_subjects,
};

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
    pub dob: NaiveDate,
    pub gender: Gender,
    pub staff_type: StaffType,
    pub profile_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = staff)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewStaff {
    pub id: String,
    pub employee_id: String,
    pub name: String,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub staff_type: StaffType,
    pub profile_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
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
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StaffQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CreateStaffRequest {
    pub id: String,
    pub employee_id: String,
    pub name: String,
    pub nic: String,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub photo_url: Option<String>,
    pub employment_status: EmploymentStatus,
    pub staff_type: StaffType,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::staff)]
pub struct UpdateStaffRequest {
    pub name: Option<String>,
    pub dob: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub staff_type: Option<StaffType>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct StaffResponse {
    pub id: String,
    pub employee_id: String,
    pub name: String,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub employment_status: Option<EmploymentStatus>,
    pub staff_type: StaffType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub profile_id: Option<String>,
    pub reward_points_balance: Option<i32>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub photo_url: Option<String>,
    pub nic: Option<String>,
    pub profile_name: Option<String>,
    pub profile_address: Option<String>,
    pub profile_phone: Option<String>,
    pub profile_photo_url: Option<String>,
    pub user_email: Option<String>,
}

impl From<Staff> for StaffResponse {
    fn from(staff: Staff) -> Self {
        Self {
            id: staff.id,
            employee_id: staff.employee_id,
            name: staff.name,
            dob: staff.dob,
            gender: staff.gender,
            employment_status: None,
            staff_type: staff.staff_type,
            created_at: staff.created_at,
            updated_at: staff.updated_at,
            profile_id: staff.profile_id,
            reward_points_balance: None,
            address: None,
            phone: None,
            email: None,
            photo_url: None,
            nic: None,
            profile_name: None,
            profile_address: None,
            profile_phone: None,
            profile_photo_url: None,
            user_email: None,
        }
    }
}

impl From<crate::database::tables::Staff> for StaffResponse {
    fn from(staff: crate::database::tables::Staff) -> Self {
        Self {
            id: staff.id,
            employee_id: staff.employee_id,
            name: staff.name,
            dob: staff.dob,
            gender: staff.gender,
            employment_status: None,
            staff_type: staff.staff_type,
            created_at: staff.created_at,
            updated_at: staff.updated_at,
            profile_id: staff.profile_id,
            reward_points_balance: None,
            address: None,
            phone: None,
            email: None,
            photo_url: None,
            nic: None,
            profile_name: None,
            profile_address: None,
            profile_phone: None,
            profile_photo_url: None,
            user_email: None,
        }
    }
}
