use crate::{
    AppState,
    database::enums::PermissionEnum,
    errors::APIError,
    models::system::setting::*,
    services::system::school_settings::*,
    services::system::school_settings,
    utils::{jwt::Authenticated, permission_verification::PermissionVerification},
};
use actix_web::web;
use actix_web::web::Json;
use apistos::api_operation;
use crate::create_admin_handlers;
use crate::services::admin_db::AdminQuery;

create_admin_handlers!(
    tag => "school_settings",
    entity => SchoolSetting,
    response => SchoolSettingResponse,
    query => AdminQuery,
    create => CreateSchoolSettingRequest,
    update => UpdateSchoolSettingRequest,
    service => SchoolSettingService
);

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
        apistos::web::scope("/school-settings")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::SystemAdmin,
            })
            .wrap(Authenticated)
            .route("", apistos::web::post().to(create_school_setting))
            .route("/{id}", apistos::web::get().to(get_school_setting_by_id))
            .route("", apistos::web::get().to(get_all_school_setting))
            .route("/{id}", apistos::web::put().to(update_school_setting))
            .route("/{id}", apistos::web::delete().to(delete_school_setting)),
    );
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

