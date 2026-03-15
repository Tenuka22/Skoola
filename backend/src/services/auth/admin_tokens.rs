use crate::models::auth::tokens::{AuthTokenQuery, AuthTokenResponse, CreateAuthTokenRequest, AuthToken};
use crate::schema::auth_tokens;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    AuthTokenAdminService,
    auth_tokens::table,
    AuthToken,
    AuthTokenResponse,
    auth_tokens::id,
    AuthTokenQuery,
    |q: auth_tokens::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(auth_tokens::metadata.like(pattern))
    },
    |q: auth_tokens::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(auth_tokens::issued_at.desc()),
        }
    }
);

impl AuthTokenAdminService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateAuthTokenRequest,
    ) -> Result<AuthTokenResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::AUTH_TOKEN)?;
        let new_item = AuthToken {
            id,
            user_id: req.user_id,
            token_hash: req.token_hash,
            token_type: req.token_type,
            issued_at: Utc::now().naive_utc(),
            expires_at: req.expires_at,
            revoked_at: None,
            is_active: true,
            metadata: req.metadata,
        };

        Self::generic_create(pool, new_item).await
    }
}
