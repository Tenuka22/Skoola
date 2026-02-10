use actix_web::{web};
use apistos::{api_operation, ApiComponent};
use diesel::prelude::*;
use actix_web::web::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::errors::APIError;
use crate::database::tables::Permission;
use crate::database::enums::{PermissionSeverity, PermissionEnum};
use crate::schema::permissions;
use crate::models::MessageResponse;
// No longer need diesel::sqlite::Sqlite for changes Vec
use diesel::AsChangeset;

#[derive(Debug, Deserialize, ApiComponent, JsonSchema)]
pub struct PermissionQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedPermissionResponse {
    pub data: Vec<Permission>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CreatePermissionRequest {
    pub name: PermissionEnum,
    pub description: String,
    pub safety_level: PermissionSeverity,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema, AsChangeset)]
#[diesel(table_name = permissions)]
pub struct UpdatePermissionRequest {
    pub name: Option<PermissionEnum>,
    pub description: Option<String>,
    pub safety_level: Option<PermissionSeverity>,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct BulkUpdatePermissionsRequest {
    pub permission_ids: Vec<i32>,
    pub name: Option<PermissionEnum>,
    pub description: Option<String>,
    pub safety_level: Option<PermissionSeverity>,
}

#[api_operation(summary = "Get permissions", description = "Get a paginated list of permissions.", tag = "permissions")]
pub async fn get_permissions(
    data: web::Data<AppState>,
    query: web::Query<PermissionQuery>,
) -> Result<Json<PaginatedPermissionResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    
    let mut data_query = permissions::table.into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        data_query = data_query.filter(permissions::name.like(pattern.clone()).or(permissions::description.like(pattern)));
    }

    let sort_col = query.sort_by.as_deref().unwrap_or("name");
    let sort_order = query.sort_order.as_deref().unwrap_or("asc");

    data_query = match (sort_col, sort_order) {
        ("name", "desc") => data_query.order(permissions::name.desc()),
        ("safety_level", "asc") => data_query.order(permissions::safety_level.asc()),
        ("safety_level", "desc") => data_query.order(permissions::safety_level.desc()),
        _ => data_query.order(permissions::name.asc()),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let mut count_query = permissions::table.into_boxed();
    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        count_query = count_query.filter(permissions::name.like(pattern.clone()).or(permissions::description.like(pattern)));
    }

    let total_permissions: i64 = count_query.count().get_result(&mut conn)?;

    let permission_list = data_query
        .limit(limit)
        .offset(offset)
        .load::<Permission>(&mut conn)?;

    let total_pages = (total_permissions as f64 / limit as f64).ceil() as i64;

    Ok(Json(PaginatedPermissionResponse {
        data: permission_list,
        total: total_permissions,
        page,
        limit,
        total_pages,
    }))
}

#[api_operation(summary = "Create permission", description = "Create a new permission.", tag = "permissions")]
pub async fn create_permission(
    data: web::Data<AppState>,
    body: web::Json<CreatePermissionRequest>,
) -> Result<Json<Permission>, APIError> {
    let mut conn = data.db_pool.get()?;
    
    diesel::insert_into(permissions::table)
        .values((
            permissions::name.eq(&body.name),
            permissions::description.eq(&body.description),
            permissions::safety_level.eq(&body.safety_level),
        ))
        .execute(&mut conn)?;

    let new_permission = permissions::table
        .order(permissions::id.desc())
        .first::<Permission>(&mut conn)?;

    Ok(Json(new_permission))
}

#[api_operation(summary = "Get permission", description = "Get a single permission by its ID.", tag = "permissions")]
pub async fn get_permission(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<Json<Permission>, APIError> {
    let mut conn = data.db_pool.get()?;
    let permission = permissions::table.find(path.into_inner()).first::<Permission>(&mut conn)?;
    Ok(Json(permission))
}

#[api_operation(summary = "Update permission", description = "Update a permission by its ID.", tag = "permissions")]
pub async fn update_permission(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    body: web::Json<UpdatePermissionRequest>,
) -> Result<Json<Permission>, APIError> {
    let mut conn = data.db_pool.get()?;
    let id = path.into_inner();

    diesel::update(permissions::table.find(id))
        .set(&body.into_inner()) // Use the struct directly
        .execute(&mut conn)?;

    let updated_permission = permissions::table.find(id).first::<Permission>(&mut conn)?;
    Ok(Json(updated_permission))
}

#[api_operation(summary = "Delete permission", description = "Delete a permission by its ID.", tag = "permissions")]
pub async fn delete_permission(
    data: web::Data<AppState>,
    path: web::Path<i32>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    diesel::delete(permissions::table.find(path.into_inner())).execute(&mut conn)?;
    Ok(Json(MessageResponse { message: "Permission deleted successfully".to_string() }))
}
