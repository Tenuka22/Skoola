use actix_web::web::{Data, Json, Path};
use apistos::{web as apistos_web, api_operation};
use crate::errors::APIError;
use crate::AppState;
use crate::models::resource_management::resource::{
    Resource, ResourceQuery, CreateResourceRequest, UpdateResourceRequest,
    ResourceAsset, ResourceAssetQuery, CreateResourceAssetRequest, UpdateResourceAssetRequest,
    ResourceDetail, ResourceDetailQuery, CreateResourceDetailRequest, UpdateResourceDetailRequest,
};
use crate::services::resource_management::{
    ResourceService, ResourceAssetService, ResourceDetailService,
};
use crate::services::resource_management;

use crate::create_admin_handlers;
use crate::models::auth::CurrentUser;
use crate::models::resource_management::{BookResourceRequest, ResourceBooking};
use crate::database::enums::PermissionEnum;
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;

create_admin_handlers!(
    tag => "resources",
    entity => Resource,
    response => Resource,
    query => ResourceQuery,
    create => CreateResourceRequest,
    update => UpdateResourceRequest,
    service => ResourceService
);

create_admin_handlers!(
    tag => "resource_assets",
    entity => ResourceAsset,
    response => ResourceAsset,
    query => ResourceAssetQuery,
    create => CreateResourceAssetRequest,
    update => UpdateResourceAssetRequest,
    service => ResourceAssetService
);

create_admin_handlers!(
    tag => "resource_details",
    entity => ResourceDetail,
    response => ResourceDetail,
    query => ResourceDetailQuery,
    create => CreateResourceDetailRequest,
    update => UpdateResourceDetailRequest,
    service => ResourceDetailService
);

#[api_operation(
    summary = "Book Resource",
    description = "Books a resource for a specific time period.",
    tag = "resource_bookings",
    operation_id = "book_resource"
)]
pub async fn book_resource(
    data: Data<AppState>,
    current_user: CurrentUser,
    body: Json<BookResourceRequest>,
) -> Result<Json<ResourceBooking>, APIError> {
    let booking =
        resource_management::book_resource(data.clone(), current_user.id, body.into_inner())
            .await?;
    Ok(Json(booking))
}

#[api_operation(
    summary = "Get Resource Bookings",
    description = "Retrieves all bookings for a specific resource.",
    tag = "resource_bookings",
    operation_id = "get_resource_bookings"
)]
pub async fn get_resource_bookings(
    data: Data<AppState>,
    path: Path<String>,
) -> Result<Json<Vec<ResourceBooking>>, APIError> {
    let resource_id = path.into_inner();
    let bookings = resource_management::get_resource_bookings(data.clone(), resource_id).await?;
    Ok(Json(bookings))
}

pub fn config(cfg: &mut apistos_web::ServiceConfig) {
    cfg.service(
        apistos_web::scope("/resource-management")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .route("/book", apistos_web::post().to(book_resource))
            .route("/bookings/{resource_id}", apistos_web::get().to(get_resource_bookings))
            // Admin handlers for Resource
            .route("/resources", apistos_web::post().to(create_resource))
            .route("/resources", apistos_web::get().to(get_all_resource))
            .route("/resources/{id}", apistos_web::get().to(get_resource_by_id))
            .route("/resources/{id}", apistos_web::put().to(update_resource))
            .route("/resources/{id}", apistos_web::delete().to(delete_resource))
            .route("/resources/bulk-delete", apistos_web::post().to(bulk_delete_resource))
            .route("/resources/bulk-update", apistos_web::post().to(bulk_update_resource))
            // Admin handlers for Resource Asset
            .route("/assets", apistos_web::post().to(create_resource_asset))
            .route("/assets", apistos_web::get().to(get_all_resource_asset))
            .route("/assets/{id}", apistos_web::get().to(get_resource_asset_by_id))
            .route("/assets/{id}", apistos_web::put().to(update_resource_asset))
            .route("/assets/{id}", apistos_web::delete().to(delete_resource_asset))
            .route("/assets/bulk-delete", apistos_web::post().to(bulk_delete_resource_asset))
            .route("/assets/bulk-update", apistos_web::post().to(bulk_update_resource_asset))
            // Admin handlers for Resource Detail
            .route("/details", apistos_web::post().to(create_resource_detail))
            .route("/details", apistos_web::get().to(get_all_resource_detail))
            .route("/details/{id}", apistos_web::get().to(get_resource_detail_by_id))
            .route("/details/{id}", apistos_web::put().to(update_resource_detail))
            .route("/details/{id}", apistos_web::delete().to(delete_resource_detail))
            .route("/details/bulk-delete", apistos_web::post().to(bulk_delete_resource_detail))
            .route("/details/bulk-update", apistos_web::post().to(bulk_update_resource_detail))
    );
}

