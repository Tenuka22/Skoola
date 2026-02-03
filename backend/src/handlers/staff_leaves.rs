use actix_web::{web, HttpResponse};
use apistos::api_operation;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;

use crate::{
    AppState,
    database::tables::StaffLeave,
    errors::APIError,
    models::staff_leaves::{ApplyLeaveRequest, ApproveRejectLeaveRequest, StaffLeaveResponse, StaffLeaveChangeset},
    schema::staff_leaves,
    database::enums::LeaveStatus,
};

#[api_operation(
    summary = "Apply for leave",
    description = "Allows a staff member to apply for leave.",
    tag = "staff_leaves"
)]
pub async fn apply_for_leave(
    data: web::Data<AppState>,
    staff_id: web::Path<String>,
    body: web::Json<ApplyLeaveRequest>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let staff_id_inner = staff_id.into_inner();

    if body.from_date > body.to_date {
        return Err(APIError::bad_request("From date cannot be after to date"));
    }

    let new_leave = StaffLeave {
        id: Uuid::new_v4().to_string(),
        staff_id: staff_id_inner,
        leave_type: body.leave_type.clone(),
        from_date: body.from_date,
        to_date: body.to_date,
        reason: body.reason.clone(),
        status: LeaveStatus::Pending.to_string(), // New leave applications are pending by default
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(staff_leaves::table)
        .values(&new_leave)
        .execute(&mut conn)?;

    Ok(HttpResponse::Created().json(StaffLeaveResponse::from(new_leave)))
}

#[api_operation(
    summary = "Approve or reject staff leave",
    description = "Approves or rejects a pending staff leave application.",
    tag = "staff_leaves"
)]
pub async fn approve_reject_leave(
    data: web::Data<AppState>,
    leave_id: web::Path<String>,
    body: web::Json<ApproveRejectLeaveRequest>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let leave_id_inner = leave_id.into_inner();

    // Check if the leave application exists
    let existing_leave: StaffLeave = staff_leaves::table
        .find(&leave_id_inner)
        .select(StaffLeave::as_select())
        .first(&mut conn)?;

    if existing_leave.status.parse::<LeaveStatus>().expect("Invalid LeaveStatus in DB") != LeaveStatus::Pending {
        return Err(APIError::bad_request("Only pending leave applications can be approved or rejected"));
    }

    let changeset = StaffLeaveChangeset {
        status: Some(body.status.to_string()),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::update(staff_leaves::table.find(&leave_id_inner))
        .set(changeset)
        .execute(&mut conn)?;

    let updated_leave = staff_leaves::table
        .find(&leave_id_inner)
        .select(StaffLeave::as_select())
        .first::<StaffLeave>(&mut conn)?;

    Ok(HttpResponse::Ok().json(StaffLeaveResponse::from(updated_leave)))
}
