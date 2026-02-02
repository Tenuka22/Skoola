use actix_web::{web, HttpResponse};
use apistos::api_operation;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    AppState,
    database::tables::{Permission},
    errors::APIError,
    models::permissions::{CreatePermissionRequest, UpdatePermissionRequest},
    schema::permissions,
};

#[api_operation(
    summary = "Get all permissions",
    description = "Returns a list of all permissions.",
    tag = "permissions"
)]
pub async fn get_permissions(
    data: web::Data<AppState>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let permissions_list = permissions::table.select(Permission::as_select()).load(&mut conn)?;
    Ok(HttpResponse::Ok().json(permissions_list))
}

#[api_operation(
    summary = "Get a permission by ID",
    description = "Returns a single permission by its ID.",
    tag = "permissions"
)]
pub async fn get_permission(
    data: web::Data<AppState>,
    permission_id: web::Path<String>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let permission = permissions::table
        .find(permission_id.into_inner())
        .select(Permission::as_select())
        .first(&mut conn)?;
    Ok(HttpResponse::Ok().json(permission))
}

#[api_operation(
    summary = "Create a new permission",
    description = "Creates a new permission.",
    tag = "permissions"
)]
pub async fn create_permission(
    data: web::Data<AppState>,
    body: web::Json<CreatePermissionRequest>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_permission = Permission {
        id: Uuid::new_v4().to_string(),
        name: body.name.clone(),
    };
    diesel::insert_into(permissions::table)
        .values(&new_permission)
        .execute(&mut conn)?;
    Ok(HttpResponse::Created().json(new_permission))
}

#[api_operation(
    summary = "Update a permission",
    description = "Updates a permission by its ID.",
    tag = "permissions"
)]
pub async fn update_permission(
    data: web::Data<AppState>,
    permission_id: web::Path<String>,
    body: web::Json<UpdatePermissionRequest>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let permission_id_inner = permission_id.into_inner();
    diesel::update(permissions::table.find(&permission_id_inner))
        .set(permissions::name.eq(&body.name))
        .execute(&mut conn)?;

    let updated_permission = permissions::table
        .find(&permission_id_inner)
        .select(Permission::as_select())
        .first(&mut conn)?;
        
    Ok(HttpResponse::Ok().json(updated_permission))
}

#[api_operation(
    summary = "Delete a permission",
    description = "Deletes a permission by its ID.",
    tag = "permissions"
)]
pub async fn delete_permission(
    data: web::Data<AppState>,
    permission_id: web::Path<String>,
) -> Result<HttpResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    diesel::delete(permissions::table.find(permission_id.into_inner())).execute(&mut conn)?;
    Ok(HttpResponse::NoContent().finish())
}
