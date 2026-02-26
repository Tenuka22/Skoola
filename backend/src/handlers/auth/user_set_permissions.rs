use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AppState, database::enums::PermissionEnum, database::tables::UserSetPermission,
    errors::APIError, models::MessageResponse, schema::user_set_permissions,
};

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct UserSetPermissionRequest {
    pub permission: PermissionEnum,
}

#[api_operation(
    summary = "Assign a permission to a user set",
    description = "Assigns a permission to a user set by Set ID and Permission Enum.",
    tag = "user_sets",
    operation_id = "assign_permission_to_user_set"
)]
pub async fn assign_permission_to_user_set(
    data: web::Data<AppState>,
    user_set_id: web::Path<String>,
    body: web::Json<UserSetPermissionRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

    let new_assignment = UserSetPermission {
        user_set_id: user_set_id.into_inner(),
        permission: body.permission.to_string(),
    };

    diesel::insert_into(user_set_permissions::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(Json(MessageResponse {
        message: "Permission assigned to user set successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Unassign a permission from a user set",
    description = "Unassigns a permission from a user set.",
    tag = "user_sets",
    operation_id = "unassign_permission_from_user_set"
)]
pub async fn unassign_permission_from_user_set(
    data: web::Data<AppState>,
    user_set_id: web::Path<String>,
    body: web::Json<UserSetPermissionRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

    diesel::delete(
        user_set_permissions::table
            .filter(user_set_permissions::user_set_id.eq(user_set_id.into_inner()))
            .filter(user_set_permissions::permission.eq(body.permission.to_string())),
    )
    .execute(&mut conn)?;

    Ok(Json(MessageResponse {
        message: "Permission unassigned from user set successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Get user set permissions",
    description = "Returns a list of all permissions assigned to a specific user set.",
    tag = "user_sets",
    operation_id = "get_user_set_permissions"
)]
pub async fn get_user_set_permissions(
    data: web::Data<AppState>,
    user_set_id: web::Path<String>,
) -> Result<Json<Vec<String>>, APIError> {
    let mut conn = data.db_pool.get()?;

    let set_perms: Vec<String> = user_set_permissions::table
        .filter(user_set_permissions::user_set_id.eq(user_set_id.into_inner()))
        .select(user_set_permissions::permission)
        .load::<String>(&mut conn)?;

    Ok(Json(set_perms))
}
