use crate::errors::iam::IAMError;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    database::tables::Session,
    schema::sessions,
    utils::logging::{log_session_created, log_session_revoked},
};

/// Creates a new session in the database.
pub fn create_session(
    conn: &mut SqliteConnection,
    user_id: &str,
    refresh_token_hash: &str,
    user_agent: Option<&str>,
    ip_address: Option<&str>,
    expires_at: NaiveDateTime,
) -> Result<Session, IAMError> {
    let new_session = Session {
        id: Uuid::new_v4().to_string(),
        user_id: user_id.to_string(),
        refresh_token_hash: refresh_token_hash.to_string(),
        user_agent: user_agent.map(|s| s.to_string()),
        ip_address: ip_address.map(|s| s.to_string()),
        created_at: chrono::Utc::now().naive_utc(),
        expires_at,
    };

    diesel::insert_into(sessions::table)
        .values(&new_session)
        .execute(conn)?;

    log_session_created(user_id, &new_session.id);
    Ok(new_session)
}

/// Finds a session by its refresh token hash.
pub fn find_session_by_refresh_token_hash(
    conn: &mut SqliteConnection,
    refresh_token_hash: &str,
) -> Result<Option<Session>, IAMError> {
    let session = sessions::table
        .filter(sessions::refresh_token_hash.eq(refresh_token_hash))
        .select(Session::as_select())
        .first(conn)
        .optional()?;

    match session.as_ref() {
        Some(s) => {
            info!(event = "session_lookup_success", session_id = %s.id, user_id = %s.user_id, "Session found by hash")
        }
        None => warn!(
            event = "session_lookup_failure",
            "Session not found by hash"
        ),
    }
    Ok(session)
}

/// Deletes a specific session.
pub fn delete_session(conn: &mut SqliteConnection, session_id: &str) -> Result<(), IAMError> {
    diesel::delete(sessions::table.find(session_id)).execute(conn)?;

    log_session_revoked(session_id, "explicit_deletion");
    Ok(())
}

/// Invalidates all sessions for a specific user.
pub fn invalidate_sessions_for_user(
    conn: &mut SqliteConnection,
    user_id: &str,
) -> Result<(), IAMError> {
    diesel::delete(sessions::table.filter(sessions::user_id.eq(user_id))).execute(conn)?;

    info!(event = "sessions_invalidated", user_id = %user_id, "All sessions invalidated for user");
    Ok(())
}
