use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;
use actix_web::web::Data;

use crate::AppState;
use crate::errors::APIError;
use crate::models::messaging::{Conversation, NewConversation, NewConversationParticipant, Message, NewMessage};
use crate::schema::{conversations, conversation_participants, messages};
use crate::handlers::messaging::CreateConversationRequest;

// Service to start a new conversation
pub async fn start_new_conversation(
    data: Data<AppState>,
    current_user_id: String,
    req: CreateConversationRequest,
) -> Result<Conversation, APIError> {
    let mut conn = data.db_pool.get()?;
    
    let mut participant_ids = req.participant_ids;
    if !participant_ids.contains(&current_user_id) {
        participant_ids.push(current_user_id);
    }

    let new_conversation_id = Uuid::new_v4().to_string();
    let new_conversation = NewConversation {
        id: new_conversation_id.clone(),
        subject: req.subject,
    };

    diesel::insert_into(conversations::table)
        .values(&new_conversation)
        .execute(&mut conn)?;

    let conversation = conversations::table
        .find(&new_conversation_id)
        .first::<Conversation>(&mut conn)?;

    let new_participants: Vec<NewConversationParticipant> = participant_ids
        .into_iter()
        .map(|user_id| NewConversationParticipant {
            conversation_id: conversation.id.clone(),
            user_id,
        })
        .collect();

    diesel::insert_into(conversation_participants::table)
        .values(&new_participants)
        .execute(&mut conn)?;

    Ok(conversation)
}

// Service to get all conversations for a user
pub async fn get_user_conversations(
    data: Data<AppState>,
    user_id: String,
) -> Result<Vec<Conversation>, APIError> {
    let mut conn = data.db_pool.get()?;

    let user_conversations = conversation_participants::table
        .filter(conversation_participants::user_id.eq(user_id))
        .inner_join(conversations::table)
        .select(conversations::all_columns)
        .load::<Conversation>(&mut conn)?;

    Ok(user_conversations)
}

// Service to send a message in a conversation
pub async fn send_message(
    data: Data<AppState>,
    sender_user_id: String,
    conversation_id: String,
    content: String,
) -> Result<Message, APIError> {
    let mut conn = data.db_pool.get()?;

    // Verify if the user is a participant of the conversation
    let is_participant = conversation_participants::table
        .filter(conversation_participants::conversation_id.eq(&conversation_id))
        .filter(conversation_participants::user_id.eq(&sender_user_id))
        .count()
        .get_result::<i64>(&mut conn)? > 0;

    if !is_participant {
        return Err(APIError::forbidden("User is not a participant of this conversation"));
    }

    let new_message_id = Uuid::new_v4().to_string();
    let new_message = NewMessage {
        id: new_message_id.clone(),
        conversation_id,
        sender_user_id,
        content,
    };

    diesel::insert_into(messages::table)
        .values(&new_message)
        .execute(&mut conn)?;

    let message = messages::table
        .find(&new_message_id)
        .first::<Message>(&mut conn)?;

    Ok(message)
}

// Service to get all messages in a conversation
pub async fn get_conversation_messages(
    data: Data<AppState>,
    user_id: String,
    conversation_id: String,
) -> Result<Vec<Message>, APIError> {
    let mut conn = data.db_pool.get()?;

    // Verify if the user is a participant of the conversation
    let is_participant = conversation_participants::table
        .filter(conversation_participants::conversation_id.eq(&conversation_id))
        .filter(conversation_participants::user_id.eq(&user_id))
        .count()
        .get_result::<i64>(&mut conn)? > 0;

    if !is_participant {
        return Err(APIError::forbidden("User is not a participant of this conversation"));
    }

    let conversation_messages = messages::table
        .filter(messages::conversation_id.eq(conversation_id))
        .order(messages::sent_at.asc())
        .load::<Message>(&mut conn)?;

    Ok(conversation_messages)
}

// Service to mark a message as read
pub async fn mark_message_as_read(
    data: Data<AppState>,
    user_id: String,
    message_id: String,
) -> Result<usize, APIError> {
    let mut conn = data.db_pool.get()?;

    // Find the message to get its conversation_id
    let message: Message = messages::table
        .filter(messages::id.eq(&message_id))
        .first::<Message>(&mut conn)
        .optional()?
        .ok_or_else(|| APIError::not_found("Message not found"))?;

    // Verify if the user is a participant of the conversation
    let is_participant = conversation_participants::table
        .filter(conversation_participants::conversation_id.eq(&message.conversation_id))
        .filter(conversation_participants::user_id.eq(&user_id))
        .count()
        .get_result::<i64>(&mut conn)? > 0;

    if !is_participant {
        return Err(APIError::forbidden("User is not a participant of this conversation"));
    }

    let updated_rows = diesel::update(messages::table.filter(messages::id.eq(message_id)))
        .set(messages::read_at.eq(Some(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    Ok(updated_rows)
}