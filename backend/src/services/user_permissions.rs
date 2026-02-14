use diesel::prelude::*;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use actix_web::web;
use std::collections::HashSet;
use std::str::FromStr;

use crate::database::enums::{PermissionEnum, RoleEnum};
use crate::errors::APIError;
use crate::config::AppState;
use crate::schema::{user_permissions, user_set_permissions, user_set_users, role_permissions, users};

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
        let role_str: String = users::table
            .filter(users::id.eq(&user_id_clone))
            .select(users::role)
            .first::<String>(&mut conn)?;
        
        let user_role = RoleEnum::from_str(&role_str).unwrap_or(RoleEnum::Guest);

        // 2. Get permissions assigned directly to the user
        let direct_permissions_str: Vec<String> = user_permissions::table
            .filter(user_permissions::user_id.eq(&user_id_clone))
            .select(user_permissions::permission)
            .load::<String>(&mut conn)?;
        
        for p_str in direct_permissions_str {
            if let Ok(p) = PermissionEnum::from_str(&p_str) {
                user_permission_enums.insert(p);
            }
        }

        // 3. Get permissions from user sets assigned to the user
        // user -> user_set_users -> user_sets -> user_set_permissions
        let user_set_ids: Vec<String> = user_set_users::table
            .filter(user_set_users::user_id.eq(&user_id_clone))
            .select(user_set_users::user_set_id)
            .load::<String>(&mut conn)?;

        if !user_set_ids.is_empty() {
            let set_permissions_str: Vec<String> = user_set_permissions::table
                .filter(user_set_permissions::user_set_id.eq_any(user_set_ids))
                .select(user_set_permissions::permission)
                .load::<String>(&mut conn)?;
            
            for p_str in set_permissions_str {
                if let Ok(p) = PermissionEnum::from_str(&p_str) {
                    user_permission_enums.insert(p);
                }
            }
        }

        // 4. Get permissions from the user's role
        let role_permissions_str: Vec<String> = role_permissions::table
            .filter(role_permissions::role_id.eq(user_role.to_string()))
            .select(role_permissions::permission)
            .load::<String>(&mut conn)?;
            
        for p_str in role_permissions_str {
            if let Ok(p) = PermissionEnum::from_str(&p_str) {
                user_permission_enums.insert(p);
            }
        }

        Ok(user_permission_enums.into_iter().collect())
    }).await.map_err(|e| APIError::internal(&format!("Failed to get user permissions: {}", e)))?
}
