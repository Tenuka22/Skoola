use crate::database::enums::{LeaveStatus, StaffLeaveType};
use crate::models::staff::staff::Staff;
use crate::schema::{staff_leaves, staff_leave_requests, staff_leave_types};
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
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
#[diesel(table_name = staff_leaves)]
#[diesel(belongs_to(Staff))]
pub struct StaffLeave {
    pub id: String,
    pub staff_id: String,
    pub leave_type: StaffLeaveType,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub reason: String,
    pub status: LeaveStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct ApplyLeaveRequest {
    pub leave_type: StaffLeaveType,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct ApproveRejectLeaveRequest {
    pub status: LeaveStatus,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct StaffLeaveResponse {
    pub id: String,
    pub staff_id: String,
    pub leave_type: StaffLeaveType,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub reason: String,
    pub status: LeaveStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StaffLeave> for StaffLeaveResponse {
    fn from(staff_leave: StaffLeave) -> Self {
        StaffLeaveResponse {
            id: staff_leave.id,
            staff_id: staff_leave.staff_id,
            leave_type: staff_leave.leave_type,
            from_date: staff_leave.from_date,
            to_date: staff_leave.to_date,
            reason: staff_leave.reason,
            status: staff_leave.status,
            created_at: staff_leave.created_at,
            updated_at: staff_leave.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct LeaveBalanceResponse {
    pub staff_id: String,
    pub leave_type: StaffLeaveType,
    pub leave_type_id: String,
    pub leave_type_name: String,
    pub annual_quota: f32,
    pub used_days: f32,
    pub remaining_days: f32,
    pub total_days_taken: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = staff_leaves)]
pub struct StaffLeaveChangeset {
    pub leave_type: Option<StaffLeaveType>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
    pub reason: Option<String>,
    pub status: Option<LeaveStatus>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StaffLeaveRequestQuery {
    pub search: Option<String>,
    pub staff_id: Option<String>,
    pub leave_type_id: Option<String>,
    pub status: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StaffLeaveRequestQuery {
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

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = staff_leave_requests)]
pub struct StaffLeaveRequest {
    pub id: String,
    pub staff_id: String,
    pub leave_type_id: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: Option<String>,
    pub status: String,
    pub approved_by: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StaffLeaveRequestResponse {
    pub id: String,
    pub staff_id: String,
    pub leave_type_id: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: Option<String>,
    pub status: String,
    pub approved_by: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StaffLeaveRequest> for StaffLeaveRequestResponse {
    fn from(request: StaffLeaveRequest) -> Self {
        Self {
            id: request.id,
            staff_id: request.staff_id,
            leave_type_id: request.leave_type_id,
            start_date: request.start_date,
            end_date: request.end_date,
            reason: request.reason,
            status: request.status,
            approved_by: request.approved_by,
            created_at: request.created_at,
            updated_at: request.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = staff_leave_requests)]
pub struct CreateStaffLeaveRequest {
    pub staff_id: String,
    pub leave_type_id: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = staff_leave_requests)]
pub struct UpdateStaffLeaveRequest {
    pub status: Option<String>,
    pub approved_by: Option<String>,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StaffLeaveTypeQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StaffLeaveTypeQuery {
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

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = staff_leave_types)]
pub struct StaffLeaveTypeModel {
    pub id: String,
    pub name: String,
    pub annual_quota: f32,
    pub requires_approval: bool,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, Clone)]
pub struct StaffLeaveTypeResponse {
    pub id: String,
    pub name: String,
    pub annual_quota: f32,
    pub requires_approval: bool,
    pub created_at: NaiveDateTime,
}

impl From<StaffLeaveTypeModel> for StaffLeaveTypeResponse {
    fn from(model: StaffLeaveTypeModel) -> Self {
        Self {
            id: model.id,
            name: model.name,
            annual_quota: model.annual_quota,
            requires_approval: model.requires_approval,
            created_at: model.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = staff_leave_types)]
pub struct CreateStaffLeaveTypeRequest {
    pub name: String,
    pub annual_quota: f32,
    pub requires_approval: bool,
}

#[derive(Debug, Default, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct UpdateStaffLeaveTypeRequest {
    pub name: Option<String>,
    pub annual_quota: Option<f32>,
    pub requires_approval: Option<bool>,
}
