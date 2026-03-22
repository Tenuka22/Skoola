use crate::schema::{conversation_participants, conversations, messages};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
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
    AsChangeset,
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
    AsChangeset,
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

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateConversationRequest {
    pub subject: String,
    pub participants: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = conversations)]
pub struct UpdateConversationRequest {
    pub subject: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct ConversationQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for ConversationQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct ConversationResponse {
    pub id: String,
    pub subject: String,
    pub created_at: NaiveDateTime,
}

impl From<Conversation> for ConversationResponse {
    fn from(c: Conversation) -> Self {
        ConversationResponse {
            id: c.id,
            subject: c.subject,
            created_at: c.created_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateMessageRequest {
    pub conversation_id: String,
    pub sender_user_id: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = messages)]
pub struct UpdateMessageRequest {
    pub content: Option<String>,
    pub read_at: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct MessageQuery {
    pub conversation_id: Option<String>,
    pub sender_user_id: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for MessageQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct MessageResponse {
    pub id: String,
    pub conversation_id: String,
    pub sender_user_id: String,
    pub content: String,
    pub sent_at: NaiveDateTime,
    pub read_at: Option<NaiveDateTime>,
}

impl From<Message> for MessageResponse {
    fn from(m: Message) -> Self {
        MessageResponse {
            id: m.id,
            conversation_id: m.conversation_id,
            sender_user_id: m.sender_user_id,
            content: m.content,
            sent_at: m.sent_at,
            read_at: m.read_at,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = conversations)]
pub struct NewConversation {
    pub id: String,
    pub subject: String,
    pub created_at: NaiveDateTime,
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
    pub sent_at: NaiveDateTime,
}
