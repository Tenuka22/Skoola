use actix_web::web;
use apistos::{api_operation, ApiComponent};
use diesel::prelude::*;
use uuid::Uuid;
use actix_web::web::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    database::tables::{Role},
    errors::APIError,
    models::roles::{CreateRoleRequest, UpdateRoleRequest},
    models::MessageResponse,
    schema::roles,
};

#[derive(Debug, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct RoleQuery {
    pub search: Option<String>,
    pub parent_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedRoleResponse {
    pub data: Vec<Role>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkDeleteRolesRequest {
    pub role_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateRolesRequest {
    pub role_ids: Vec<String>,
    pub name: Option<String>,
    pub parent_id: Option<String>,
}

#[api_operation(
    summary = "Get all roles",
    description = "Returns a paginated list of all roles.",
    tag = "roles",
    operation_id = "get_all_roles"
)]
pub async fn get_roles(
    data: web::Data<AppState>,
    query: web::Query<RoleQuery>,
) -> Result<Json<PaginatedRoleResponse>, APIError> {
    let inner_query = query.into_inner();
    let (roles_list, total_roles, total_pages) =
        crate::services::roles::get_roles_paginated(data.clone(), inner_query.clone()).await?;
    Ok(Json(PaginatedRoleResponse {
        data: roles_list,
        total: total_roles,
        page: inner_query.page.unwrap_or(1),
        limit: inner_query.limit.unwrap_or(10),
        total_pages,
    }))
}

#[api_operation(
    summary = "Bulk delete roles",
    description = "Deletes multiple roles by their IDs.",
    tag = "roles",
    operation_id = "bulk_delete_roles"
)]
pub async fn bulk_delete_roles(
    data: web::Data<AppState>,
    body: web::Json<BulkDeleteRolesRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    crate::services::roles::bulk_delete_roles(data.clone(), body.into_inner().role_ids).await?;
    Ok(Json(MessageResponse { message: "Roles deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Bulk update roles",
    description = "Updates multiple roles' information.",
    tag = "roles",
    operation_id = "bulk_update_roles"
)]
pub async fn bulk_update_roles(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateRolesRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    crate::services::roles::bulk_update_roles(data.clone(), body.into_inner()).await?;
    Ok(Json(MessageResponse { message: "Roles updated successfully".to_string() }))
}

#[api_operation(
    summary = "Get a role by ID",
    description = "Returns a single role by its ID.",
    tag = "roles",
    operation_id = "get_role_by_id"
)]
pub async fn get_role(
    data: web::Data<AppState>,
    role_id: web::Path<String>,
) -> Result<Json<Role>, APIError> {
    let mut conn = data.db_pool.get()?;
    let role = roles::table
        .find(role_id.into_inner())
        .select(Role::as_select())
        .first(&mut conn)?;
    Ok(Json(role))
}

#[api_operation(
    summary = "Create a new role",
    description = "Creates a new role.",
    tag = "roles",
    operation_id = "create_role"
)]
pub async fn create_role(
    data: web::Data<AppState>,
    body: web::Json<CreateRoleRequest>,
) -> Result<Json<Role>, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_role = Role {
        id: Uuid::new_v4().to_string(),
        name: body.name.clone(),
        parent_id: body.parent_id.clone(),
    };
    diesel::insert_into(roles::table)
        .values(&new_role)
        .execute(&mut conn)?;
    Ok(Json(new_role))
}

#[api_operation(
    summary = "Update a role",
    description = "Updates a role by its ID.",
    tag = "roles",
    operation_id = "update_role"
)]
pub async fn update_role(
    data: web::Data<AppState>,
    role_id: web::Path<String>,
    body: web::Json<UpdateRoleRequest>,
) -> Result<Json<Role>, APIError> {
    let mut conn = data.db_pool.get()?;
    let role_id_inner = role_id.into_inner();
    diesel::update(roles::table.find(&role_id_inner))
        .set((
            roles::name.eq(&body.name),
            roles::parent_id.eq(&body.parent_id),
        ))
        .execute(&mut conn)?;

    let updated_role = roles::table
        .find(&role_id_inner)
        .select(Role::as_select())
        .first(&mut conn)?;
        
    Ok(Json(updated_role))
}

#[api_operation(
    summary = "Delete a role",
    description = "Deletes a role by its ID.",
    tag = "roles",
    operation_id = "delete_role"
)]
pub async fn delete_role(
    data: web::Data<AppState>,
    role_id: web::Path<String>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    diesel::delete(roles::table.find(role_id.into_inner())).execute(&mut conn)?;
    Ok(Json(MessageResponse { message: "Role deleted successfully".to_string() }))
}
