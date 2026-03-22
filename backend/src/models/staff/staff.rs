use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::database::enums::{EmploymentStatus, Gender, StaffType};
use crate::schema::staff;

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
        ..Default::default()}
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

impl From<CreateStaffRequest> for Staff {
    fn from(req: CreateStaffRequest) -> Self {
        Staff {
            id: req.id,
            employee_id: req.employee_id,
            name: req.name,
            dob: req.dob,
            gender: req.gender,
            staff_type: req.staff_type,
            profile_id: None,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct UpdateStaffRequest {
    // staff
    pub employee_id: Option<String>,
    pub name: Option<String>,
    pub dob: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub staff_type: Option<StaffType>,
    pub profile_id: Option<String>,

    // profiles (linked via profile_id)
    pub profile_name: Option<String>,

    // profile_contacts / staff_contacts
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,

    // profile_media / staff_media
    pub photo_url: Option<String>,

    // staff_identity
    pub nic: Option<String>,

    // staff_employment_status
    pub employment_status: Option<EmploymentStatus>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
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
#[diesel(table_name = crate::schema::staff_contacts)]
#[diesel(primary_key(staff_id))]
pub struct StaffContact {
    pub staff_id: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub address_latitude: Option<f32>,
    pub address_longitude: Option<f32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct StaffContactResponse {
    pub staff_id: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub address_latitude: Option<f32>,
    pub address_longitude: Option<f32>,
}

impl From<StaffContact> for StaffContactResponse {
    fn from(c: StaffContact) -> Self {
        Self {
            staff_id: c.staff_id,
            address: c.address,
            phone: c.phone,
            email: c.email,
            address_latitude: c.address_latitude,
            address_longitude: c.address_longitude,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateStaffContactRequest {
    pub staff_id: String,
    pub address: String,
    pub phone: String,
    pub email: String,
    pub address_latitude: Option<f32>,
    pub address_longitude: Option<f32>,
}

impl From<CreateStaffContactRequest> for StaffContact {
    fn from(req: CreateStaffContactRequest) -> Self {
        StaffContact {
            staff_id: req.staff_id,
            address: req.address,
            phone: req.phone,
            email: req.email,
            address_latitude: req.address_latitude,
            address_longitude: req.address_longitude,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = crate::schema::staff_contacts)]
pub struct UpdateStaffContactRequest {
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub address_latitude: Option<f32>,
    pub address_longitude: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct StaffContactQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StaffContactQuery {
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
#[diesel(table_name = crate::schema::staff_media)]
#[diesel(primary_key(staff_id))]
pub struct StaffMedia {
    pub staff_id: String,
    pub photo_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct StaffMediaResponse {
    pub staff_id: String,
    pub photo_url: Option<String>,
}

impl From<StaffMedia> for StaffMediaResponse {
    fn from(m: StaffMedia) -> Self {
        Self {
            staff_id: m.staff_id,
            photo_url: m.photo_url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateStaffMediaRequest {
    pub staff_id: String,
    pub photo_url: Option<String>,
}

impl From<CreateStaffMediaRequest> for StaffMedia {
    fn from(req: CreateStaffMediaRequest) -> Self {
        StaffMedia {
            staff_id: req.staff_id,
            photo_url: req.photo_url,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = crate::schema::staff_media)]
pub struct UpdateStaffMediaRequest {
    pub photo_url: Option<String>,
}

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
#[diesel(table_name = crate::schema::staff_reward_snapshots)]
#[diesel(primary_key(staff_id))]
pub struct StaffRewardSnapshot {
    pub staff_id: String,
    pub reward_points_balance: i32,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct StaffRewardSnapshotResponse {
    pub staff_id: String,
    pub reward_points_balance: i32,
    pub updated_at: NaiveDateTime,
}

impl From<StaffRewardSnapshot> for StaffRewardSnapshotResponse {
    fn from(s: StaffRewardSnapshot) -> Self {
        Self {
            staff_id: s.staff_id,
            reward_points_balance: s.reward_points_balance,
            updated_at: s.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateStaffRewardSnapshotRequest {
    pub staff_id: String,
    pub reward_points_balance: i32,
}

impl From<CreateStaffRewardSnapshotRequest> for StaffRewardSnapshot {
    fn from(req: CreateStaffRewardSnapshotRequest) -> Self {
        StaffRewardSnapshot {
            staff_id: req.staff_id,
            reward_points_balance: req.reward_points_balance,
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = crate::schema::staff_reward_snapshots)]
pub struct UpdateStaffRewardSnapshotRequest {
    pub reward_points_balance: Option<i32>,
}

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
#[diesel(table_name = crate::schema::staff_employment_status)]
#[diesel(primary_key(staff_id))]
pub struct StaffEmploymentStatus {
    pub staff_id: String,
    pub employment_status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StaffEmploymentStatusQuery {
    pub search: Option<String>,
    pub employment_status: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StaffEmploymentStatusQuery {
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
pub struct CreateStaffEmploymentStatusRequest {
    pub staff_id: String,
    pub employment_status: String,
}

impl From<CreateStaffEmploymentStatusRequest> for StaffEmploymentStatus {
    fn from(req: CreateStaffEmploymentStatusRequest) -> Self {
        StaffEmploymentStatus {
            staff_id: req.staff_id,
            employment_status: req.employment_status,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::staff_employment_status)]
pub struct UpdateStaffEmploymentStatusRequest {
    pub employment_status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StaffEmploymentStatusResponse {
    pub staff_id: String,
    pub employment_status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StaffEmploymentStatus> for StaffEmploymentStatusResponse {
    fn from(status: StaffEmploymentStatus) -> Self {
        Self {
            staff_id: status.staff_id,
            employment_status: status.employment_status,
            created_at: status.created_at,
            updated_at: status.updated_at,
        }
    }
}

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
#[diesel(table_name = crate::schema::staff_identity)]
#[diesel(primary_key(staff_id))]
pub struct StaffIdentity {
    pub staff_id: String,
    pub nic: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StaffIdentityQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StaffIdentityQuery {
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
pub struct CreateStaffIdentityRequest {
    pub staff_id: String,
    pub nic: String,
}

impl From<CreateStaffIdentityRequest> for StaffIdentity {
    fn from(req: CreateStaffIdentityRequest) -> Self {
        StaffIdentity {
            staff_id: req.staff_id,
            nic: req.nic,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::staff_identity)]
pub struct UpdateStaffIdentityRequest {
    pub nic: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StaffIdentityResponse {
    pub staff_id: String,
    pub nic: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StaffIdentity> for StaffIdentityResponse {
    fn from(identity: StaffIdentity) -> Self {
        Self {
            staff_id: identity.staff_id,
            nic: identity.nic,
            created_at: identity.created_at,
            updated_at: identity.updated_at,
        }
    }
}
