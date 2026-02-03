use actix_web::{web, HttpResponse};
use apistos::api_operation;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;

use crate::{
    AppState,
    database::tables::StaffAttendance,
    errors::APIError,
    models::staff_attendance::{MarkStaffAttendanceRequest, BulkMarkStaffAttendanceRequest, StaffAttendanceResponse, BulkMarkStaffAttendanceItem, UpdateStaffAttendanceRequest, StaffAttendanceChangeset, StaffAttendanceDateQuery},
    schema::staff_attendance,
};

#[api_operation(
    summary = "Mark daily staff attendance",
    description = "Marks attendance for a single staff member for a specific date.",
    tag = "staff_attendance"
)]
pub async fn mark_staff_attendance_daily(
    data: web::Data<AppState>,
    staff_id: web::Path<String>,
    body: web::Json<MarkStaffAttendanceRequest>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let staff_id_inner = staff_id.into_inner();

    // Check for existing attendance record for the same staff and date
    let existing_attendance: Option<StaffAttendance> = staff_attendance::table
        .filter(staff_attendance::staff_id.eq(&staff_id_inner))
        .filter(staff_attendance::date.eq(&body.date))
        .select(StaffAttendance::as_select())
        .first(&mut conn)
        .optional()?;

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

    Ok(HttpResponse::Created().json(StaffAttendanceResponse::from(new_attendance)))
}

#[api_operation(
    summary = "Mark bulk staff attendance",
    description = "Marks attendance for multiple staff members for a specific date.",
    tag = "staff_attendance"
)]
pub async fn mark_bulk_staff_attendance(
    data: web::Data<AppState>,
    body: web::Json<BulkMarkStaffAttendanceRequest>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let mut new_attendance_records = Vec::new();
    let mut conflicts = Vec::new();

    for item in body.attendance_records.iter() {
        // Check for existing attendance record for the same staff and date
        let existing_attendance: Option<StaffAttendance> = staff_attendance::table
            .filter(staff_attendance::staff_id.eq(&item.staff_id))
            .filter(staff_attendance::date.eq(&body.date))
            .select(StaffAttendance::as_select())
            .first(&mut conn)
            .optional()?;

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
        Ok(HttpResponse::Created().json(new_attendance_records.into_iter().map(StaffAttendanceResponse::from).collect::<Vec<_>>()))
    } else {
        let error_message = format!("Attendance already marked for staff IDs: {:?}", conflicts);
        Err(APIError::conflict(&error_message))
    }
}

#[api_operation(
    summary = "Update staff attendance record",
    description = "Updates an existing staff attendance record by ID.",
    tag = "staff_attendance"
)]
pub async fn update_staff_attendance(
    data: web::Data<AppState>,
    attendance_id: web::Path<String>,
    body: web::Json<UpdateStaffAttendanceRequest>,
) -> Result<HttpResponse, APIError> {
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

    Ok(HttpResponse::Ok().json(StaffAttendanceResponse::from(updated_attendance)))
}

#[api_operation(
    summary = "View staff attendance by date",
    description = "Returns a list of staff attendance records for a specific date.",
    tag = "staff_attendance"
)]
pub async fn get_staff_attendance_by_date(
    data: web::Data<AppState>,
    query: web::Query<StaffAttendanceDateQuery>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;

    let attendance_list = staff_attendance::table
        .filter(staff_attendance::date.eq(&query.date))
        .select(StaffAttendance::as_select())
        .load::<StaffAttendance>(&mut conn)?;

    Ok(HttpResponse::Ok().json(attendance_list.into_iter().map(StaffAttendanceResponse::from).collect::<Vec<_>>()))
}
