use crate::database::enums::PermissionEnum;
use crate::handlers::messaging;
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/messaging")
            .wrap(Authenticated)
            .route("/conversations", web::post().to(messaging::create_conversation))
            .route("/conversations", web::get().to(messaging::get_user_conversations))
            .route("/conversations/{id}/messages", web::post().to(messaging::send_message))
            .route("/conversations/{id}/messages", web::get().to(messaging::get_conversation_messages))
            .route("/messages/{id}/read", web::post().to(messaging::mark_message_as_read)),
    )
    .service(
        web::scope("/admin/conversations")
            .wrap(PermissionVerification { required_permission: PermissionEnum::MessagingManage })
            .wrap(Authenticated)
            .route("", web::post().to(messaging::admin::create_conversation))
            .route("/{id}", web::get().to(messaging::admin::get_conversation_by_id))
            .route("", web::get().to(messaging::admin::get_all_conversation))
            .route("/{id}", web::put().to(messaging::admin::update_conversation))
            .route("/{id}", web::delete().to(messaging::admin::delete_conversation))
            .route("/bulk", web::delete().to(messaging::admin::bulk_delete_conversation)),
    )
    .service(
        web::scope("/admin/messages")
            .wrap(PermissionVerification { required_permission: PermissionEnum::MessagingManage })
            .wrap(Authenticated)
            .route("", web::post().to(messaging::admin::create_message))
            .route("/{id}", web::get().to(messaging::admin::get_message_by_id))
            .route("", web::get().to(messaging::admin::get_all_message))
            .route("/{id}", web::put().to(messaging::admin::update_message))
            .route("/{id}", web::delete().to(messaging::admin::delete_message))
            .route("/bulk", web::delete().to(messaging::admin::bulk_delete_message)),
    );
}
