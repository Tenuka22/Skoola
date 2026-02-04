use actix_web::web;
use apistos::api_operation;
use chrono::NaiveDate;
use serde::Deserialize;
use schemars::JsonSchema;
use apistos::ApiComponent;
use actix_web::web::Json;

use crate::{
    AppState,
    errors::APIError,
    models::student_attendance::{
        BulkMarkStudentAttendanceRequest, MarkStudentAttendanceRequest, UpdateStudentAttendanceRequest,
        GetAttendanceByClassAndDatePath, GetAttendanceByStudentPath, GenerateAttendanceReportRequest,
        LowAttendanceStudentQuery, SendAbsenceNotificationRequest, StudentAttendanceResponse, StudentAttendanceReportResponse
    },
    models::MessageResponse,
    services::student_attendance,
};

#[api_operation(
    summary = "Bulk mark student attendance by class",
    description = "Marks attendance for multiple students in a class.",
    tag = "student_attendance"
)]
pub async fn bulk_mark_student_attendance(
    data: web::Data<AppState>,
    body: web::Json<BulkMarkStudentAttendanceRequest>,
) -> Result<Json<Vec<StudentAttendanceResponse>>, APIError> {
    let marked_records = student_attendance::bulk_mark_student_attendance(data.clone(), body.into_inner()).await?;
    Ok(Json(marked_records))
}

#[api_operation(
    summary = "Mark individual student attendance",
    description = "Marks attendance for a single student.",
    tag = "student_attendance"
)]
pub async fn mark_individual_student_attendance(
    data: web::Data<AppState>,
    body: web::Json<MarkStudentAttendanceRequest>,
) -> Result<Json<StudentAttendanceResponse>, APIError> {
    let marked_record = student_attendance::mark_individual_student_attendance(data.clone(), body.into_inner()).await?;
    Ok(Json(marked_record))
}

#[api_operation(
    summary = "Update student attendance record",
    description = "Updates an existing student attendance record.",
    tag = "student_attendance"
)]
pub async fn update_student_attendance(
    data: web::Data<AppState>,
    path: web::Path<String>, // attendance_id
    body: web::Json<UpdateStudentAttendanceRequest>,
) -> Result<Json<StudentAttendanceResponse>, APIError> {
    let attendance_id = path.into_inner();
    let updated_record = student_attendance::update_student_attendance(data.clone(), attendance_id, body.into_inner()).await?;
    Ok(Json(updated_record))
}

#[api_operation(
    summary = "Get attendance by class and date",
    description = "Retrieves attendance records for a specific class on a given date.",
    tag = "student_attendance"
)]
pub async fn get_attendance_by_class_and_date(
    data: web::Data<AppState>,
    path: web::Path<GetAttendanceByClassAndDatePath>, // (class_id, date)
) -> Result<Json<Vec<StudentAttendanceResponse>>, APIError> {
    let path_params = path.into_inner();
    let records = student_attendance::get_attendance_by_class_and_date(data.clone(), path_params.class_id, path_params.date).await?;
    Ok(Json(records))
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct GetAttendanceByStudentQuery {
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[api_operation(
    summary = "Get attendance by student",
    description = "Retrieves attendance records for a specific student, optionally filtered by date range.",
    tag = "student_attendance"
)]
pub async fn get_attendance_by_student(
    data: web::Data<AppState>,
    path: web::Path<GetAttendanceByStudentPath>, // student_id
    web::Query(query): web::Query<GetAttendanceByStudentQuery>,
) -> Result<Json<Vec<StudentAttendanceResponse>>, APIError> {
    let path_params = path.into_inner();
    let records = student_attendance::get_attendance_by_student(
        data.clone(),
        path_params.student_id,
        query.from_date,
        query.to_date,
    )
    .await?;
    Ok(Json(records))
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct CalculateAttendancePercentageQuery {
    pub from_date: NaiveDate,
    pub to_date: NaiveDate,
}

#[api_operation(
    summary = "Calculate student attendance percentage",
    description = "Calculates the attendance percentage for a student within a specified date range.",
    tag = "student_attendance"
)]
pub async fn calculate_student_attendance_percentage(
    data: web::Data<AppState>,
    path: web::Path<String>, // student_id
    web::Query(query): web::Query<CalculateAttendancePercentageQuery>,
) -> Result<Json<f64>, APIError> {
    let student_id = path.into_inner();
    let percentage = student_attendance::calculate_attendance_percentage(
        data.clone(),
        student_id,
        query.from_date,
        query.to_date,
    )
    .await?;
    Ok(Json(percentage))
}

#[api_operation(
    summary = "Generate attendance report for a class",
    description = "Generates a detailed attendance report for all students in a specific class within a date range.",
    tag = "student_attendance"
)]
pub async fn generate_attendance_report(
    data: web::Data<AppState>,
    web::Query(query): web::Query<GenerateAttendanceReportRequest>,
) -> Result<Json<Vec<StudentAttendanceReportResponse>>, APIError> {
    let report = student_attendance::generate_attendance_report(data.clone(), query).await?;
    Ok(Json(report))
}

#[api_operation(
    summary = "Get students with low attendance",
    description = "Retrieves a list of students in a class with attendance percentage below a specified threshold.",
    tag = "student_attendance"
)]
pub async fn get_students_with_low_attendance(
    data: web::Data<AppState>,
    web::Query(query): web::Query<LowAttendanceStudentQuery>,
) -> Result<Json<Vec<StudentAttendanceReportResponse>>, APIError> {
    let students = student_attendance::get_students_with_low_attendance(data.clone(), query).await?;
    Ok(Json(students))
}



#[api_operation(
    summary = "Send absence notifications to parents",
    description = "Sends email notifications to parents of absent students for a specific class on a given date.",
    tag = "student_attendance"
)]
pub async fn send_absence_notifications(
    data: web::Data<AppState>,
    body: web::Json<SendAbsenceNotificationRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    student_attendance::send_absence_notifications(data.clone(), body.class_id.clone(), body.date).await?;
    Ok(Json(MessageResponse { message: "Absence notifications process initiated.".to_string() }))
}