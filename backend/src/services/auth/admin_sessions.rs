use crate::models::auth::session::{SessionQuery, SessionResponse, CreateSessionRequest, Session};
use crate::schema::sessions;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    SessionAdminService,
    sessions::table,
    Session,
    SessionResponse,
    sessions::id,
    SessionQuery,
    |q: sessions::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(sessions::user_agent.like(pattern))
    },
    |q: sessions::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(sessions::created_at.desc()),
        }
    }
);

impl SessionAdminService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateSessionRequest,
    ) -> Result<SessionResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::SESSION)?;
        let now = Utc::now().naive_utc();
        let new_item = Session {
            id,
            user_id: req.user_id,
            auth_token_id: req.auth_token_id,
            verification_token_id: req.verification_token_id,
            user_agent: req.user_agent,
            ip_address: req.ip_address,
            created_at: now,
            expires_at: req.expires_at,
            is_active: true,
            disabled_at: None,
            disabled_reason: None,
            last_seen_at: None,
        };

        Self::generic_create(pool, new_item).await
    }
}
