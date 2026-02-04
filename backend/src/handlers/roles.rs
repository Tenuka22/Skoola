use actix_web::web;
use apistos::api_operation;
use diesel::prelude::*;
use uuid::Uuid;
use actix_web::web::Json;

use crate::{
    AppState,
    database::tables::{Role},
    errors::APIError,
    models::roles::{CreateRoleRequest, UpdateRoleRequest},
    models::MessageResponse,
    schema::roles,
};

#[api_operation(
    summary = "Get all roles",
    description = "Returns a list of all roles.",
    tag = "roles"
)]
pub async fn get_roles(
    data: web::Data<AppState>,
) -> Result<Json<Vec<Role>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let roles_list = roles::table.select(Role::as_select()).load(&mut conn)?;
    Ok(Json(roles_list))
}

#[api_operation(
    summary = "Get a role by ID",
    description = "Returns a single role by its ID.",
    tag = "roles"
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
    tag = "roles"
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
    tag = "roles"
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
    tag = "roles"
)]
pub async fn delete_role(
    data: web::Data<AppState>,
    role_id: web::Path<String>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    diesel::delete(roles::table.find(role_id.into_inner())).execute(&mut conn)?;
    Ok(Json(MessageResponse { message: "Role deleted successfully".to_string() }))
}
