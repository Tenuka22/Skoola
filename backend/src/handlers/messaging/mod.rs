use actix_web::web;
use actix_web::web::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::AppState;
use crate::APIError;
use crate::services::messaging;
use crate::models::messaging::{Conversation, Message, ConversationParticipant};

use schemars::JsonSchema;
use apistos::{api_operation, ApiComponent};

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

use crate::models::auth::CurrentUser;

#[api_operation(
    summary = "Create Conversation",
    description = "Starts a new conversation with one or more participants.",
    tag = "Messaging",
    operation_id = "create_conversation"
)]
pub async fn create_conversation(
    data: web::Data<AppState>,
    current_user: CurrentUser,
    body: web::Json<CreateConversationRequest>,
) -> Result<Json<ConversationResponse>, APIError> {
    let conversation =
        messaging::start_new_conversation(data.clone(), current_user.id, body.into_inner()).await?;
    Ok(Json(ConversationResponse::from(conversation)))
}

#[api_operation(
    summary = "Get User Conversations",
    description = "Retrieves all conversations the current user is participating in.",
    tag = "Messaging",
    operation_id = "get_user_conversations"
)]
pub async fn get_user_conversations(
    data: web::Data<AppState>,
    current_user: CurrentUser,
) -> Result<Json<Vec<ConversationResponse>>, APIError> {
    let conversations = messaging::get_user_conversations(data.clone(), current_user.id).await?;
    Ok(Json(conversations.into_iter().map(ConversationResponse::from).collect()))
}

#[api_operation(
    summary = "Send Message",
    description = "Sends a new message to a specific conversation.",
    tag = "Messaging",
    operation_id = "send_message"
)]
pub async fn send_message(
    data: web::Data<AppState>,
    current_user: CurrentUser,
    path: web::Path<String>,
    body: web::Json<SendMessageRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let conversation_id = path.into_inner();
    let message = messaging::send_message(
        data.clone(),
        current_user.id,
        conversation_id,
        body.into_inner().content,
    )
    .await?;
    Ok(Json(MessageResponse::from(message)))
}

#[api_operation(
    summary = "Get Conversation Messages",
    description = "Retrieves all messages for a specific conversation.",
    tag = "Messaging",
    operation_id = "get_conversation_messages"
)]
pub async fn get_conversation_messages(
    data: web::Data<AppState>,
    current_user: CurrentUser,
    path: web::Path<String>,
) -> Result<Json<Vec<MessageResponse>>, APIError> {
    let conversation_id = path.into_inner();
    let messages = messaging::get_conversation_messages(data.clone(), current_user.id, conversation_id).await?;
    Ok(Json(messages.into_iter().map(MessageResponse::from).collect()))
}

#[api_operation(
    summary = "Mark Message as Read",
    description = "Marks a specific message as read by the current user.",
    tag = "Messaging",
    operation_id = "mark_message_as_read"
)]
pub async fn mark_message_as_read(
    data: web::Data<AppState>,
    current_user: CurrentUser,
    path: web::Path<String>,
) -> Result<Json<usize>, APIError> {
    let message_id = path.into_inner();
    let updated_rows = messaging::mark_message_as_read(data.clone(), current_user.id, message_id).await?;
    Ok(Json(updated_rows))
}