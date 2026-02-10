use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::web;
use std::collections::HashSet;

use crate::database::enums::{PermissionEnum, RoleEnum};
use crate::errors::APIError;
use crate::config::AppState;
use crate::schema::{permissions, user_permissions, user_permission_sets, permission_set_permissions, role_permissions, users};

pub async fn get_all_user_permissions(
    pool: web::Data<AppState>,
    user_id: &str,
) -> Result<Vec<PermissionEnum>, APIError> {
    let pool_clone = pool.clone();
    let user_id_clone = user_id.to_string();

    web::block(move || {
        let mut conn = pool_clone.db_pool.get()?;
        let mut user_permission_enums = HashSet::new();

        // 1. Get the user's role
        let user_role: RoleEnum = users::table
            .filter(users::id.eq(user_id_clone.clone()))
            .select(users::role)
            .first::<RoleEnum>(&mut conn)?;

        // 2. Get permissions assigned directly to the user
        let direct_permissions: Vec<PermissionEnum> = user_permissions::table
            .filter(user_permissions::user_id.eq(user_id_clone.clone()))
            .inner_join(permissions::table)
            .select(permissions::name)
            .load::<PermissionEnum>(&mut conn)?;
        user_permission_enums.extend(direct_permissions);

        // 3. Get permissions from permission sets assigned to the user
        let permission_set_ids: Vec<String> = user_permission_sets::table
            .filter(user_permission_sets::user_id.eq(user_id_clone.clone()))
            .select(user_permission_sets::permission_set_id)
            .load::<String>(&mut conn)?;

        if !permission_set_ids.is_empty() {
            let permissions_from_sets: Vec<PermissionEnum> = permission_set_permissions::table
                .filter(permission_set_permissions::permission_set_id.eq_any(permission_set_ids))
                .inner_join(permissions::table)
                .select(permissions::name)
                .load::<PermissionEnum>(&mut conn)?;
            user_permission_enums.extend(permissions_from_sets);
        }

        // 4. Get permissions from the user's role
        let role_permissions: Vec<PermissionEnum> = role_permissions::table
            .filter(role_permissions::role_id.eq(user_role.to_string()))
            .inner_join(permissions::table)
            .select(permissions::name)
            .load::<PermissionEnum>(&mut conn)?;
        user_permission_enums.extend(role_permissions);

        Ok(user_permission_enums.into_iter().collect())
    }).await.map_err(|e| APIError::internal(&format!("Failed to get user permissions: {}", e)))?
}
