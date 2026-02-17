use actix_web::web;
use apistos::api_operation;
use actix_web::web::Json;
use crate::{
    AppState,
    errors::APIError,
    models::staff_attendance::{
        MarkStaffAttendanceRequest, BulkMarkStaffAttendanceRequest, StaffAttendanceResponse, UpdateStaffAttendanceRequest,
        SuggestSubstituteRequest, CreateSubstitutionRequest, SubstitutionResponse
    },
    models::attendance_v2::{CreateLessonProgressRequest, LessonProgressResponse},
    services::staff::staff_attendance,
    utils::jwt::UserId,
};
use chrono::NaiveDate; // Added NaiveDate explicitly and ParseError

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
    _user_id: UserId,
) -> Result<Json<StaffAttendanceResponse>, APIError> {
    let res = staff_attendance::mark_daily_staff_attendance(data, staff_id.into_inner(), body.into_inner()).await?;
    Ok(Json(res))
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
    _user_id: UserId, // Changed to _user_id
) -> Result<Json<Vec<StaffAttendanceResponse>>, APIError> {
    let res = staff_attendance::bulk_mark_staff_attendance(data, body.into_inner(), _user_id.0).await?;
    Ok(Json(res))
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
    _user_id: UserId, // Changed to _user_id
) -> Result<Json<StaffAttendanceResponse>, APIError> {
    let res = staff_attendance::update_staff_attendance(data, attendance_id.into_inner(), body.into_inner(), _user_id.0).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Sync staff leaves to attendance",
    description = "Automatically marks staff as 'Excused' if they have approved leave for the date.",
    tag = "staff_attendance",
    operation_id = "sync_staff_leaves"
)]
pub async fn sync_leaves(
    data: web::Data<AppState>,
    path: web::Path<String>, // Changed to String
) -> Result<Json<i32>, APIError> {
    let date_str = path.into_inner();
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")
        .map_err(|_| APIError::bad_request("Invalid date format. Expected YYYY-MM-DD."))?;
    let count = staff_attendance::sync_staff_leaves(data, date).await?;
    Ok(Json(count))
}

#[api_operation(
    summary = "View staff attendance by date",
    description = "Returns a list of staff attendance records for a specific date.",
    tag = "staff_attendance",
    operation_id = "get_staff_attendance_by_date"
)]
pub async fn get_staff_attendance_by_date(
    data: web::Data<AppState>,
    query: web::Query<crate::models::staff_attendance::StaffAttendanceDateQuery>,
) -> Result<Json<Vec<StaffAttendanceResponse>>, APIError> {
    let res = staff_attendance::get_attendance_by_date(data, query.date).await?;
    Ok(Json(res))
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
    query: web::Query<crate::models::staff_attendance::StaffAttendanceByStaffQuery>,
) -> Result<Json<Vec<StaffAttendanceResponse>>, APIError> {
    let res = staff_attendance::get_attendance_by_staff(data, staff_id.into_inner(), query.start_date, query.end_date).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get my substitutions",
    description = "Returns a list of substitution assignments for the current teacher on a specific date.",
    tag = "staff_attendance",
    operation_id = "get_my_substitutions"
)]
pub async fn get_my_substitutions(
    data: web::Data<AppState>,
    query: web::Query<crate::models::staff_attendance::StaffAttendanceDateQuery>,
    user_id: UserId,
) -> Result<Json<Vec<SubstitutionResponse>>, APIError> {
    let res: Vec<crate::database::tables::Substitution> = staff_attendance::get_substitutions_by_teacher(data, user_id.0, query.date).await?;
    Ok(Json(res.into_iter().map(|s| SubstitutionResponse {
        id: s.id,
        original_teacher_id: s.original_teacher_id,
        substitute_teacher_id: s.substitute_teacher_id,
        timetable_id: s.timetable_id,
        date: s.date,
        status: s.status.to_string(),
        remarks: s.remarks,
    }).collect()))
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
) -> Result<Json<crate::models::staff_attendance::MonthlyAttendancePercentageResponse>, APIError> {
    let (year, month) = path_info.into_inner();
    let res = staff_attendance::calculate_monthly_percentage(data, staff_id.into_inner(), year, month).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Suggest a substitute teacher",
    description = "Finds an available teaching staff member for a specific timetable slot and date.",
    tag = "staff_attendance",
    operation_id = "suggest_substitute"
)]
pub async fn suggest_substitute(
    data: web::Data<AppState>,
    body: web::Json<SuggestSubstituteRequest>,
) -> Result<Json<Option<crate::models::staff::StaffResponse>>, APIError> {
    let res: Option<crate::database::tables::Staff> = staff_attendance::suggest_substitute(data, body.timetable_id.clone(), body.date).await?;
    Ok(Json(res.map(crate::models::staff::StaffResponse::from)))
}

#[api_operation(
    summary = "Create an auto-substitution",
    description = "Automatically assigns a substitute teacher for an absent teacher's slot.",
    tag = "staff_attendance",
    operation_id = "create_substitution"
)]
pub async fn create_substitution(
    data: web::Data<AppState>,
    body: web::Json<CreateSubstitutionRequest>,
) -> Result<Json<SubstitutionResponse>, APIError> {
    let res = staff_attendance::create_auto_substitution(data, body.original_teacher_id.clone(), body.timetable_id.clone(), body.date).await?;
    Ok(Json(SubstitutionResponse {
        id: res.id,
        original_teacher_id: res.original_teacher_id,
        substitute_teacher_id: res.substitute_teacher_id,
        timetable_id: res.timetable_id,
        date: res.date,
        status: res.status.to_string(),
        remarks: res.remarks,
    }))
}

#[api_operation(
    summary = "Record lesson progress",
    description = "Allows a teacher to record the topics covered during a lesson.",
    tag = "staff_attendance",
    operation_id = "record_lesson_progress"
)]
pub async fn record_lesson_progress(
    data: web::Data<AppState>,
    body: web::Json<CreateLessonProgressRequest>,
    teacher_id: UserId,
) -> Result<Json<LessonProgressResponse>, APIError> {
    let res = staff_attendance::record_progress(data, body.into_inner(), teacher_id.0).await?;
    Ok(Json(LessonProgressResponse {
        id: res.id,
        class_id: res.class_id,
        subject_id: res.subject_id,
        teacher_id: res.teacher_id,
        date: res.date,
        topic_covered: res.topic_covered,
        progress_percentage: res.progress_percentage,
    }))
}

#[api_operation(
    summary = "Get lesson progress by class and subject",
    description = "Returns the progress history for a specific class and subject.",
    tag = "staff_attendance",
    operation_id = "get_lesson_progress"
)]
pub async fn get_lesson_progress(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>, // (class_id, subject_id)
) -> Result<Json<Vec<LessonProgressResponse>>, APIError> {
    let (class_id, subject_id) = path.into_inner();
    let res: Vec<crate::database::tables::LessonProgress> = staff_attendance::get_progress_by_class(data, class_id, subject_id).await?;
    Ok(Json(res.into_iter().map(|p| LessonProgressResponse {
        id: p.id,
        class_id: p.class_id,
        subject_id: p.subject_id,
        teacher_id: p.teacher_id,
        date: p.date,
        topic_covered: p.topic_covered,
        progress_percentage: p.progress_percentage,
    }).collect()))
}
