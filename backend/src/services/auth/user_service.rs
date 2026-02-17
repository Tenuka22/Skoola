use diesel::prelude::*;
use crate::database::tables::User;
use crate::models::auth::UserProfileResponse;
use crate::errors::iam::IAMError;
use crate::schema::users;

/// Converts a User database model to a UserProfileResponse.
pub fn user_to_user_profile_response(user: User) -> Result<UserProfileResponse, IAMError> {
    Ok(UserProfileResponse {
        id: user.id.to_string(),
        email: user.email,
        is_verified: user.is_verified,
        created_at: user.created_at,
        updated_at: user.updated_at,
        roles: vec![user.role],
    })
}

/// Fetches a user by their ID from the database.
pub fn get_user_by_id(conn: &mut SqliteConnection, user_id: &str) -> Result<User, IAMError> {
    users::table
        .find(user_id)
        .first::<User>(conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => IAMError::UserNotFound { identifier: user_id.to_string() },
            _ => IAMError::from(e),
        })
}
