use actix_web::web;
use apistos::{api_operation, ApiComponent};
use diesel::prelude::*;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

use crate::{
    AppState,
    database::enums::PermissionEnum,
    database::tables::RolePermission,
    errors::APIError,
    models::MessageResponse,
    schema::role_permissions,
};

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct RolePermissionRequest {
    pub permission: PermissionEnum,
}

#[api_operation(
    summary = "Assign a permission to a role",
    description = "Assigns a permission to a role by Role ID (Enum string) and Permission Enum.",
    tag = "roles",
    operation_id = "assign_permission_to_role"
)]
pub async fn assign_permission_to_role(
    data: web::Data<AppState>,
    role_id: web::Path<String>,
    body: web::Json<RolePermissionRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_assignment = RolePermission {
        role_id: role_id.into_inner(),
        permission: body.permission.to_string(),
    };

    diesel::insert_into(role_permissions::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(Json(MessageResponse { message: "Permission assigned to role successfully".to_string() }))
}

#[api_operation(
    summary = "Unassign a permission from a role",
    description = "Unassigns a permission from a role.",
    tag = "roles",
    operation_id = "unassign_permission_from_role"
)]
pub async fn unassign_permission_from_role(
    data: web::Data<AppState>,
    role_id: web::Path<String>,
    body: web::Json<RolePermissionRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    diesel::delete(
        role_permissions::table
            .filter(role_permissions::role_id.eq(role_id.into_inner()))
            .filter(role_permissions::permission.eq(body.permission.to_string())),
    )
    .execute(&mut conn)?;

    Ok(Json(MessageResponse { message: "Permission unassigned from role successfully".to_string() }))
}

#[api_operation(
    summary = "Get role permissions",
    description = "Returns a list of all permissions assigned to a specific role.",
    tag = "roles",
    operation_id = "get_role_permissions"
)]
pub async fn get_role_permissions(
    data: web::Data<AppState>,
    role_id: web::Path<String>,
) -> Result<Json<Vec<String>>, APIError> {
    let mut conn = data.db_pool.get()?;
    
    let role_perms: Vec<String> = role_permissions::table
        .filter(role_permissions::role_id.eq(role_id.into_inner()))
        .select(role_permissions::permission)
        .load::<String>(&mut conn)?;

    Ok(Json(role_perms))
}