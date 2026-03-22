use crate::models::messaging::{
    ConversationResponse, CreateConversationRequest, UpdateConversationRequest, ConversationQuery,
    MessageResponse, CreateMessageRequest, UpdateMessageRequest, MessageQuery,
};
use crate::services::messaging::conversations::ConversationsService;
use crate::services::messaging::messages::MessagesService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "admin_messaging_conversations",
    entity => Conversation,
    response => ConversationResponse,
    query => ConversationQuery,
    create => CreateConversationRequest,
    update => UpdateConversationRequest,
    service => ConversationsService,
    methods => {
        create => create_conversation,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
    }
);

create_admin_handlers!(
    tag => "admin_messaging_messages",
    entity => Message,
    response => MessageResponse,
    query => MessageQuery,
    create => CreateMessageRequest,
    update => UpdateMessageRequest,
    service => MessagesService,
    methods => {
        create => create_message,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
    }
);

