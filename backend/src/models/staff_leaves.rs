use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use chrono::{NaiveDate, NaiveDateTime};

use crate::database::enums::LeaveStatus;
use crate::database::tables::StaffLeave;
use diesel::AsChangeset;
use crate::schema::staff_leaves;

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct ApplyLeaveRequest {
    pub leave_type: String,
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
    pub leave_type: String,
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
    pub reason: String,
    pub status: LeaveStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, AsChangeset, Serialize, Deserialize, ApiComponent, JsonSchema)]
#[diesel(table_name = staff_leaves)]
pub struct StaffLeaveChangeset {
    pub status: Option<String>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct LeaveBalanceResponse {
    pub staff_id: String,
    pub leave_type: String,
    pub total_days_taken: i64,
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
            status: staff_leave.status.parse().unwrap_or(LeaveStatus::Pending),
            created_at: staff_leave.created_at,
            updated_at: staff_leave.updated_at,
        }
    }
}

