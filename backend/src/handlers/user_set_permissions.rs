use actix_web::web;
use apistos::api_operation;
use diesel::prelude::*;
use actix_web::web::Json;
use std::str::FromStr;

use crate::{
    AppState,
    database::enums::PermissionEnum,
    database::tables::UserSetPermission,
    errors::APIError,
    models::MessageResponse,
    schema::user_set_permissions,
};

#[api_operation(
    summary = "Assign a permission to a user set",
    description = "Assigns a permission to a user set by Set ID and Permission Enum.",
    tag = "user_sets"
)]
pub async fn assign_permission_to_user_set(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let (user_set_id, permission_str) = path.into_inner();
    
    // Validate permission enum
    let permission_enum = PermissionEnum::from_str(&permission_str)
        .map_err(|_| APIError::bad_request("Invalid permission"))?;

    let new_assignment = UserSetPermission {
        user_set_id,
        permission: permission_enum.to_string(),
    };

    diesel::insert_into(user_set_permissions::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(Json(MessageResponse { message: "Permission assigned to user set successfully".to_string() }))
}

#[api_operation(
    summary = "Unassign a permission from a user set",
    description = "Unassigns a permission from a user set.",
    tag = "user_sets"
)]
pub async fn unassign_permission_from_user_set(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let (user_set_id, permission_str) = path.into_inner();

    diesel::delete(
        user_set_permissions::table
            .filter(user_set_permissions::user_set_id.eq(user_set_id))
            .filter(user_set_permissions::permission.eq(permission_str)),
    )
    .execute(&mut conn)?;

    Ok(Json(MessageResponse { message: "Permission unassigned from user set successfully".to_string() }))
}

#[api_operation(
    summary = "Get user set permissions",
    description = "Returns a list of all permissions assigned to a specific user set.",
    tag = "user_sets"
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
