use crate::errors::iam::IAMError;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use tracing::{info, warn};

use crate::{
    database::enums::AuthTokenType,
    database::tables::{AuthToken, Session},
    models::ids::{IdPrefix, generate_prefixed_id},
    schema::{auth_tokens, sessions},
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
    let now = chrono::Utc::now().naive_utc();
    let auth_token_id = generate_prefixed_id(conn, IdPrefix::AUTH_TOKEN)?;
    let new_auth_token = AuthToken {
        id: auth_token_id.clone(),
        user_id: user_id.to_string(),
        token_hash: refresh_token_hash.to_string(),
        token_type: AuthTokenType::Refresh,
        issued_at: now,
        expires_at,
        revoked_at: None,
        is_active: true,
        metadata: None,
    };

    let new_session = Session {
        id: generate_prefixed_id(conn, IdPrefix::SESSION)?,
        user_id: user_id.to_string(),
        auth_token_id: Some(auth_token_id),
        verification_token_id: None,
        user_agent: user_agent.map(|s| s.to_string()),
        ip_address: ip_address.map(|s| s.to_string()),
        created_at: now,
        expires_at,
        is_active: true,
        disabled_at: None,
        disabled_reason: None,
        last_seen_at: None,
    };

    diesel::insert_into(auth_tokens::table)
        .values(&new_auth_token)
        .execute(conn)?;

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
    let now = chrono::Utc::now().naive_utc();
    let session = sessions::table
        .inner_join(auth_tokens::table.on(sessions::auth_token_id.eq(auth_tokens::id.nullable())))
        .filter(auth_tokens::token_hash.eq(refresh_token_hash))
        .filter(auth_tokens::is_active.eq(true))
        .filter(auth_tokens::revoked_at.is_null())
        .filter(auth_tokens::expires_at.gt(now))
        .filter(sessions::is_active.eq(true))
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
    let now = chrono::Utc::now().naive_utc();
    let session: Session = sessions::table.find(session_id).first(conn)?;

    diesel::update(sessions::table.find(session_id))
        .set((
            sessions::is_active.eq(false),
            sessions::disabled_at.eq(Some(now)),
            sessions::disabled_reason.eq(Some("explicit_deletion".to_string())),
        ))
        .execute(conn)?;

    if let Some(token_id) = session.auth_token_id {
        diesel::update(auth_tokens::table.find(token_id))
            .set((
                auth_tokens::is_active.eq(false),
                auth_tokens::revoked_at.eq(Some(now)),
            ))
            .execute(conn)?;
    }

    log_session_revoked(session_id, "explicit_deletion");
    Ok(())
}

/// Invalidates all sessions for a specific user.
pub fn invalidate_sessions_for_user(
    conn: &mut SqliteConnection,
    user_id: &str,
) -> Result<(), IAMError> {
    let now = chrono::Utc::now().naive_utc();
    let session_ids: Vec<(String, Option<String>)> = sessions::table
        .filter(sessions::user_id.eq(user_id))
        .select((sessions::id, sessions::auth_token_id))
        .load(conn)?;

    diesel::update(sessions::table.filter(sessions::user_id.eq(user_id)))
        .set((
            sessions::is_active.eq(false),
            sessions::disabled_at.eq(Some(now)),
            sessions::disabled_reason.eq(Some("user_invalidation".to_string())),
        ))
        .execute(conn)?;

    let token_ids: Vec<String> = session_ids
        .into_iter()
        .filter_map(|(_, token_id)| token_id)
        .collect();
    if !token_ids.is_empty() {
        diesel::update(auth_tokens::table.filter(auth_tokens::id.eq_any(token_ids)))
            .set((
                auth_tokens::is_active.eq(false),
                auth_tokens::revoked_at.eq(Some(now)),
            ))
            .execute(conn)?;
    }

    info!(event = "sessions_invalidated", user_id = %user_id, "All sessions invalidated for user");
    Ok(())
}
