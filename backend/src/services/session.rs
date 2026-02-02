use crate::errors::APIError;
use diesel::prelude::*;
use chrono::NaiveDateTime;
use uuid::Uuid;
use tracing::{info, warn}; // Import warn

use crate::{
    database::connection::DbPool,
    database::tables::Session,
    schema::sessions,
};

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
    ) -> Result<Session, APIError> {
        let mut conn = self.db_pool.get().map_err(|e| APIError::internal(format!("Failed to get DB connection from pool: {}", e).as_str()))?;

        let new_session = Session {
            id: Uuid::new_v4().to_string(),
            user_id: user_id.clone(),
            refresh_token_hash,
            user_agent: user_agent.clone(),
            ip_address: ip_address.clone(),
            created_at: chrono::Utc::now().naive_utc(),
            expires_at,
        };

        diesel::insert_into(sessions::table)
            .values(&new_session)
            .execute(&mut conn)
            .map_err(|e| APIError::internal(format!("Failed to insert new session: {}", e).as_str()))?;

        info!(
            "ACTION: Session created | user_id: {} | session_id: {} | ip_address: {:?} | user_agent: {:?}",
            user_id, new_session.id, ip_address, user_agent
        );
        Ok(new_session)
    }

    pub async fn find_session_by_refresh_token_hash(
        &self,
        refresh_token_hash: &str,
    ) -> Result<Option<Session>, APIError> {
        let mut conn = self.db_pool.get().map_err(|e| APIError::internal(format!("Failed to get DB connection from pool: {}", e).as_str()))?;

        let session = sessions::table
            .filter(sessions::refresh_token_hash.eq(refresh_token_hash))
            .select(Session::as_select())
            .first(&mut conn)
            .optional()
            .map_err(|e| APIError::internal(format!("Failed to query session by refresh token hash: {}", e).as_str()))?;
        
        match session.as_ref() {
            Some(s) => info!("ACTION: Session found by hash | session_id: {} | user_id: {}", s.id, s.user_id),
            None => warn!("ACTION: Session not found by hash"),
        }
        Ok(session)
    }

    pub async fn delete_session(&self, session_id: &str) -> Result<(), APIError> {
        let mut conn = self.db_pool.get().map_err(|e| APIError::internal(format!("Failed to get DB connection from pool: {}", e).as_str()))?;

        diesel::delete(sessions::table.find(session_id))
            .execute(&mut conn)
            .map_err(|e| APIError::internal(format!("Failed to delete session: {}", e).as_str()))?;

        info!("ACTION: Session deleted | session_id: {}", session_id);
        Ok(())
    }

    pub async fn invalidate_sessions_for_user(&self, user_id: &str) -> Result<(), APIError> {
        let mut conn = self.db_pool.get().map_err(|e| APIError::internal(format!("Failed to get DB connection from pool: {}", e).as_str()))?;

        diesel::delete(sessions::table.filter(sessions::user_id.eq(user_id)))
            .execute(&mut conn)
            .map_err(|e| APIError::internal(format!("Failed to invalidate sessions for user: {}", e).as_str()))?;

        info!("ACTION: Invalidated all sessions | user_id: {}", user_id);
        Ok(())
    }
}
