use crate::models::messaging::{Message, MessageQuery, MessageResponse, CreateMessageRequest};
use crate::schema::messages;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    MessagesService,
    messages::table,
    Message,
    MessageResponse,
    messages::id,
    MessageQuery,
    |q: messages::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(messages::content.like(pattern))
    },
    |q: messages::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("content", "asc") => q.order(messages::content.asc()),
            ("content", "desc") => q.order(messages::content.desc()),
            _ => q.order(messages::sent_at.desc()),
        }
    }
);

impl MessagesService {
    pub async fn create_message(
        pool: web::Data<AppState>,
        req: CreateMessageRequest,
    ) -> Result<MessageResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::MESSAGE)?;
        let new_item = Message {
            id,
            conversation_id: req.conversation_id,
            sender_user_id: req.sender_user_id,
            content: req.content,
            sent_at: Utc::now().naive_utc(),
            read_at: None,
        };

        Self::generic_create(pool, new_item).await
    }
}
