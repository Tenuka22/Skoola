use crate::schema::{conversation_participants, conversations, messages};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = conversations)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Conversation {
    pub id: String,
    pub subject: String,
    pub created_at: NaiveDateTime,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = conversation_participants)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ConversationParticipant {
    pub conversation_id: String,
    pub user_id: String,
}

#[derive(
    Debug,
    Clone,
    PartialEq,
    Queryable,
    Selectable,
    Insertable,
    Serialize,
    Deserialize,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = messages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Message {
    pub id: String,
    pub conversation_id: String,
    pub sender_user_id: String,
    pub content: String,
    pub sent_at: NaiveDateTime,
    pub read_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, PartialEq, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = conversations)]
pub struct NewConversation {
    pub id: String,
    pub subject: String,
}

#[derive(Debug, Clone, PartialEq, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = conversation_participants)]
pub struct NewConversationParticipant {
    pub conversation_id: String,
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub id: String,
    pub conversation_id: String,
    pub sender_user_id: String,
    pub content: String,
}
