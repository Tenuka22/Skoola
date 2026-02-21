use diesel::prelude::*;
use diesel::connection::AnsiConnection;
use uuid::Uuid;
use anyhow::Result;
use chrono::Utc;

use crate::models::messaging::{Conversation, NewConversation, ConversationParticipant, NewConversationParticipant, Message, NewMessage};
use crate::schema::{conversations, conversation_participants, messages};

// Service to start a new conversation
pub fn start_new_conversation(
    conn: &mut impl AnsiConnection,
    subject: String,
    participant_ids: Vec<String>,
) -> Result<Conversation> {
    let new_conversation_id = Uuid::new_v4().to_string();
    let new_conversation = NewConversation {
        id: new_conversation_id.clone(),
        subject,
    };

    let conversation = diesel::insert_into(conversations::table)
        .values(&new_conversation)
        .get_result::<Conversation>(conn)?;

    let new_participants: Vec<NewConversationParticipant> = participant_ids
        .into_iter()
        .map(|user_id| NewConversationParticipant {
            conversation_id: conversation.id.clone(),
            user_id,
        })
        .collect();

    diesel::insert_into(conversation_participants::table)
        .values(&new_participants)
        .execute(conn)?;

    Ok(conversation)
}

// Service to get all conversations for a user
pub fn get_user_conversations(
    conn: &mut impl AnsiConnection,
    user_id: &str,
) -> Result<Vec<Conversation>> {
    let user_conversations = conversation_participants::table
        .filter(conversation_participants::user_id.eq(user_id))
        .inner_join(conversations::table)
        .select(conversations::all_columns)
        .load::<Conversation>(conn)?;

    Ok(user_conversations)
}

// Service to send a message in a conversation
pub fn send_message(
    conn: &mut impl AnsiConnection,
    conversation_id: String,
    sender_user_id: String,
    content: String,
) -> Result<Message> {
    let new_message_id = Uuid::new_v4().to_string();
    let new_message = NewMessage {
        id: new_message_id.clone(),
        conversation_id,
        sender_user_id,
        content,
    };

    let message = diesel::insert_into(messages::table)
        .values(&new_message)
        .get_result::<Message>(conn)?;

    Ok(message)
}

// Service to get all messages in a conversation
pub fn get_conversation_messages(
    conn: &mut impl AnsiConnection,
    conversation_id: &str,
) -> Result<Vec<Message>> {
    let conversation_messages = messages::table
        .filter(messages::conversation_id.eq(conversation_id))
        .order(messages::sent_at.asc())
        .load::<Message>(conn)?;

    Ok(conversation_messages)
}

// Service to mark a message as read
pub fn mark_message_as_read(
    conn: &mut impl AnsiConnection,
    message_id: &str,
    user_id: &str,
) -> Result<usize> {
    // In a real application, you might want to verify if the user is a participant
    // of the conversation before marking the message as read.
    // For this basic implementation, we'll assume the user has access.

    let updated_rows = diesel::update(messages::table.filter(messages::id.eq(message_id)))
        .set(messages::read_at.eq(Some(Utc::now().naive_utc())))
        .execute(conn)?;

    Ok(updated_rows)
}
