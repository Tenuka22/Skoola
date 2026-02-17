use diesel::prelude::*;
use std::collections::HashSet;
use std::str::FromStr;

use crate::database::enums::{PermissionEnum, RoleEnum};
use crate::errors::iam::IAMError;
use crate::schema::{user_permissions, user_set_permissions, user_set_users, role_permissions, users};
use crate::utils::logging::log_permission_denied;

/// Retrieves all permissions for a user based on their direct permissions, user sets, and role.
pub fn fetch_all_user_permissions(
    conn: &mut SqliteConnection,
    user_id: &str,
) -> Result<Vec<PermissionEnum>, IAMError> {
    let mut user_permission_enums = HashSet::new();

    // 1. Get the user's role
    let role_str: String = users::table
        .filter(users::id.eq(user_id))
        .select(users::role)
        .first::<String>(conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => IAMError::UserNotFound { identifier: user_id.to_string() },
            _ => IAMError::from(e),
        })?;
    
    let user_role = RoleEnum::from_str(&role_str).unwrap_or(RoleEnum::Guest);

    // 2. Get permissions assigned directly to the user
    let direct_permissions_str: Vec<String> = user_permissions::table
        .filter(user_permissions::user_id.eq(user_id))
        .select(user_permissions::permission)
        .load::<String>(conn)?;
    
    for p_str in direct_permissions_str {
        if let Ok(p) = PermissionEnum::from_str(&p_str) {
            user_permission_enums.insert(p);
        }
    }

    // 3. Get permissions from user sets assigned to the user
    let user_set_ids: Vec<String> = user_set_users::table
        .filter(user_set_users::user_id.eq(user_id))
        .select(user_set_users::user_set_id)
        .load::<String>(conn)?;

    if !user_set_ids.is_empty() {
        let set_permissions_str: Vec<String> = user_set_permissions::table
            .filter(user_set_permissions::user_set_id.eq_any(user_set_ids))
            .select(user_set_permissions::permission)
            .load::<String>(conn)?;
        
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
        .load::<String>(conn)?;
        
    for p_str in role_permissions_str {
        if let Ok(p) = PermissionEnum::from_str(&p_str) {
            user_permission_enums.insert(p);
        }
    }

    Ok(user_permission_enums.into_iter().collect())
}

/// Checks if a user has a specific permission.
pub fn has_permission(
    conn: &mut SqliteConnection,
    user_id: &str,
    permission: PermissionEnum,
) -> Result<bool, IAMError> {
    let permissions = fetch_all_user_permissions(conn, user_id)?;
    let has = permissions.contains(&permission);
    if !has {
        log_permission_denied(user_id, "unknown", &permission.to_string());
    }
    Ok(has)
}
