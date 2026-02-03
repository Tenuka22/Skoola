use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::student_attendance::{StudentAttendance, MarkStudentAttendanceRequest, StudentAttendanceResponse, BulkMarkStudentAttendanceRequest, UpdateStudentAttendanceRequest},
};
use actix_web::web;
use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use crate::schema::student_attendance;
use crate::database::enums::AttendanceStatus;

pub async fn bulk_mark_student_attendance(
    pool: web::Data<AppState>,
    bulk_request: BulkMarkStudentAttendanceRequest,
) -> Result<Vec<StudentAttendanceResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut marked_attendance_records = Vec::new();

    for record_request in bulk_request.attendance_records {
        let attendance_id = Uuid::new_v4().to_string();
        let new_attendance = StudentAttendance {
            id: attendance_id,
            student_id: record_request.student_id,
            class_id: record_request.class_id,
            date: record_request.date,
            status: record_request.status,
            marked_by: record_request.marked_by,
            remarks: record_request.remarks,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        diesel::insert_into(student_attendance::table)
            .values(&new_attendance)
            .execute(&mut conn)?;
        
        marked_attendance_records.push(StudentAttendanceResponse::from(new_attendance));
    }

    Ok(marked_attendance_records)
}

pub async fn mark_individual_student_attendance(
    pool: web::Data<AppState>,
    record_request: MarkStudentAttendanceRequest,
) -> Result<StudentAttendanceResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let attendance_id = Uuid::new_v4().to_string();
    let new_attendance = StudentAttendance {
        id: attendance_id,
        student_id: record_request.student_id,
        class_id: record_request.class_id,
        date: record_request.date,
        status: record_request.status,
        marked_by: record_request.marked_by,
        remarks: record_request.remarks,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(student_attendance::table)
        .values(&new_attendance)
        .execute(&mut conn)?;

    Ok(StudentAttendanceResponse::from(new_attendance))
}

pub async fn update_student_attendance(
    pool: web::Data<AppState>,
    attendance_id: String,
    update_request: UpdateStudentAttendanceRequest,
) -> Result<StudentAttendanceResponse, APIError> {
    let mut conn = pool.db_pool.get()?;

    let target = student_attendance::table.filter(student_attendance::id.eq(&attendance_id));

    let updated_count = diesel::update(target)
        .set((update_request, student_attendance::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    if updated_count == 0 {
        return Err(APIError::not_found(&format!("Attendance record with ID {} not found", attendance_id)));
    }

    let updated_record: StudentAttendance = student_attendance::table
        .filter(student_attendance::id.eq(&attendance_id))
        .first(&mut conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => APIError::not_found(&format!("Attendance record with ID {} not found", attendance_id)),
            _ => APIError::internal(&e.to_string()),
        })?;
    
    Ok(StudentAttendanceResponse::from(updated_record))
}

pub async fn get_attendance_by_class_and_date(
    pool: web::Data<AppState>,
    class_id: String,
    date: NaiveDate,
) -> Result<Vec<StudentAttendanceResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let attendance_records: Vec<StudentAttendance> = student_attendance::table
        .filter(student_attendance::class_id.eq(&class_id))
        .filter(student_attendance::date.eq(&date))
        .load::<StudentAttendance>(&mut conn)?;
    
    let responses: Vec<StudentAttendanceResponse> = attendance_records
        .into_iter()
        .map(StudentAttendanceResponse::from)
        .collect();

    Ok(responses)
}

pub async fn get_attendance_by_student(
    pool: web::Data<AppState>,
    student_id: String,
    from_date: Option<NaiveDate>,
    to_date: Option<NaiveDate>,
) -> Result<Vec<StudentAttendanceResponse>, APIError> {
    let mut conn = pool.db_pool.get()?;

    let mut query = student_attendance::table
        .filter(student_attendance::student_id.eq(&student_id))
        .into_boxed();

    if let Some(f_date) = from_date {
        query = query.filter(student_attendance::date.ge(f_date));
    }
    if let Some(t_date) = to_date {
        query = query.filter(student_attendance::date.le(t_date));
    }

    let attendance_records: Vec<StudentAttendance> = query
        .order(student_attendance::date.desc())
        .load::<StudentAttendance>(&mut conn)?;

    let responses: Vec<StudentAttendanceResponse> = attendance_records
        .into_iter()
        .map(StudentAttendanceResponse::from)
        .collect();

    Ok(responses)
}

pub async fn calculate_attendance_percentage(
    pool: web::Data<AppState>,
    student_id: String,
    from_date: NaiveDate,
    to_date: NaiveDate,
) -> Result<f64, APIError> {
    let mut conn = pool.db_pool.get()?;

    let total_days = (to_date - from_date).num_days() + 1;

    if total_days <= 0 {
        return Err(APIError::bad_request("To date must be after or equal to from date"));
    }

    let present_days = student_attendance::table
        .filter(student_attendance::student_id.eq(&student_id))
        .filter(student_attendance::date.ge(from_date))
        .filter(student_attendance::date.le(to_date))
        .filter(student_attendance::status.eq(AttendanceStatus::Present))
        .count()
        .get_result::<i64>(&mut conn)?;
    
    let percentage = (present_days as f64 / total_days as f64) * 100.0;

    Ok(percentage)
}