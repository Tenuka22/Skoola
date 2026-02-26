use crate::database::enums::PermissionEnum;
use crate::handlers::behavior_management::*;
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/behavior-incident-types")
            .wrap(Authenticated)
            .service(
                web::resource("")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::BehaviorIncidentTypeCreate,
                    })
                    .route(web::post().to(create_behavior_incident_type)),
            )
            .service(
                web::resource("/{type_id}")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::BehaviorIncidentTypeRead,
                    })
                    .route(web::get().to(get_behavior_incident_type_by_id)),
            )
            .service(
                web::resource("")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::BehaviorIncidentTypeRead,
                    })
                    .route(web::get().to(get_all_behavior_incident_types)),
            )
            .service(
                web::resource("/{type_id}")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::BehaviorIncidentTypeUpdate,
                    })
                    .route(web::put().to(update_behavior_incident_type)),
            )
            .service(
                web::resource("/{type_id}")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::BehaviorIncidentTypeDelete,
                    })
                    .route(web::delete().to(delete_behavior_incident_type)),
            ),
    )
    .service(
        web::scope("/behavior-incidents")
            .wrap(Authenticated)
            .service(
                web::resource("")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::BehaviorIncidentRecord,
                    })
                    .route(web::post().to(record_behavior_incident)),
            )
            .service(
                web::resource("/students/{student_id}")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::BehaviorIncidentRead,
                    })
                    .route(web::get().to(get_student_behavior_incidents)),
            )
            .service(
                web::resource("/{incident_id}")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::BehaviorIncidentRead,
                    })
                    .route(web::get().to(get_behavior_incident_by_id)),
            )
            .service(
                web::resource("/{incident_id}")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::BehaviorIncidentUpdate,
                    })
                    .route(web::put().to(update_behavior_incident)),
            )
            .service(
                web::resource("/{incident_id}")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::BehaviorIncidentDelete,
                    })
                    .route(web::delete().to(delete_behavior_incident)),
            ),
    );
}
