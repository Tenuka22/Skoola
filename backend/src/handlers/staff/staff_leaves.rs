use actix_web::web;
use actix_web::web::Json;
use apistos::api_operation;

use crate::{
    AppState,
    errors::APIError,
    models::staff::leave::{
        ApplyLeaveRequest, ApproveRejectLeaveRequest, StaffLeaveResponse,
        LeaveBalanceResponse,
    },
    services::staff::staff_leaves,
};

#[api_operation(
    summary = "Apply for leave",
    description = "Allows a staff member to submit a leave request.",
    tag = "staff_leaves",
    operation_id = "apply_for_leave"
)]
pub async fn apply_for_leave(
    data: web::Data<AppState>,
    staff_id: web::Path<String>,
    body: Json<ApplyLeaveRequest>,
) -> Result<Json<StaffLeaveResponse>, APIError> {
    let res = staff_leaves::apply_for_leave(data, staff_id.into_inner(), body.into_inner()).await?;
    Ok(Json(StaffLeaveResponse::from(res)))
}

#[api_operation(
    summary = "Approve or reject leave",
    description = "Allows an admin or supervisor to approve or reject a pending leave request.",
    tag = "staff_leaves",
    operation_id = "approve_reject_leave"
)]
pub async fn approve_reject_leave(
    data: web::Data<AppState>,
    leave_id: web::Path<String>,
    body: Json<ApproveRejectLeaveRequest>,
) -> Result<Json<StaffLeaveResponse>, APIError> {
    let res = staff_leaves::approve_reject_leave(data, leave_id.into_inner(), body.into_inner().status).await?;
    Ok(Json(StaffLeaveResponse::from(res)))
}

#[api_operation(
    summary = "View leave balance",
    description = "Retrieves the current leave balance for a specific staff member.",
    tag = "staff_leaves",
    operation_id = "view_leave_balance"
)]
pub async fn view_leave_balance(
    data: web::Data<AppState>,
    staff_id: web::Path<String>,
) -> Result<Json<Vec<LeaveBalanceResponse>>, APIError> {
    let staff_id_inner = staff_id.into_inner();
    let leave_balances = staff_leaves::get_staff_leave_balance(data, staff_id_inner).await?;
    Ok(Json(leave_balances))
}

