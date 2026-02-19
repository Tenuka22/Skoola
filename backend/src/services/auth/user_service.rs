use diesel::prelude::*;
use crate::database::tables::User;
use crate::models::auth::UserProfileResponse;
use crate::errors::iam::IAMError;
use crate::schema::users;

use diesel::prelude::*;
use crate::database::tables::User;
use crate::models::auth::UserProfileResponse;
use crate::errors::iam::IAMError;
use crate::schema::{users, profiles, user_profiles};
use crate::models::Profile;

/// Converts a User database model to a UserProfileResponse.
pub fn user_to_user_profile_response(conn: &mut SqliteConnection, user: User) -> Result<UserProfileResponse, IAMError> {
    let (profile, user_profile_entry): (Profile, Option<crate::models::auth_user::UserProfile>) = user_profiles::table
        .filter(user_profiles::user_id.eq(&user.id))
        .inner_join(profiles::table)
        .select((Profile::as_select(), Option::<crate::models::auth_user::UserProfile>::as_select()))
        .first(conn)
        .optional()?
        .unwrap_or_else(|| (
            // Default profile if not found
            Profile {
                id: "".to_string(), // placeholder
                name: "Unknown".to_string(),
                address: None,
                phone: None,
                photo_url: None,
                created_at: chrono::Utc::now().naive_utc(),
                updated_at: chrono::Utc::now().naive_utc(),
            },
            None
        ));

    let roles = user_profile_entry.map_or_else(
        || vec![user.role.clone()],
        |_| vec![user.role.clone()] // Assuming roles are still primarily derived from User for now
    );

    Ok(UserProfileResponse {
        id: user.id.to_string(),
        email: user.email,
        is_verified: user.is_verified,
        created_at: user.created_at,
        updated_at: user.updated_at,
        roles,
        profile_id: if profile.id.is_empty() { None } else { Some(profile.id) },
        name: Some(profile.name),
        address: profile.address,
        phone: profile.phone,
        photo_url: profile.photo_url,
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
