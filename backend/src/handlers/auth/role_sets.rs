use actix_web::web;
use apistos::{ApiComponent, api_operation};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use web::Json;

use crate::{
    AppState,
    database::tables::{RoleSet, RoleSetRole},
    errors::APIError,
    models::{MessageResponse, auth::RoleSetGetRoleResponse},
    schema::{role_set_roles, role_sets},
};

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct CreateRoleSetRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct UpdateRoleSetRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, ApiComponent, JsonSchema)]
pub struct RoleSetRoleRequest {
    pub role_id: String,
}

#[api_operation(
    summary = "Get all role sets",
    description = "Returns a list of all role sets.",
    tag = "role_sets",
    operation_id = "get_all_role_sets"
)]
pub async fn get_all_role_sets(data: web::Data<AppState>) -> Result<Json<Vec<RoleSet>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let sets = role_sets::table
        .select(RoleSet::as_select())
        .load::<RoleSet>(&mut conn)?;
    Ok(Json(sets))
}

#[api_operation(
    summary = "Create a new role set",
    description = "Creates a new role set.",
    tag = "role_sets",
    operation_id = "create_role_set"
)]
pub async fn create_role_set(
    data: web::Data<AppState>,
    body: web::Json<CreateRoleSetRequest>,
) -> Result<Json<RoleSet>, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_set = RoleSet {
        id: Uuid::new_v4().to_string(),
        name: body.name.clone(),
        description: body.description.clone(),
    };

    diesel::insert_into(role_sets::table)
        .values(&new_set)
        .execute(&mut conn)?;

    Ok(Json(new_set))
}

#[api_operation(
    summary = "Update a role set",
    description = "Updates a role set by its ID.",
    tag = "role_sets",
    operation_id = "update_role_set"
)]
pub async fn update_role_set(
    data: web::Data<AppState>,
    role_set_id: web::Path<String>,
    body: web::Json<UpdateRoleSetRequest>,
) -> Result<Json<RoleSet>, APIError> {
    let mut conn = data.db_pool.get()?;
    let id = role_set_id.into_inner();

    if let Some(name) = &body.name {
        diesel::update(role_sets::table.find(&id))
            .set(role_sets::name.eq(name))
            .execute(&mut conn)?;
    }

    if let Some(description) = &body.description {
        diesel::update(role_sets::table.find(&id))
            .set(role_sets::description.eq(description))
            .execute(&mut conn)?;
    }

    let updated = role_sets::table.find(id).first::<RoleSet>(&mut conn)?;
    Ok(Json(updated))
}

#[api_operation(
    summary = "Delete a role set",
    description = "Deletes a role set by its ID.",
    tag = "role_sets",
    operation_id = "delete_role_set"
)]
pub async fn delete_role_set(
    data: web::Data<AppState>,
    role_set_id: web::Path<String>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    diesel::delete(role_sets::table.find(role_set_id.into_inner())).execute(&mut conn)?;
    Ok(Json(MessageResponse {
        message: "Role set deleted successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Get roles in a role set",
    description = "Returns a list of all role IDs assigned to a specific role set.",
    tag = "role_sets",
    operation_id = "get_role_set_roles"
)]
pub async fn get_role_set_roles(
    data: web::Data<AppState>,
    role_set_id: web::Path<String>,
) -> Result<Json<RoleSetGetRoleResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

    let role_set_id = role_set_id.into_inner();

    let roles = role_set_roles::table
        .filter(role_set_roles::role_set_id.eq(role_set_id))
        .select(role_set_roles::role_id)
        .load::<String>(&mut conn)?;

    Ok(Json(RoleSetGetRoleResponse { roles }))
}

#[api_operation(
    summary = "Assign a role to a role set",
    description = "Assigns a role to a role set.",
    tag = "role_sets",
    operation_id = "assign_role_to_role_set"
)]
pub async fn assign_role_to_role_set(
    data: web::Data<AppState>,
    role_set_id: web::Path<String>,
    body: web::Json<RoleSetRoleRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_assignment = RoleSetRole {
        role_set_id: role_set_id.into_inner(),
        role_id: body.role_id.clone(),
    };

    diesel::insert_into(role_set_roles::table)
        .values(&new_assignment)
        .execute(&mut conn)?;

    Ok(Json(MessageResponse {
        message: "Role assigned to role set successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Unassign a role from a role set",
    description = "Removes a role from a role set.",
    tag = "role_sets",
    operation_id = "unassign_role_from_role_set"
)]
pub async fn unassign_role_from_role_set(
    data: web::Data<AppState>,
    role_set_id: web::Path<String>,
    body: web::Json<RoleSetRoleRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    diesel::delete(
        role_set_roles::table
            .filter(role_set_roles::role_set_id.eq(role_set_id.into_inner()))
            .filter(role_set_roles::role_id.eq(body.role_id.clone())),
    )
    .execute(&mut conn)?;

    Ok(Json(MessageResponse {
        message: "Role unassigned from role set successfully".to_string(),
    }))
}
