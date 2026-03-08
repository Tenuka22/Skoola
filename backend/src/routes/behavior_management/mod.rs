use crate::database::enums::PermissionEnum;
use crate::handlers::behavior_management::{record_incident, get_all_behavior_incident, get_behavior_incident_by_id, update_behavior_incident, delete_behavior_incident, bulk_delete_behavior_incident, bulk_update_behavior_incident, create_behavior_incident_type, get_all_behavior_incident_type, get_behavior_incident_type_by_id, update_behavior_incident_type, delete_behavior_incident_type, bulk_delete_behavior_incident_type, bulk_update_behavior_incident_type};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/incident-types")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::BehaviorIncidentTypeManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(create_behavior_incident_type))
            .route("/{id}", web::get().to(get_behavior_incident_type_by_id))
            .route("", web::get().to(get_all_behavior_incident_type))
            .route("/{id}", web::put().to(update_behavior_incident_type))
            .route("/{id}", web::delete().to(delete_behavior_incident_type))
            .route("/bulk", web::delete().to(bulk_delete_behavior_incident_type))
            .route("/bulk", web::patch().to(bulk_update_behavior_incident_type)),
    )
    .service(
        web::scope("/incidents")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::BehaviorIncidentManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(record_incident))
            .route("/{id}", web::get().to(get_behavior_incident_by_id))
            .route("", web::get().to(get_all_behavior_incident))
            .route("/{id}", web::put().to(update_behavior_incident))
            .route("/{id}", web::delete().to(delete_behavior_incident))
            .route("/bulk", web::delete().to(bulk_delete_behavior_incident))
            .route("/bulk", web::patch().to(bulk_update_behavior_incident)),
    );
}
