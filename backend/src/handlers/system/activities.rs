use actix_web::{HttpRequest, web};
use apistos::api_operation;
use actix_web::web::Json;
use serde::Deserialize;
use schemars::JsonSchema;
use apistos::ApiComponent;
use crate::{
    AppState,
    errors::APIError,
    models::system::activity::{CreateActivityRequest, ActivityResponse, EnrollParticipantRequest, CreateActivityTypeRequest, ActivityTypeResponse, MarkActivityAttendanceRequest},
    services::system::activities,
    utils::{jwt::Authenticated, permission_verification::PermissionVerification, jwt::UserId},
    database::enums::PermissionEnum,
};

#[api_operation(
    summary = "Get my activities",
    description = "Returns a list of activities the current user is enrolled in.",
    tag = "activities",
    operation_id = "get_my_activities"
)]
pub async fn get_my_activities(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<Json<Vec<ActivityResponse>>, APIError> {
    let user_id = UserId::from_request(&req)?;
    let res = activities::get_user_activities(data, user_id.0).await?;
    Ok(Json(res))
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct GetActivitiesQuery {
    pub academic_year_id: Option<String>,
}

#[api_operation(
    summary = "Get activities",
    description = "Returns a list of school activities, optionally filtered by academic year.",
    tag = "activities",
    operation_id = "get_activities"
)]
pub async fn get_activities(
    data: web::Data<AppState>,
    query: web::Query<GetActivitiesQuery>,
) -> Result<Json<Vec<ActivityResponse>>, APIError> {
    let res = activities::get_activities(data, query.academic_year_id.clone()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Mark activity attendance",
    description = "Marks attendance for a participant in an activity.",
    tag = "activities",
    operation_id = "mark_activity_attendance"
)]
pub async fn mark_activity_attendance(
    data: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>, // activity_id
    body: web::Json<MarkActivityAttendanceRequest>,
) -> Result<Json<String>, APIError> {
    let marker_id = UserId::from_request(&req)?;
    let status = body.status.parse().map_err(|_| APIError::bad_request("Invalid attendance status"))?;
    activities::mark_activity_attendance(data, path.into_inner(), body.user_id.clone(), status, marker_id.0).await?;
    Ok(Json("Attendance marked successfully".to_string()))
}

#[api_operation(
    summary = "Create a new activity type",
    description = "Registers a new activity type (Sport, Detention, etc.).",
    tag = "activities",
    operation_id = "create_activity_type"
)]
pub async fn create_activity_type(
    data: web::Data<AppState>,
    body: web::Json<CreateActivityTypeRequest>,
) -> Result<Json<ActivityTypeResponse>, APIError> {
    let res = activities::create_activity_type(data, body.into_inner()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get all activity types",
    description = "Returns a list of all registered activity types.",
    tag = "activities",
    operation_id = "get_all_activity_types"
)]
pub async fn get_all_activity_types(
    data: web::Data<AppState>,
) -> Result<Json<Vec<ActivityTypeResponse>>, APIError> {
    let res = activities::get_all_activity_types(data).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Create a new activity",
    description = "Registers a new school activity (sport, detention, club, etc.).",
    tag = "activities",
    operation_id = "create_activity"
)]
pub async fn create_activity(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<CreateActivityRequest>,
) -> Result<Json<ActivityResponse>, APIError> {
    let user_id = UserId::from_request(&req)?;
    let res = activities::create_activity(data, body.into_inner(), user_id.0).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Enroll a participant in an activity",
    description = "Adds a student or staff member to an activity.",
    tag = "activities",
    operation_id = "enroll_participant"
)]
pub async fn enroll_participant(
    data: web::Data<AppState>,
    path: web::Path<String>, // activity_id
    body: web::Json<EnrollParticipantRequest>,
) -> Result<Json<String>, APIError> {
    activities::enroll_participant(data, path.into_inner(), body.into_inner()).await?;
    Ok(Json("Enrolled successfully".to_string()))
}

pub fn config(cfg: &mut apistos::web::ServiceConfig) {
    cfg.service(
        apistos::web::scope("/activities")
            .wrap(Authenticated)
            .route("/my", apistos::web::get().to(get_my_activities))
            .route("", apistos::web::get().to(get_activities))
            .route("/{activity_id}/attendance", apistos::web::post().to(mark_activity_attendance))
            .service(
                apistos::web::scope("")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::CoCurricularManage,
                    })
                    .route("", apistos::web::post().to(create_activity))
                    .route("/types", apistos::web::post().to(create_activity_type))
                    .route("/types", apistos::web::get().to(get_all_activity_types))
                    .route("/{activity_id}/enroll", apistos::web::post().to(enroll_participant)),
            ),
    );
}
