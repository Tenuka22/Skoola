use diesel::prelude::*;
use crate::database::tables::{User, UserRole, Role, RoleEnum};
use crate::schema::{user_roles, roles};
use crate::models::auth::UserProfileResponse;
use crate::errors::APIError;

pub fn user_to_user_profile_response(user: User, conn: &mut SqliteConnection) -> Result<UserProfileResponse, APIError> {
    let user_roles_data: Vec<UserRole> = user_roles::table
        .filter(user_roles::user_id.eq(&user.id))
        .load::<UserRole>(conn)?;

    let role_ids: Vec<String> = user_roles_data.into_iter().map(|ur| ur.role_id).collect();

    let roles_data: Vec<Role> = roles::table
        .filter(roles::id.eq_any(role_ids))
        .load::<Role>(conn)?;

    let role_enums: Vec<RoleEnum> = roles_data
        .into_iter()
        .filter_map(|r| r.name.parse::<RoleEnum>().ok())
        .collect();

    Ok(UserProfileResponse {
        id: user.id.to_string(),
        email: user.email,
        is_verified: user.is_verified,
        created_at: user.created_at,
        updated_at: user.updated_at,
        roles: role_enums,
    })
}
