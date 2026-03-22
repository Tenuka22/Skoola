use crate::{
    AppState,
    database::enums::PermissionEnum,
    errors::APIError,
    models::system::activity::*,
    services::system::activities::*,
    services::system::activities,
    utils::{jwt::Authenticated, jwt::UserId, permission_verification::PermissionVerification},
};
use actix_web::web::Json;
use actix_web::{HttpRequest, web};
use apistos::ApiComponent;
use apistos::api_operation;
use schemars::JsonSchema;
use serde::Deserialize;
use crate::create_admin_handlers;
use crate::services::admin_db::AdminQuery;

create_admin_handlers!(
    tag => "activity_types",
    entity => ActivityType,
    response => ActivityType,
    query => AdminQuery,
    create => CreateActivityTypeRequest,
    update => UpdateActivityTypeRequest,
    service => ActivityTypeService
);

create_admin_handlers!(
    tag => "activities",
    entity => Activity,
    response => Activity,
    query => AdminQuery,
    create => CreateActivityRequest,
    update => UpdateActivityRequest,
    service => ActivityService,
    methods => {
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update,
        bulk_create => generic_bulk_create,
    }
);

create_admin_handlers!(
    tag => "activity_participants",
    entity => ActivityParticipant,
    response => ActivityParticipant,
    query => AdminQuery,
    create => EnrollParticipantRequest,
    update => ActivityParticipant,
    service => ActivityParticipantService
);

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
    Ok(Json(res.into_iter().map(ActivityResponse::from).collect()))
}

#[api_operation(
    summary = "Create Activity",
    description = "Creates a new activity and sets the current user as the creator.",
    tag = "activities",
    operation_id = "create_activity"
)]
pub async fn create_activity(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<CreateActivityRequest>,
) -> Result<Json<Activity>, APIError> {
    let user_id = UserId::from_request(&req)?;
    let res = ActivityService::create_with_logic(data, body.into_inner(), user_id.0).await?;
    Ok(Json(res))
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct GetActivitiesQuery {
    pub academic_year_id: Option<String>,
}

#[api_operation(
    summary = "Get activities list",
    description = "Returns a list of school activities, optionally filtered by academic year.",
    tag = "activities",
    operation_id = "get_activities_list"
)]
pub async fn get_activities_list(
    data: web::Data<AppState>,
    query: web::Query<GetActivitiesQuery>,
) -> Result<Json<Vec<ActivityResponse>>, APIError> {
    let res = activities::get_activities(data, query.academic_year_id.clone()).await?;
    Ok(Json(res.into_iter().map(ActivityResponse::from).collect()))
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
    activities::mark_activity_attendance(
        data,
        path.into_inner(),
        body.user_id.clone(),
        body.status.clone(),
        marker_id.0,
    )
    .await?;
    Ok(Json("Attendance marked successfully".to_string()))
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
            .route("", apistos::web::get().to(get_activities_list))
            .route("/{id}", apistos::web::get().to(get_activity_by_id))
            .route("/{id}/attendance", apistos::web::post().to(mark_activity_attendance))
            .service(
                apistos::web::scope("/admin")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::CoCurricularManage,
                    })
                    .route("", apistos::web::post().to(create_activity))
                    .route("/{id}", apistos::web::put().to(update_activity))
                    .route("/{id}", apistos::web::delete().to(delete_activity))
                    .route("/bulk", apistos::web::delete().to(bulk_delete_activity)),
            ),
    );
    cfg.service(
        apistos::web::scope("/activity-types")
            .wrap(Authenticated)
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::SystemAdmin,
            })
            .route("", apistos::web::post().to(create_activity_type))
            .route("/{id}", apistos::web::get().to(get_activity_type_by_id))
            .route("", apistos::web::get().to(get_all_activity_type))
            .route("/{id}", apistos::web::put().to(update_activity_type))
            .route("/{id}", apistos::web::delete().to(delete_activity_type)),
    );
    cfg.service(
        apistos::web::scope("/activity-participants")
            .wrap(Authenticated)
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::SystemAdmin,
            })
            .route("", apistos::web::get().to(get_all_activity_participant))
            .route("/{id}", apistos::web::delete().to(delete_activity_participant)),
    );
}

