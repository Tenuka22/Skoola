use diesel::prelude::*;
use crate::{
    database::tables::StaffLeave,
    schema::staff_leaves,
    errors::APIError,
    AppState,
    models::staff_leaves::LeaveBalanceResponse,
    database::enums::LeaveStatus,
};
use actix_web::web;

pub async fn get_staff_leave_balance(
    pool: web::Data<AppState>,
    staff_id: String,
) -> Result<Vec<LeaveBalanceResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let approved_leaves: Vec<StaffLeave> = staff_leaves::table
        .filter(staff_leaves::staff_id.eq(&staff_id))
        .filter(staff_leaves::status.eq(LeaveStatus::Approved.to_string()))
        .load::<StaffLeave>(&mut conn)?;

    let mut leave_balances: std::collections::HashMap<String, i64> = std::collections::HashMap::new();

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
            total_days_taken,
        })
        .collect();

    Ok(result)
}
