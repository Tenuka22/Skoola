use actix_web::web;
use apistos::{api_operation, ApiComponent};
use diesel::prelude::*;
use uuid::Uuid;
use actix_web::web::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    database::tables::{Permission},
    errors::APIError,
    models::permissions::{CreatePermissionRequest, UpdatePermissionRequest},
    models::MessageResponse,
    schema::permissions,
};

#[derive(Debug, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct PermissionQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedPermissionResponse {
    pub data: Vec<Permission>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkDeletePermissionsRequest {
    pub permission_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdatePermissionsRequest {
    pub permission_ids: Vec<String>,
    pub name: Option<String>,
}

#[api_operation(
    summary = "Get all permissions",
    description = "Returns a paginated list of all permissions.",
    tag = "permissions"
)]
pub async fn get_permissions(
    data: web::Data<AppState>,
    query: web::Query<PermissionQuery>,
) -> Result<Json<PaginatedPermissionResponse>, APIError> {
    let inner_query = query.into_inner();
    let (permissions_list, total_permissions, total_pages) =
        crate::services::permissions::get_permissions_paginated(data.clone(), inner_query.clone()).await?;
    Ok(Json(PaginatedPermissionResponse {
        data: permissions_list,
        total: total_permissions,
        page: inner_query.page.unwrap_or(1),
        limit: inner_query.limit.unwrap_or(10),
        total_pages,
    }))
}

#[api_operation(
    summary = "Bulk delete permissions",
    description = "Deletes multiple permissions by their IDs.",
    tag = "permissions"
)]
pub async fn bulk_delete_permissions(
    data: web::Data<AppState>,
    body: web::Json<BulkDeletePermissionsRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    crate::services::permissions::bulk_delete_permissions(data.clone(), body.into_inner().permission_ids).await?;
    Ok(Json(MessageResponse { message: "Permissions deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Bulk update permissions",
    description = "Updates multiple permissions' information.",
    tag = "permissions"
)]
pub async fn bulk_update_permissions(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdatePermissionsRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    crate::services::permissions::bulk_update_permissions(data.clone(), body.into_inner()).await?;
    Ok(Json(MessageResponse { message: "Permissions updated successfully".to_string() }))
}

#[api_operation(
    summary = "Get a permission by ID",
    description = "Returns a single permission by its ID.",
    tag = "permissions"
)]
pub async fn get_permission(
    data: web::Data<AppState>,
    permission_id: web::Path<String>,
) -> Result<Json<Permission>, APIError> {
    let mut conn = data.db_pool.get()?;
    let permission = permissions::table
        .find(permission_id.into_inner())
        .select(Permission::as_select())
        .first(&mut conn)?;
    Ok(Json(permission))
}

#[api_operation(
    summary = "Create a new permission",
    description = "Creates a new permission.",
    tag = "permissions"
)]
pub async fn create_permission(
    data: web::Data<AppState>,
    body: web::Json<CreatePermissionRequest>,
) -> Result<Json<Permission>, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_permission = Permission {
        id: Uuid::new_v4().to_string(),
        name: body.name.clone(),
    };
    diesel::insert_into(permissions::table)
        .values(&new_permission)
        .execute(&mut conn)?;
    Ok(Json(new_permission))
}

#[api_operation(
    summary = "Update a permission",
    description = "Updates a permission by its ID.",
    tag = "permissions"
)]
pub async fn update_permission(
    data: web::Data<AppState>,
    permission_id: web::Path<String>,
    body: web::Json<UpdatePermissionRequest>,
) -> Result<Json<Permission>, APIError> {
    let mut conn = data.db_pool.get()?;
    let permission_id_inner = permission_id.into_inner();
    diesel::update(permissions::table.find(&permission_id_inner))
        .set(permissions::name.eq(&body.name))
        .execute(&mut conn)?;

    let updated_permission = permissions::table
        .find(&permission_id_inner)
        .select(Permission::as_select())
        .first(&mut conn)?;
        
    Ok(Json(updated_permission))
}

#[api_operation(
    summary = "Delete a permission",
    description = "Deletes a permission by its ID.",
    tag = "permissions"
)]
pub async fn delete_permission(
    data: web::Data<AppState>,
    permission_id: web::Path<String>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    diesel::delete(permissions::table.find(permission_id.into_inner())).execute(&mut conn)?;
    Ok(Json(MessageResponse { message: "Permission deleted successfully".to_string() }))
}
