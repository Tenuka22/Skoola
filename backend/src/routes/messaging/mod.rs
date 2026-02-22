use apistos::web;
use crate::handlers::messaging;
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use crate::database::enums::PermissionEnum;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/conversations")
            .wrap(Authenticated)
            .service(
                web::resource("")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::MessagingCreate })
                    .route(web::post().to(messaging::create_conversation)),
            )
            .service(
                web::resource("")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::MessagingRead })
                    .route(web::get().to(messaging::get_user_conversations)),
            )
            .service(
                web::resource("/{conversation_id}/messages")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::MessagingSend })
                    .route(web::post().to(messaging::send_message)),
            )
            .service(
                web::resource("/{conversation_id}/messages")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::MessagingRead })
                    .route(web::get().to(messaging::get_conversation_messages)),
            ),
    )
    .service(
        web::scope("/messages")
            .wrap(Authenticated)
            .service(
                web::resource("/{message_id}/read")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::MessagingRead })
                    .route(web::put().to(messaging::mark_message_as_read)),
            ),
    );
}
