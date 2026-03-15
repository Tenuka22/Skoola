use crate::{
    AppState, database::enums::{LeaveStatus, StaffLeaveType}, errors::APIError,
    models::staff::leave::{LeaveBalanceResponse, StaffLeave, ApplyLeaveRequest},
    schema::staff_leaves,
    models::ids::{generate_prefixed_id, IdPrefix},
};
use actix_web::web;
use diesel::prelude::*;
use chrono::Utc;

pub async fn get_staff_leave_balance(
    pool: web::Data<AppState>,
    staff_id: String,
) -> Result<Vec<LeaveBalanceResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let approved_leaves: Vec<StaffLeave> = staff_leaves::table
        .filter(staff_leaves::staff_id.eq(&staff_id))
        .filter(staff_leaves::status.eq(LeaveStatus::Approved))
        .load::<StaffLeave>(&mut conn)?;

    let mut leave_balances: std::collections::HashMap<StaffLeaveType, i64> =
        std::collections::HashMap::new();

    for leave in approved_leaves {
        let from_date = leave.from_date;
        let to_date = leave.to_date;

        let num_days = (to_date - from_date).num_days() + 1; // +1 to include both start and end dates

        *leave_balances.entry(leave.leave_type).or_insert(0) += num_days;
    }

    let result: Vec<LeaveBalanceResponse> = leave_balances
        .into_iter()
        .map(|(leave_type, total_days_taken)| LeaveBalanceResponse {
            staff_id: staff_id.clone(),
            leave_type,
            leave_type_id: "".to_string(),
            leave_type_name: "".to_string(),
            annual_quota: 0.0,
            used_days: total_days_taken as f32,
            remaining_days: 0.0,
            total_days_taken,
        })
        .collect();

    Ok(result)
}

pub async fn apply_for_leave(
    pool: web::Data<AppState>,
    staff_id: String,
    req: ApplyLeaveRequest,
) -> Result<StaffLeave, APIError> {
    let mut conn = pool.db_pool.get()?;
    let id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;
    let now = Utc::now().naive_utc();

    let new_leave = StaffLeave {
        id,
        staff_id,
        leave_type: req.leave_type,
        from_date: req.from_date,
        to_date: req.to_date,
        reason: req.reason,
        status: LeaveStatus::Pending,
        created_at: now,
        updated_at: now,
    };

    diesel::insert_into(staff_leaves::table)
        .values(&new_leave)
        .execute(&mut conn)?;

    Ok(new_leave)
}

pub async fn approve_reject_leave(
    pool: web::Data<AppState>,
    leave_id: String,
    status: LeaveStatus,
) -> Result<StaffLeave, APIError> {
    let mut conn = pool.db_pool.get()?;
    let now = Utc::now().naive_utc();

    diesel::update(staff_leaves::table.find(&leave_id))
        .set((
            staff_leaves::status.eq(status),
            staff_leaves::updated_at.eq(now),
        ))
        .execute(&mut conn)?;

    let updated = staff_leaves::table.find(leave_id).first::<StaffLeave>(&mut conn)?;
    Ok(updated)
}
