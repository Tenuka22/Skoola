use actix_web::web;
use apistos::api_operation;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::{Utc, NaiveDate, Days};
use actix_web::web::Json;
// use serde_json; // Removed unused import

use crate::{
    AppState,
    database::tables::StaffAttendance,
    errors::APIError,
    models::staff_attendance::{MarkStaffAttendanceRequest, BulkMarkStaffAttendanceRequest, StaffAttendanceResponse, UpdateStaffAttendanceRequest, StaffAttendanceChangeset, StaffAttendanceDateQuery, StaffAttendanceByStaffQuery, MonthlyAttendancePercentageResponse},
    schema::staff_attendance,
};

#[api_operation(
    summary = "Mark daily staff attendance",
    description = "Marks attendance for a single staff member for a specific date.",
    tag = "staff_attendance",
    operation_id = "mark_staff_attendance_daily"
)]
pub async fn mark_staff_attendance_daily(
    data: web::Data<AppState>,
    staff_id: web::Path<String>,
    body: web::Json<MarkStaffAttendanceRequest>,
) -> Result<Json<StaffAttendanceResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let staff_id_inner = staff_id.into_inner();

    // Check for existing attendance record for the same staff and date
    let existing_attendance: Option<StaffAttendance> = staff_attendance::table
        .filter(staff_attendance::staff_id.eq(&staff_id_inner))
        .filter(staff_attendance::date.eq(&body.date))
        .select(StaffAttendance::as_select())
        .first(&mut conn).optional()?;

    if existing_attendance.is_some() {
        return Err(APIError::conflict("Attendance already marked for this staff member on this date"));
    }

    let new_attendance = StaffAttendance {
        id: Uuid::new_v4().to_string(),
        staff_id: staff_id_inner.clone(),
        date: body.date,
        status: body.status.to_string(),
        time_in: body.time_in,
        time_out: body.time_out,
        remarks: body.remarks.clone(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(staff_attendance::table)
        .values(&new_attendance)
        .execute(&mut conn)?;

    Ok(Json(StaffAttendanceResponse::from(new_attendance)))
}

#[api_operation(
    summary = "Mark bulk staff attendance",
    description = "Marks attendance for multiple staff members for a specific date.",
    tag = "staff_attendance",
    operation_id = "mark_bulk_staff_attendance"
)]
pub async fn mark_bulk_staff_attendance(
    data: web::Data<AppState>,
    body: web::Json<BulkMarkStaffAttendanceRequest>,
) -> Result<Json<Vec<StaffAttendanceResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let mut new_attendance_records = Vec::new();
    let mut conflicts = Vec::new();

    for item in body.attendance_records.iter() {
        // Check for existing attendance record for the same staff and date
        let existing_attendance: Option<StaffAttendance> = staff_attendance::table
            .filter(staff_attendance::staff_id.eq(&item.staff_id))
            .filter(staff_attendance::date.eq(&body.date))
            .select(StaffAttendance::as_select())
            .first(&mut conn).optional()?;

        if existing_attendance.is_some() {
            conflicts.push(item.staff_id.clone());
        } else {
            new_attendance_records.push(StaffAttendance {
                id: Uuid::new_v4().to_string(),
                staff_id: item.staff_id.clone(),
                date: body.date,
                status: item.status.to_string(),
                time_in: item.time_in,
                time_out: item.time_out,
                remarks: item.remarks.clone(),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            });
        }
    }

    if !new_attendance_records.is_empty() {
        diesel::insert_into(staff_attendance::table)
            .values(&new_attendance_records)
            .execute(&mut conn)?;
    }

    if conflicts.is_empty() {
        Ok(Json(new_attendance_records.into_iter().map(StaffAttendanceResponse::from).collect::<Vec<_>>()))
    } else {
        let error_message = format!("Attendance already marked for staff IDs: {:?}", conflicts);
        Err(APIError::conflict(&error_message))
    }
}

#[api_operation(
    summary = "Update staff attendance record",
    description = "Updates an existing staff attendance record by ID.",
    tag = "staff_attendance",
    operation_id = "update_staff_attendance"
)]
pub async fn update_staff_attendance(
    data: web::Data<AppState>,
    attendance_id: web::Path<String>,
    body: web::Json<UpdateStaffAttendanceRequest>,
) -> Result<Json<StaffAttendanceResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let attendance_id_inner = attendance_id.into_inner();

    let changeset = StaffAttendanceChangeset {
        status: body.status.as_ref().map(|s| s.to_string()),
        time_in: body.time_in,
        time_out: body.time_out,
        remarks: body.remarks.clone(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::update(staff_attendance::table.find(&attendance_id_inner))
        .set(changeset)
        .execute(&mut conn)?;

    let updated_attendance = staff_attendance::table
        .find(&attendance_id_inner)
        .select(StaffAttendance::as_select())
        .first::<StaffAttendance>(&mut conn)?;

    Ok(Json(StaffAttendanceResponse::from(updated_attendance)))
}

#[api_operation(
    summary = "View staff attendance by date",
    description = "Returns a list of staff attendance records for a specific date.",
    tag = "staff_attendance",
    operation_id = "get_staff_attendance_by_date"
)]
pub async fn get_staff_attendance_by_date(
    data: web::Data<AppState>,
    query: web::Query<StaffAttendanceDateQuery>,
) -> Result<Json<Vec<StaffAttendanceResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;

    let attendance_list = staff_attendance::table
        .filter(staff_attendance::date.eq(&query.date))
        .select(StaffAttendance::as_select())
        .load::<StaffAttendance>(&mut conn)?;

    Ok(Json(attendance_list.into_iter().map(StaffAttendanceResponse::from).collect::<Vec<_>>()))
}

#[api_operation(
    summary = "View staff attendance by staff member",
    description = "Returns a list of staff attendance records for a specific staff member, optionally filtered by a date range.",
    tag = "staff_attendance",
    operation_id = "get_staff_attendance_by_staff_member"
)]
pub async fn get_staff_attendance_by_staff_member(
    data: web::Data<AppState>,
    staff_id: web::Path<String>,
    query: web::Query<StaffAttendanceByStaffQuery>,
) -> Result<Json<Vec<StaffAttendanceResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let staff_id_inner = staff_id.into_inner();
    let mut attendance_query = staff_attendance::table.into_boxed();

    attendance_query = attendance_query.filter(staff_attendance::staff_id.eq(&staff_id_inner));

    if let Some(start_date) = query.start_date {
        attendance_query = attendance_query.filter(staff_attendance::date.ge(start_date));
    }
    if let Some(end_date) = query.end_date {
        attendance_query = attendance_query.filter(staff_attendance::date.le(end_date));
    }

    let attendance_list = attendance_query
        .select(StaffAttendance::as_select())
        .load::<StaffAttendance>(&mut conn)?;

    Ok(Json(attendance_list.into_iter().map(StaffAttendanceResponse::from).collect::<Vec<_>>()))
}

#[api_operation(
    summary = "Calculate monthly attendance percentage",
    description = "Calculates the attendance percentage for a staff member for a given month and year.",
    tag = "staff_attendance",
    operation_id = "calculate_monthly_staff_attendance_percentage"
)]
pub async fn calculate_monthly_attendance_percentage(
    data: web::Data<AppState>,
    staff_id: web::Path<String>,
    path_info: web::Path<(i32, u32)>, // (year, month)
) -> Result<Json<MonthlyAttendancePercentageResponse>, APIError> {
    let (year, month) = path_info.into_inner();
    let staff_id_inner = staff_id.into_inner();
    let mut conn = data.db_pool.get()?;

    // Determine the start and end dates of the month
    let start_of_month = NaiveDate::from_ymd_opt(year, month, 1)
        .ok_or_else(|| APIError::bad_request("Invalid month or year"))?;
    let end_of_month = NaiveDate::from_ymd_opt(year, month, 1)
        .and_then(|d| d.checked_add_months(chrono::Months::new(1)))
        .and_then(|d| d.checked_sub_days(Days::new(1)))
        .ok_or_else(|| APIError::internal("Could not determine end of month"))?;

    // Filter attendance records for the given staff, month, and year
    let attendance_records = staff_attendance::table
        .filter(staff_attendance::staff_id.eq(&staff_id_inner))
        .filter(staff_attendance::date.ge(start_of_month))
        .filter(staff_attendance::date.le(end_of_month))
        .select(StaffAttendance::as_select())
        .load::<StaffAttendance>(&mut conn)?;

    let present_days = attendance_records
        .iter()
        .filter(|rec| matches!(rec.status.parse::<crate::database::enums::AttendanceStatus>(), Ok(crate::database::enums::AttendanceStatus::Present)))
        .count() as i64;

    let total_working_days = attendance_records.len() as i64;

    let attendance_percentage = if total_working_days > 0 {
        (present_days as f64 / total_working_days as f64) * 100.0
    } else {
        0.0
    };

    Ok(Json(MonthlyAttendancePercentageResponse {
        staff_id: staff_id_inner,
        month,
        year,
        present_days,
        total_working_days,
        attendance_percentage,
    }))
}
