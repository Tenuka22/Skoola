use actix_web::web;
use apistos::api_operation;
use actix_web::web::Json;
use crate::{
    AppState,
    errors::APIError,
    models::school_settings::{SchoolSettingResponse, UpdateSchoolSettingRequest},
    services::system::school_settings,
    utils::{jwt::Authenticated, permission_verification::PermissionVerification},
    database::enums::PermissionEnum,
};

#[api_operation(
    summary = "Get all school settings",
    description = "Retrieves all global school configurations.",
    tag = "settings",
    operation_id = "get_all_settings"
)]
pub async fn get_all_settings(
    data: web::Data<AppState>,
) -> Result<Json<Vec<SchoolSettingResponse>>, APIError> {
    let res = school_settings::get_all_settings(data).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Update a school setting",
    description = "Updates or creates a global school configuration.",
    tag = "settings",
    operation_id = "update_setting"
)]
pub async fn update_setting(
    data: web::Data<AppState>,
    path: web::Path<String>, // setting_key
    body: web::Json<UpdateSchoolSettingRequest>,
) -> Result<Json<SchoolSettingResponse>, APIError> {
    let res = school_settings::update_setting(data, path.into_inner(), body.into_inner()).await?;
    Ok(Json(res))
}

pub fn config(cfg: &mut apistos::web::ServiceConfig) {
    cfg.service(
        apistos::web::scope("/settings")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::SystemAdmin,
            })
            .wrap(Authenticated)
            .route("", apistos::web::get().to(get_all_settings))
            .route("/{key}", apistos::web::put().to(update_setting)),
    );
}
