use actix_web::{web, HttpResponse, Responder};
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::SqliteConnection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::services::messaging;
use crate::models::messaging::Conversation;
use crate::models::messaging::Message;
use crate::models::messaging::ConversationParticipant;
use crate::models::auth::user::CurrentUser;
use crate::errors::iam::IamError;
use crate::util::permission_verification::has_permission;

use schemars::JsonSchema;
use apistos::ApiComponent;

pub type Pool = web::Data<r2d2::Pool<ConnectionManager<SqliteConnection>>>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ConversationResponse {
    pub id: String,
    pub subject: String,
    pub created_at: chrono::NaiveDateTime,
}

impl From<Conversation> for ConversationResponse {
    fn from(conversation: Conversation) -> Self {
        ConversationResponse {
            id: conversation.id,
            subject: conversation.subject,
            created_at: conversation.created_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct MessageResponse {
    pub id: String,
    pub conversation_id: String,
    pub sender_user_id: String,
    pub content: String,
    pub sent_at: chrono::NaiveDateTime,
    pub read_at: Option<chrono::NaiveDateTime>,
}

impl From<Message> for MessageResponse {
    fn from(message: Message) -> Self {
        MessageResponse {
            id: message.id,
            conversation_id: message.conversation_id,
            sender_user_id: message.sender_user_id,
            content: message.content,
            sent_at: message.sent_at,
            read_at: message.read_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ConversationParticipantResponse {
    pub conversation_id: String,
    pub user_id: String,
}

impl From<ConversationParticipant> for ConversationParticipantResponse {
    fn from(participant: ConversationParticipant) -> Self {
        ConversationParticipantResponse {
            conversation_id: participant.conversation_id,
            user_id: participant.user_id,
        }
    }
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct CreateConversationRequest {
    #[validate(length(min = 1, message = "Subject cannot be empty"))]
    pub subject: String,
    #[validate(length(min = 1, message = "At least one participant is required"))]
    pub participant_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Validate, JsonSchema, ApiComponent)]
pub struct SendMessageRequest {
    #[validate(length(min = 1, message = "Content cannot be empty"))]
    pub content: String,
}

#[apistos::web("/conversations", post, 
    operation_id = "create_conversation", 
    tag = "Messaging", 
    request_body(content = "CreateConversationRequest", description = "Create conversation request"), 
    responses( (status = 201, description = "Conversation created", content = "ConversationResponse") ) 
)]
pub async fn create_conversation(pool: Pool, current_user: CurrentUser, req: web::Json<CreateConversationRequest>) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "messaging:create")?;

    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let mut participant_ids = req.participant_ids.clone();
    if !participant_ids.contains(&current_user.id) {
        participant_ids.push(current_user.id.clone());
    }

    let conversation = web::block(move || {
        messaging::start_new_conversation(&mut conn, req.subject.clone(), participant_ids)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Created().json(ConversationResponse::from(conversation)))
}

#[apistos::web("/conversations", get, 
    operation_id = "get_user_conversations", 
    tag = "Messaging", 
    responses( (status = 200, description = "Conversations retrieved", content = "Vec<ConversationResponse>") ) 
)]
pub async fn get_user_conversations(pool: Pool, current_user: CurrentUser) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "messaging:view")?;

    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let conversations = web::block(move || {
        messaging::get_user_conversations(&mut conn, &current_user.id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Ok().json(conversations.into_iter().map(ConversationResponse::from).collect::<Vec<_>>()))
}

#[apistos::web("/conversations/{conversation_id}/messages", post, 
    operation_id = "send_message", 
    tag = "Messaging", 
    request_body(content = "SendMessageRequest", description = "Send message request"), 
    responses( (status = 201, description = "Message sent", content = "MessageResponse") ) 
)]
pub async fn send_message(
    pool: Pool,
    current_user: CurrentUser,
    path: web::Path<String>,
    req: web::Json<SendMessageRequest>,
) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "messaging:send")?;

    let conversation_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    // Verify if the user is a participant of the conversation
    let is_participant = web::block(move || {
        use crate::schema::conversation_participants::dsl as cp_dsl;
        cp_dsl::conversation_participants
            .filter(cp_dsl::conversation_id.eq(&conversation_id))
            .filter(cp_dsl::user_id.eq(&current_user.id))
            .count()
            .get_result::<i64>(&mut conn)
            .map(|count| count > 0)
            .map_err(IamError::ServiceError)
    })
    .await??;

    if !is_participant {
        return Err(IamError::Forbidden("User is not a participant of this conversation".to_string()));
    }

    let mut conn = pool.get().map_err(IamError::PoolError)?;
    let message = web::block(move || {
        messaging::send_message(
            &mut conn,
            conversation_id,
            current_user.id,
            req.content.clone(),
        )
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Created().json(MessageResponse::from(message)))
}

#[apistos::web("/conversations/{conversation_id}/messages", get, 
    operation_id = "get_conversation_messages", 
    tag = "Messaging", 
    responses( (status = 200, description = "Messages retrieved", content = "Vec<MessageResponse>") ) 
)]
pub async fn get_conversation_messages(
    pool: Pool,
    current_user: CurrentUser,
    path: web::Path<String>,
) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "messaging:view")?;

    let conversation_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    // Verify if the user is a participant of the conversation
    let is_participant = web::block(move || {
        use crate::schema::conversation_participants::dsl as cp_dsl;
        cp_dsl::conversation_participants
            .filter(cp_dsl::conversation_id.eq(&conversation_id))
            .filter(cp_dsl::user_id.eq(&current_user.id))
            .count()
            .get_result::<i64>(&mut conn)
            .map(|count| count > 0)
            .map_err(IamError::ServiceError)
    })
    .await??;

    if !is_participant {
        return Err(IamError::Forbidden("User is not a participant of this conversation".to_string()));
    }

    let mut conn = pool.get().map_err(IamError::PoolError)?;
    let messages = web::block(move || {
        messaging::get_conversation_messages(&mut conn, &conversation_id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Ok().json(messages.into_iter().map(MessageResponse::from).collect::<Vec<_>>()))
}

#[apistos::web("/messages/{message_id}/read", put, 
    operation_id = "mark_message_as_read", 
    tag = "Messaging", 
    responses( (status = 200, description = "Message marked as read", content = "usize") ) 
)]
pub async fn mark_message_as_read(
    pool: Pool,
    current_user: CurrentUser,
    path: web::Path<String>,
) -> Result<impl Responder, IamError> {
    has_permission(&current_user, "messaging:read")?;

    let message_id = path.into_inner();
    let mut conn = pool.get().map_err(IamError::PoolError)?;

    let updated_rows = web::block(move || {
        messaging::mark_message_as_read(&mut conn, &message_id, &current_user.id)
    })
    .await?
    .map_err(IamError::ServiceError)?;

    Ok(HttpResponse::Ok().json(updated_rows))
}
