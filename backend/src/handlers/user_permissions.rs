use actix_web::web;
use apistos::api_operation;
use diesel::prelude::*;
use actix_web::web::Json;

use crate::{
    AppState,
    database::tables::{Permission},
    errors::APIError,
    models::MessageResponse,
    schema::{user_permissions, permissions},
};

#[api_operation(
    summary = "Assign a permission to a user",
    description = "Assigns a permission to a user by their IDs.",
    tag = "users"
)]
pub async fn assign_permission_to_user(
    data: web::Data<AppState>,
    path: web::Path<(String, i32)>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let (user_id, permission_id) = path.into_inner();

    diesel::insert_into(user_permissions::table)
        .values((
            user_permissions::user_id.eq(user_id),
            user_permissions::permission_id.eq(permission_id),
        ))
        .execute(&mut conn)?;

    Ok(Json(MessageResponse { message: "Permission assigned to user successfully".to_string() }))
}

#[api_operation(
    summary = "Unassign a permission from a user",
    description = "Unassigns a permission from a user by their IDs.",
    tag = "users"
)]
pub async fn unassign_permission_from_user(
    data: web::Data<AppState>,
    path: web::Path<(String, i32)>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let (user_id, permission_id) = path.into_inner();

    diesel::delete(
        user_permissions::table
            .filter(user_permissions::user_id.eq(user_id))
            .filter(user_permissions::permission_id.eq(permission_id)),
    )
    .execute(&mut conn)?;

    Ok(Json(MessageResponse { message: "Permission unassigned from user successfully".to_string() }))
}

#[api_operation(
    summary = "Get user permissions",
    description = "Returns a list of all permissions assigned to a specific user.",
    tag = "users"
)]
pub async fn get_user_permissions(
    data: web::Data<AppState>,
    user_id: web::Path<String>,
) -> Result<Json<Vec<Permission>>, APIError> {
    let mut conn = data.db_pool.get()?;
    
    let user_perms = user_permissions::table
        .filter(user_permissions::user_id.eq(user_id.into_inner()))
        .inner_join(permissions::table)
        .select(permissions::all_columns)
        .load::<Permission>(&mut conn)?;

    Ok(Json(user_perms))
}
