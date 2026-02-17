use crate::errors::iam::IAMError;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use uuid::Uuid;
use tracing::{info, warn};

use crate::{
    database::connection::DbPool,
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
        Some(s) => info!(event = "session_lookup_success", session_id = %s.id, user_id = %s.user_id, "Session found by hash"),
        None => warn!(event = "session_lookup_failure", "Session not found by hash"),
    }
    Ok(session)
}

/// Deletes a specific session.
pub fn delete_session(conn: &mut SqliteConnection, session_id: &str) -> Result<(), IAMError> {
    diesel::delete(sessions::table.find(session_id))
        .execute(conn)?;

    log_session_revoked(session_id, "explicit_deletion");
    Ok(())
}

/// Invalidates all sessions for a specific user.
pub fn invalidate_sessions_for_user(conn: &mut SqliteConnection, user_id: &str) -> Result<(), IAMError> {
    diesel::delete(sessions::table.filter(sessions::user_id.eq(user_id)))
        .execute(conn)?;

    info!(event = "sessions_invalidated", user_id = %user_id, "All sessions invalidated for user");
    Ok(())
}

// Legacy struct for backward compatibility
pub struct SessionService {
    pub db_pool: DbPool,
}

impl SessionService {
    pub fn new(db_pool: DbPool) -> Self {
        SessionService { db_pool }
    }

    pub async fn create_session(
        &self,
        user_id: String,
        refresh_token_hash: String,
        user_agent: Option<String>,
        ip_address: Option<String>,
        expires_at: NaiveDateTime,
    ) -> Result<Session, crate::errors::APIError> {
        let mut conn = self.db_pool.get()?;
        create_session(
            &mut conn,
            &user_id,
            &refresh_token_hash,
            user_agent.as_deref(),
            ip_address.as_deref(),
            expires_at,
        ).map_err(|e| crate::errors::APIError::from(e))
    }

    pub async fn find_session_by_refresh_token_hash(
        &self,
        refresh_token_hash: &str,
    ) -> Result<Option<Session>, crate::errors::APIError> {
        let mut conn = self.db_pool.get()?;
        find_session_by_refresh_token_hash(&mut conn, refresh_token_hash)
            .map_err(|e| crate::errors::APIError::from(e))
    }

    pub async fn delete_session(&self, session_id: &str) -> Result<(), crate::errors::APIError> {
        let mut conn = self.db_pool.get()?;
        delete_session(&mut conn, session_id)
            .map_err(|e| crate::errors::APIError::from(e))
    }

    pub async fn invalidate_sessions_for_user(&self, user_id: &str) -> Result<(), crate::errors::APIError> {
        let mut conn = self.db_pool.get()?;
        invalidate_sessions_for_user(&mut conn, user_id)
            .map_err(|e| crate::errors::APIError::from(e))
    }
}
