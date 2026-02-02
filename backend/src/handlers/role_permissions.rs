use actix_web::{web, HttpResponse};
use apistos::api_operation;
use diesel::prelude::*;

use crate::{
    AppState,
    database::tables::RolePermission,
    errors::APIError,
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
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let (role_id, permission_id) = path.into_inner();

    let new_assignment = RolePermission {
        role_id,
        permission_id,
    };

    diesel::insert_into(role_permissions::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(HttpResponse::Ok().finish())
}

#[api_operation(
    summary = "Unassign a permission from a role",
    description = "Unassigns a permission from a role by their IDs.",
    tag = "roles"
)]
pub async fn unassign_permission_from_role(
    data: web::Data<AppState>,
    path: web::Path<(String, String)>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let (role_id, permission_id) = path.into_inner();

    diesel::delete(
        role_permissions::table
            .filter(role_permissions::role_id.eq(role_id))
            .filter(role_permissions::permission_id.eq(permission_id)),
    )
    .execute(&mut conn)?;

    Ok(HttpResponse::NoContent().finish())
}
