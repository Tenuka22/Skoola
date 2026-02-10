use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::web;

use crate::database::enums::PermissionEnum;

use crate::errors::APIError;
use crate::config::AppState;
use crate::schema::{permission_set_permissions, permissions};

pub async fn get_permissions_for_permission_set(
    pool: web::Data<AppState>,
    permission_set_id: &str,
) -> Result<Vec<PermissionEnum>, APIError> {
    let pool_clone = pool.clone();
    let permission_set_id_clone = permission_set_id.to_string();

    web::block(move || {
        let mut conn = pool_clone.db_pool.get()?;

        let permissions_in_set: Vec<PermissionEnum> = permission_set_permissions::table
            .filter(permission_set_permissions::permission_set_id.eq(permission_set_id_clone))
            .inner_join(permissions::table)
            .select(permissions::name)
            .load::<PermissionEnum>(&mut conn)?;

        Ok(permissions_in_set)
    }).await.map_err(|e| APIError::internal(&format!("Failed to get permissions for permission set: {}", e)))?
}

pub async fn assign_permission_to_set(
    pool: web::Data<AppState>,
    permission_set_id: &str,
    permission_id: i32,
) -> Result<(), APIError> {
    let pool_clone = pool.clone();
    let permission_set_id_clone = permission_set_id.to_string();

    web::block(move || {
        let mut conn = pool_clone.db_pool.get()?;

        let new_assignment = crate::database::tables::NewPermissionSetPermission {
            permission_set_id: permission_set_id_clone,
            permission_id,
        };

        diesel::insert_into(permission_set_permissions::table)
            .values(&new_assignment)
            .execute(&mut conn)?;

        Ok(())
    }).await.map_err(|e| APIError::internal(&format!("Failed to assign permission to set: {}", e)))?
}

pub async fn unassign_permission_from_set(
    pool: web::Data<AppState>,
    permission_set_id: &str,
    permission_id: i32,
) -> Result<(), APIError> {
    let pool_clone = pool.clone();
    let permission_set_id_clone = permission_set_id.to_string();

    web::block(move || {
        let mut conn = pool_clone.db_pool.get()?;

        diesel::delete(permission_set_permissions::table)
            .filter(permission_set_permissions::permission_set_id.eq(permission_set_id_clone))
            .filter(permission_set_permissions::permission_id.eq(permission_id))
            .execute(&mut conn)?;

        Ok(())
        }).await.map_err(|e| APIError::internal(&format!("Failed to unassign permission from set: {}", e)))?        }
