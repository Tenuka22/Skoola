use actix_web::{web, HttpResponse};
use apistos::api_operation;
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::errors::APIError;

use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Deserialize, ApiComponent, JsonSchema)]
pub struct PermissionQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BulkUpdatePermissionsRequest {
    pub permission_ids: Vec<i32>,
    pub name: Option<String>,
}

#[api_operation(summary = "Get permissions", description = "Get a paginated list of permissions.", tag = "permissions")]
pub async fn get_permissions(
    _data: web::Data<AppState>,
    _query: web::Query<PermissionQuery>,
) -> Result<HttpResponse, APIError> {
    Ok(HttpResponse::Ok().finish())
}

#[api_operation(summary = "Create permission", description = "Create a new permission.", tag = "permissions")]
pub async fn create_permission(
    _data: web::Data<AppState>,
) -> Result<HttpResponse, APIError> {
    Ok(HttpResponse::Ok().finish())
}

#[api_operation(summary = "Get permission", description = "Get a single permission by its ID.", tag = "permissions")]
pub async fn get_permission(
    _data: web::Data<AppState>,
    _path: web::Path<i32>,
) -> Result<HttpResponse, APIError> {
    Ok(HttpResponse::Ok().finish())
}

#[api_operation(summary = "Update permission", description = "Update a permission by its ID.", tag = "permissions")]
pub async fn update_permission(
    _data: web::Data<AppState>,
    _path: web::Path<i32>,
) -> Result<HttpResponse, APIError> {
    Ok(HttpResponse::Ok().finish())
}

#[api_operation(summary = "Delete permission", description = "Delete a permission by its ID.", tag = "permissions")]
pub async fn delete_permission(
    _data: web::Data<AppState>,
    _path: web::Path<i32>,
) -> Result<HttpResponse, APIError> {
    Ok(HttpResponse::Ok().finish())
}
