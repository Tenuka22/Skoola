use crate::AppState;
use crate::errors::APIError;
use crate::models::staff::attendance::{
    BulkMarkStaffAttendanceRequest, MarkStaffAttendanceRequest, StaffAttendanceResponse,
    SubstitutionResponse, UpdateStaffAttendanceRequest,
    MarkTeacherPeriodAttendanceRequest, TeacherPeriodAttendanceResponse
};
use crate::models::curriculum_management::{CreateLessonProgressRequest, LessonProgress};
use crate::services::staff::staff_attendance::{self, CreateSubstitutionRequest};
use crate::utils::jwt::UserId;
use actix_web::HttpRequest;
use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::web::Path;
use apistos::api_operation;
use chrono::NaiveDate;

#[api_operation(
    summary = "Mark teacher period attendance",
    description = "Marks attendance for a teacher for a specific period in the timetable.",
    tag = "staff_attendance",
    operation_id = "mark_teacher_period_attendance"
)]
pub async fn mark_teacher_period_attendance(
    data: Data<AppState>,
    req: HttpRequest,
    body: Json<MarkTeacherPeriodAttendanceRequest>,
) -> Result<Json<TeacherPeriodAttendanceResponse>, APIError> {
    let marker_id = UserId::from_request(&req)?;
    let res = staff_attendance::mark_period_attendance(
        data,
        body.into_inner(),
        marker_id.0,
    )
    .await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Mark staff attendance daily",
    description = "Records daily attendance for a staff member.",
    tag = "staff_attendance",
    operation_id = "mark_staff_attendance_daily"
)]
pub async fn mark_staff_attendance_daily(
    data: Data<AppState>,
    req: HttpRequest,
    path: Path<String>, // staff_id
    body: Json<MarkStaffAttendanceRequest>,
) -> Result<Json<StaffAttendanceResponse>, APIError> {
    let marker_id = UserId::from_request(&req)?;
    let res = staff_attendance::mark_daily_attendance(
        data,
        path.into_inner(),
        body.into_inner(),
        marker_id.0,
    )
    .await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Mark bulk staff attendance",
    description = "Records daily attendance for multiple staff members.",
    tag = "staff_attendance",
    operation_id = "mark_bulk_staff_attendance"
)]
pub async fn mark_bulk_staff_attendance(
    data: Data<AppState>,
    req: HttpRequest,
    body: Json<BulkMarkStaffAttendanceRequest>,
) -> Result<Json<Vec<StaffAttendanceResponse>>, APIError> {
    let marker_id = UserId::from_request(&req)?;
    let res = staff_attendance::mark_bulk_attendance(
        data,
        body.into_inner(),
        marker_id.0,
    )
    .await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Update staff attendance record",
    description = "Updates an existing staff attendance record.",
    tag = "staff_attendance",
    operation_id = "update_staff_attendance"
)]
pub async fn update_staff_attendance(
    data: Data<AppState>,
    path: Path<String>, // attendance_id
    body: Json<UpdateStaffAttendanceRequest>,
) -> Result<Json<StaffAttendanceResponse>, APIError> {
    let res = staff_attendance::update_attendance(
        data,
        path.into_inner(),
        body.into_inner(),
    )
    .await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Sync leaves with attendance",
    description = "Synchronizes approved staff leaves with attendance records for a specific date.",
    tag = "staff_attendance",
    operation_id = "sync_leaves"
)]
pub async fn sync_leaves() -> Result<Json<String>, APIError> {
    staff_attendance::sync_leaves_to_attendance().await?;
    Ok(Json("Leaves synced successfully".to_string()))
}

#[api_operation(
    summary = "View staff attendance by date",
    description = "Returns a list of staff attendance records for a specific date.",
    tag = "staff_attendance",
    operation_id = "get_staff_attendance_by_date"
)]
pub async fn get_staff_attendance_by_date(
    data: Data<AppState>,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<StaffAttendanceResponse>>, APIError> {
    let date_str = query.get("date").ok_or_else(|| APIError::bad_request("Missing date parameter"))?;
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| APIError::bad_request("Invalid date format"))?;
    let res = staff_attendance::get_attendance_by_date(data, date).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "View staff attendance by staff member",
    description = "Returns a list of staff attendance records for a specific staff member, optionally filtered by a date range.",
    tag = "staff_attendance",
    operation_id = "get_staff_attendance_by_staff_member"
)]
pub async fn get_staff_attendance_by_staff_member(
    data: Data<AppState>,
    path: Path<String>, // staff_id
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<StaffAttendanceResponse>>, APIError> {
    let staff_id = path.into_inner();
    let start_date = query.get("start_date").and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
    let end_date = query.get("end_date").and_then(|s| NaiveDate::parse_from_str(s, "%Y-%m-%d").ok());
    let res = staff_attendance::get_attendance_by_staff(data, staff_id, start_date, end_date).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get my substitutions",
    description = "Returns a list of substitution assignments for the current teacher on a specific date.",
    tag = "staff_attendance",
    operation_id = "get_my_substitutions"
)]
pub async fn get_my_substitutions(
    data: Data<AppState>,
    req: HttpRequest,
    query: web::Query<std::collections::HashMap<String, String>>,
) -> Result<Json<Vec<SubstitutionResponse>>, APIError> {
    let teacher_id = UserId::from_request(&req)?;
    let date_str = query.get("date").ok_or_else(|| APIError::bad_request("Missing date parameter"))?;
    let date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").map_err(|_| APIError::bad_request("Invalid date format"))?;
    let res = staff_attendance::get_substitutions_for_teacher(data, teacher_id.0, date).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Suggest a substitute teacher",
    description = "Recommends a qualified substitute teacher for a given period and date.",
    tag = "staff_attendance",
    operation_id = "suggest_substitute"
)]
pub async fn suggest_substitute() -> Result<Json<Vec<crate::models::staff::staff::StaffResponse>>, APIError> {
    let res = staff_attendance::suggest_substitute().await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Create substitution assignment",
    description = "Manually assigns a substitute teacher for a specific period.",
    tag = "staff_attendance",
    operation_id = "create_substitution"
)]
pub async fn create_substitution(
    data: Data<AppState>,
    body: Json<CreateSubstitutionRequest>,
) -> Result<Json<SubstitutionResponse>, APIError> {
    let res = staff_attendance::assign_substitute(data, body.into_inner()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Record lesson progress",
    description = "Logs the details of what was covered in a specific lesson.",
    tag = "staff_attendance",
    operation_id = "record_lesson_progress"
)]
pub async fn record_lesson_progress(
    data: Data<AppState>,
    req: HttpRequest,
    body: Json<CreateLessonProgressRequest>,
) -> Result<Json<LessonProgress>, APIError> {
    let teacher_id = UserId::from_request(&req)?;
    let res = staff_attendance::record_progress(data, body.into_inner(), teacher_id.0).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get lesson progress by class and subject",
    description = "Returns the progress history for a specific class and subject.",
    tag = "staff_attendance",
    operation_id = "get_lesson_progress"
)]
pub async fn get_lesson_progress(
    data: Data<AppState>,
    path: Path<(String, String)>, // (class_id, subject_id)
) -> Result<Json<Vec<LessonProgress>>, APIError> {
    let (class_id, subject_id) = path.into_inner();
    let res = staff_attendance::get_progress_by_class(data, class_id, subject_id).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Calculate monthly attendance percentage",
    description = "Calculates the attendance percentage for a staff member for a specific month.",
    tag = "staff_attendance",
    operation_id = "calculate_monthly_attendance_percentage"
)]
pub async fn calculate_monthly_attendance_percentage() -> Result<Json<f32>, APIError> {
    let res = staff_attendance::calculate_monthly_percentage().await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get missed topics for student",
    description = "Retrieves topics that a student missed due to being absent in certain periods.",
    tag = "students",
    operation_id = "get_student_missed_topics"
)]
pub async fn get_student_missed_topics() -> Result<Json<Vec<LessonProgress>>, APIError> {
    // Implementation placeholder
    Ok(Json(Vec::new()))
}
