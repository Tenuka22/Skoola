use actix_web::web;
use crate::handlers::messaging;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(messaging::create_conversation);
    cfg.service(messaging::get_user_conversations);
    cfg.service(messaging::send_message);
    cfg.service(messaging::get_conversation_messages);
    cfg.service(messaging::mark_message_as_read);
}
