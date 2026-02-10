use diesel::prelude::*;
use crate::database::tables::{User};
use crate::models::auth::UserProfileResponse;
use crate::errors::APIError;

pub fn user_to_user_profile_response(user: User, _conn: &mut SqliteConnection) -> Result<UserProfileResponse, APIError> {
    Ok(UserProfileResponse {
        id: user.id.to_string(),
        email: user.email,
        is_verified: user.is_verified,
        created_at: user.created_at,
        updated_at: user.updated_at,
        roles: vec![user.role],
    })
}