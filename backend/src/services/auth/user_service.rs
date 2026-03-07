use crate::database::tables::User;
use crate::errors::iam::IAMError;
use crate::models::Profile;
use crate::models::auth::UserProfileResponse;
use crate::schema::{profiles, user_profiles, user_status, users};
use diesel::prelude::*;

/// Converts a User database model to a UserProfileResponse.
pub fn user_to_user_profile_response(
    conn: &mut SqliteConnection,
    user: User,
) -> Result<UserProfileResponse, IAMError> {
    let profile: Profile =
        user_profiles::table
            .filter(user_profiles::user_id.eq(&user.id))
            .inner_join(profiles::table)
            .select(Profile::as_select())
            .first(conn)
            .optional()?
            .unwrap_or_else(|| {
                Profile {
                    id: "".to_string(),
                    name: "Unknown".to_string(),
                    created_at: chrono::Utc::now().naive_utc(),
                    updated_at: chrono::Utc::now().naive_utc(),
                }
            });

    let is_verified = user_status::table
        .filter(user_status::user_id.eq(&user.id))
        .select(user_status::is_verified)
        .first::<bool>(conn)
        .optional()?;
    let roles = vec![user.role.clone()];

    Ok(UserProfileResponse {
        id: user.id.to_string(),
        email: user.email,
        is_verified,
        created_at: user.created_at,
        updated_at: user.updated_at,
        roles,
        profile_id: if profile.id.is_empty() {
            None
        } else {
            Some(profile.id)
        },
        name: Some(profile.name),
        address: None,
        phone: None,
        photo_url: None,
    })
}

/// Fetches a user by their ID from the database.
pub fn get_user_by_id(conn: &mut SqliteConnection, user_id: &str) -> Result<User, IAMError> {
    users::table
        .find(user_id)
        .select(User::as_select())
        .first::<User>(conn)
        .map_err(|e| match e {
            diesel::result::Error::NotFound => IAMError::UserNotFound {
                identifier: user_id.to_string(),
            },
            _ => IAMError::from(e),
        })
}
