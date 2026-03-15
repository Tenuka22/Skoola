use crate::models::messaging::{Conversation, ConversationQuery, ConversationResponse, CreateConversationRequest};
use crate::schema::conversations;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    ConversationsService,
    conversations::table,
    Conversation,
    ConversationResponse,
    conversations::id,
    ConversationQuery,
    |q: conversations::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(conversations::subject.like(pattern))
    },
    |q: conversations::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("subject", "asc") => q.order(conversations::subject.asc()),
            ("subject", "desc") => q.order(conversations::subject.desc()),
            _ => q.order(conversations::created_at.desc()),
        }
    }
);

impl ConversationsService {
    pub async fn create_conversation(
        pool: web::Data<AppState>,
        req: CreateConversationRequest,
    ) -> Result<ConversationResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::CONVERSATION)?;
        let new_item = Conversation {
            id,
            subject: req.subject,
            created_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }
}
