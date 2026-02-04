use actix_web::web;
use apistos::api_operation;
use diesel::prelude::*;
use actix_web::web::Json;

use crate::{
    AppState,
    database::tables::RolePermission,
    errors::APIError,
    models::MessageResponse,
    schema::role_permissions,
};

#[derive(serde::Deserialize)]
pub struct PermissionAssignmentRequest {
    pub permission_id: String,
}

#[api_operation(
    summary = "Assign a permission to a role",
    description = "Assigns a permission to a role by their IDs.",
    tag = "roles"
)]
pub async fn assign_permission_to_role(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let (role_id, permission_id) = path.into_inner();

    let new_assignment = RolePermission {
        role_id,
        permission_id,
    };

    diesel::insert_into(role_permissions::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(Json(MessageResponse { message: "Permission assigned to role successfully".to_string() }))
}

#[api_operation(
    summary = "Unassign a permission from a role",
    description = "Unassigns a permission from a role by their IDs.",
    tag = "roles"
)]
pub async fn unassign_permission_from_role(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let (role_id, permission_id) = path.into_inner();

    diesel::delete(
        role_permissions::table
            .filter(role_permissions::role_id.eq(role_id))
            .filter(role_permissions::permission_id.eq(permission_id)),
    )
    .execute(&mut conn)?;

    Ok(Json(MessageResponse { message: "Permission unassigned from role successfully".to_string() }))
}
