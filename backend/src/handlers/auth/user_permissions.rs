use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AppState, database::enums::PermissionEnum, database::tables::UserPermission, errors::APIError,
    models::MessageResponse, schema::user_permissions,
};

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct UserPermissionRequest {
    pub permission: PermissionEnum,
}

#[api_operation(
    summary = "Assign a permission to a user",
    description = "Assigns a permission to a user by ID and Permission Enum.",
    tag = "users",
    operation_id = "assign_permission_to_user"
)]
pub async fn assign_permission_to_user(
    data: web::Data<AppState>,
    user_id: web::Path<String>,
    body: web::Json<UserPermissionRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_assignment = UserPermission {
        user_id: user_id.into_inner(),
        permission: body.permission.to_string(),
    };

    diesel::insert_into(user_permissions::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(Json(MessageResponse {
        message: "Permission assigned to user successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Unassign a permission from a user",
    description = "Unassigns a permission from a user by ID and Permission Enum.",
    tag = "users",
    operation_id = "unassign_permission_from_user"
)]
pub async fn unassign_permission_from_user(
    data: web::Data<AppState>,
    user_id: web::Path<String>,
    body: web::Json<UserPermissionRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    diesel::delete(
        user_permissions::table
            .filter(user_permissions::user_id.eq(user_id.into_inner()))
            .filter(user_permissions::permission.eq(body.permission.to_string())),
    )
    .execute(&mut conn)?;

    Ok(Json(MessageResponse {
        message: "Permission unassigned from user successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Get user permissions",
    description = "Returns a list of all permissions assigned to a specific user (direct assignment).",
    tag = "users",
    operation_id = "get_user_permissions"
)]
pub async fn get_user_permissions(
    data: web::Data<AppState>,
    user_id: web::Path<String>,
) -> Result<Json<Vec<String>>, APIError> {
    let mut conn = data.db_pool.get()?;

    let user_perms: Vec<String> = user_permissions::table
        .filter(user_permissions::user_id.eq(user_id.into_inner()))
        .select(user_permissions::permission)
        .load::<String>(&mut conn)?;

    Ok(Json(user_perms))
}
