use actix_web::web::Data;
use chrono::Utc;
use diesel::prelude::*;

use crate::AppState;
use crate::errors::APIError;
use crate::handlers::messaging::CreateConversationRequest;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::models::messaging::{
    Conversation, Message, NewConversation, NewConversationParticipant, NewMessage,
};
use crate::schema::{conversation_participants, conversations as conversations_schema, messages as messages_schema};

pub mod conversations;
pub mod messages;

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

    let new_conversation_id = generate_prefixed_id(&mut conn, IdPrefix::CONVERSATION)?;
    let now = Utc::now().naive_utc();
    let new_conversation = NewConversation {
        id: new_conversation_id.clone(),
        subject: req.subject,
        created_at: now,
    };

    diesel::insert_into(conversations_schema::table)
        .values(&new_conversation)
        .execute(&mut conn)?;

    let conversation = conversations_schema::table
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
        .inner_join(conversations_schema::table)
        .select(conversations_schema::all_columns)
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
        .get_result::<i64>(&mut conn)?
        > 0;

    if !is_participant {
        return Err(APIError::forbidden(
            "User is not a participant of this conversation",
        ));
    }

    let new_message_id = generate_prefixed_id(&mut conn, IdPrefix::MESSAGE)?;
    let new_message = NewMessage {
        id: new_message_id.clone(),
        conversation_id,
        sender_user_id,
        content,
        sent_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(messages_schema::table)
        .values(&new_message)
        .execute(&mut conn)?;

    let message = messages_schema::table
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
        .get_result::<i64>(&mut conn)?
        > 0;

    if !is_participant {
        return Err(APIError::forbidden(
            "User is not a participant of this conversation",
        ));
    }

    let conversation_messages = messages_schema::table
        .filter(messages_schema::conversation_id.eq(conversation_id))
        .order(messages_schema::sent_at.asc())
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
    let message: Message = messages_schema::table
        .filter(messages_schema::id.eq(&message_id))
        .first::<Message>(&mut conn)
        .optional()?
        .ok_or_else(|| APIError::not_found("Message not found"))?;

    // Verify if the user is a participant of the conversation
    let is_participant = conversation_participants::table
        .filter(conversation_participants::conversation_id.eq(&message.conversation_id))
        .filter(conversation_participants::user_id.eq(&user_id))
        .count()
        .get_result::<i64>(&mut conn)?
        > 0;

    if !is_participant {
        return Err(APIError::forbidden(
            "User is not a participant of this conversation",
        ));
    }

    let updated_rows = diesel::update(messages_schema::table.filter(messages_schema::id.eq(message_id)))
        .set(messages_schema::read_at.eq(Some(Utc::now().naive_utc())))
        .execute(&mut conn)?;

    Ok(updated_rows)
}
