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
                    .route(
                        web::post()
                            .wrap(PermissionVerification {
                                required_permission: PermissionEnum::BehaviorIncidentTypeCreate,
                            })
                            .to(create_behavior_incident_type),
                    )
                    .route(
                        web::get()
                            .wrap(PermissionVerification {
                                required_permission: PermissionEnum::BehaviorIncidentTypeRead,
                            })
                            .to(get_all_behavior_incident_types),
                    ),
            )
            .service(
                web::resource("/{type_id}")
                    .route(
                        web::get()
                            .wrap(PermissionVerification {
                                required_permission: PermissionEnum::BehaviorIncidentTypeRead,
                            })
                            .to(get_behavior_incident_type_by_id),
                    )
                    .route(
                        web::put()
                            .wrap(PermissionVerification {
                                required_permission: PermissionEnum::BehaviorIncidentTypeUpdate,
                            })
                            .to(update_behavior_incident_type),
                    )
                    .route(
                        web::delete()
                            .wrap(PermissionVerification {
                                required_permission: PermissionEnum::BehaviorIncidentTypeDelete,
                            })
                            .to(delete_behavior_incident_type),
                    ),
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
                    .route(
                        web::get()
                            .wrap(PermissionVerification {
                                required_permission: PermissionEnum::BehaviorIncidentRead,
                            })
                            .to(get_behavior_incident_by_id),
                    )
                    .route(
                        web::put()
                            .wrap(PermissionVerification {
                                required_permission: PermissionEnum::BehaviorIncidentUpdate,
                            })
                            .to(update_behavior_incident),
                    )
                    .route(
                        web::delete()
                            .wrap(PermissionVerification {
                                required_permission: PermissionEnum::BehaviorIncidentDelete,
                            })
                            .to(delete_behavior_incident),
                    ),
            ),
    );
}
