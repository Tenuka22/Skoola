use actix_web::HttpRequest;
use actix_web::web;
use actix_web::web::Data;
use actix_web::web::Json;
use actix_web::web::Path;
use crate::AppState;
use crate::errors::APIError;
use crate::models::staff::attendance::{
    MarkStaffAttendanceRequest, BulkMarkStaffAttendanceRequest, StaffAttendanceResponse, UpdateStaffAttendanceRequest,
    SuggestSubstituteRequest, CreateSubstitutionRequest, SubstitutionResponse, CreateLessonProgressRequest, LessonProgressResponse
};
use crate::services::staff::staff_attendance;
use crate::utils::jwt::UserId;
use chrono::NaiveDate;
use apistos::api_operation;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::JoinOnDsl;
use diesel::ExpressionMethods;
use diesel::SelectableHelper;

#[api_operation(
    summary = "Mark daily staff attendance",
    description = "Marks attendance for a single staff member for a specific date.",
    tag = "staff_attendance",
    operation_id = "mark_staff_attendance_daily"
)]
pub async fn mark_staff_attendance_daily(
    data: Data<AppState>,
    req: HttpRequest,
    staff_id: Path<String>,
    body: Json<MarkStaffAttendanceRequest>,
) -> Result<Json<StaffAttendanceResponse>, APIError> {
    let _user_id = UserId::from_request(&req)?;
    let res = staff_attendance::mark_daily_staff_attendance(data, staff_id.into_inner(), body.into_inner()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Mark bulk staff attendance",
    description = "Marks attendance for multiple staff members for a specific date.",
    tag = "staff_attendance",
    operation_id = "mark_staff_attendance_bulk"
)]
pub async fn mark_bulk_staff_attendance(
    data: Data<AppState>,
    req: HttpRequest,
    body: Json<BulkMarkStaffAttendanceRequest>,
) -> Result<Json<Vec<StaffAttendanceResponse>>, APIError> {
    let _user_id = UserId::from_request(&req)?;
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
    data: Data<AppState>,
    req: HttpRequest,
    attendance_id: Path<String>,
    body: Json<UpdateStaffAttendanceRequest>,
) -> Result<Json<StaffAttendanceResponse>, APIError> {
    let _user_id = UserId::from_request(&req)?;
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
    data: Data<AppState>,
    path: Path<String>,
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
    data: Data<AppState>,
    query: web::Query<crate::models::staff::attendance::StaffAttendanceDateQuery>,
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
    data: Data<AppState>,
    staff_id: Path<String>,
    query: web::Query<crate::models::staff::attendance::StaffAttendanceByStaffQuery>,
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
    data: Data<AppState>,
    req: HttpRequest,
    query: web::Query<crate::models::staff::attendance::StaffAttendanceDateQuery>,
) -> Result<Json<Vec<SubstitutionResponse>>, APIError> {
    let user_id = UserId::from_request(&req)?;
    let res = staff_attendance::get_substitutions_by_teacher(data, user_id.0, query.date).await?;
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
    data: Data<AppState>,
    staff_id: Path<String>,
    path_info: Path<(i32, u32)>, // (year, month)
) -> Result<Json<crate::models::staff::attendance::MonthlyAttendancePercentageResponse>, APIError> {
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
    data: Data<AppState>,
    body: Json<SuggestSubstituteRequest>,
) -> Result<Json<Option<crate::models::staff::staff::StaffResponse>>, APIError> {
    let res: Option<crate::models::staff::staff::Staff> = staff_attendance::suggest_substitute(data.clone(), body.timetable_id.clone(), body.date).await?;
    
    if let Some(staff_member) = res {
        let mut conn = data.db_pool.get()?;
        use crate::schema::{profiles, user_profiles, users};

        let (profile, user_profile): (crate::models::Profile, Option<crate::models::auth_user::User>) = profiles::table
            .find(staff_member.profile_id.clone().ok_or_else(|| APIError::not_found("Profile not found for staff member"))?)
            .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
            .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
            .select((crate::models::Profile::as_select(), Option::<crate::models::auth_user::User>::as_select()))
            .first(&mut conn)?;

        Ok(Json(Some(crate::models::staff::staff::StaffResponse {
            id: staff_member.id,
            employee_id: staff_member.employee_id,
            name: staff_member.name,
            address: staff_member.address,
            phone: staff_member.phone,
            email: staff_member.email,
            photo_url: staff_member.photo_url,
            nic: staff_member.nic,
            dob: staff_member.dob,
            gender: staff_member.gender,
            employment_status: staff_member.employment_status,
            staff_type: staff_member.staff_type,
            created_at: staff_member.created_at,
            updated_at: staff_member.updated_at,
            profile_id: staff_member.profile_id,
            profile_name: Some(profile.name),
            profile_address: profile.address,
            profile_phone: profile.phone,
            profile_photo_url: profile.photo_url,
            user_email: user_profile.map(|u| u.email),
        })))
    } else {
        Ok(Json(None))
    }
}

#[api_operation(
    summary = "Create an auto-substitution",
    description = "Automatically assigns a substitute teacher for an absent teacher's slot.",
    tag = "staff_attendance",
    operation_id = "create_substitution"
)]
pub async fn create_substitution(
    data: Data<AppState>,
    body: Json<CreateSubstitutionRequest>,
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
    data: Data<AppState>,
    req: HttpRequest,
    body: Json<CreateLessonProgressRequest>,
) -> Result<Json<LessonProgressResponse>, APIError> {
    let teacher_id = UserId::from_request(&req)?;
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
    data: Data<AppState>,
    path: Path<(String, String)>, // (class_id, subject_id)
) -> Result<Json<Vec<LessonProgressResponse>>, APIError> {
    let (class_id, subject_id) = path.into_inner();
    let res = staff_attendance::get_progress_by_class(data, class_id, subject_id).await?;
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
