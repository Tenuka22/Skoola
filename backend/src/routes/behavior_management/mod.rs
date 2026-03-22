use crate::database::enums::PermissionEnum;
use crate::handlers::behavior_management::{
    record_incident, get_all_behavior_incident, get_behavior_incident_by_id, update_behavior_incident, delete_behavior_incident, bulk_delete_behavior_incident, bulk_update_behavior_incident,
    create_behavior_incident_type, get_all_behavior_incident_type, get_behavior_incident_type_by_id, update_behavior_incident_type, delete_behavior_incident_type, bulk_delete_behavior_incident_type, bulk_update_behavior_incident_type,
    create_behavior_incident_severity_level, get_all_behavior_incident_severity_level, get_behavior_incident_severity_level_by_id, update_behavior_incident_severity_level, delete_behavior_incident_severity_level, bulk_delete_behavior_incident_severity_level, bulk_update_behavior_incident_severity_level,
    create_behavior_incident_action, get_all_behavior_incident_action, get_behavior_incident_action_by_id, update_behavior_incident_action, delete_behavior_incident_action, bulk_delete_behavior_incident_action, bulk_update_behavior_incident_action,
    create_behavior_incident_evidence, get_all_behavior_incident_evidence, get_behavior_incident_evidence_by_id, delete_behavior_incident_evidence, bulk_delete_behavior_incident_evidence,
    create_behavior_incident_followup, get_all_behavior_incident_followup, get_behavior_incident_followup_by_id, delete_behavior_incident_followup, bulk_delete_behavior_incident_followup,
    create_behavior_incident_detail, get_all_behavior_incident_detail, get_behavior_incident_detail_by_id, update_behavior_incident_detail, delete_behavior_incident_detail, bulk_delete_behavior_incident_detail, bulk_update_behavior_incident_detail
};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/incident-details")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::BehaviorIncidentManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(create_behavior_incident_detail))
            .route("/{id}", web::get().to(get_behavior_incident_detail_by_id))
            .route("", web::get().to(get_all_behavior_incident_detail))
            .route("/{id}", web::put().to(update_behavior_incident_detail))
            .route("/{id}", web::delete().to(delete_behavior_incident_detail))
            .route("/bulk", web::delete().to(bulk_delete_behavior_incident_detail))
            .route("/bulk", web::patch().to(bulk_update_behavior_incident_detail)),
    )
    .service(
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
        web::scope("/severity-levels")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::BehaviorIncidentTypeManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(create_behavior_incident_severity_level))
            .route("/{id}", web::get().to(get_behavior_incident_severity_level_by_id))
            .route("", web::get().to(get_all_behavior_incident_severity_level))
            .route("/{id}", web::put().to(update_behavior_incident_severity_level))
            .route("/{id}", web::delete().to(delete_behavior_incident_severity_level))
            .route("/bulk", web::delete().to(bulk_delete_behavior_incident_severity_level))
            .route("/bulk", web::patch().to(bulk_update_behavior_incident_severity_level)),
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
    )
    .service(
        web::scope("/incident-actions")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::BehaviorIncidentManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(create_behavior_incident_action))
            .route("/{id}", web::get().to(get_behavior_incident_action_by_id))
            .route("", web::get().to(get_all_behavior_incident_action))
            .route("/{id}", web::put().to(update_behavior_incident_action))
            .route("/{id}", web::delete().to(delete_behavior_incident_action))
            .route("/bulk", web::delete().to(bulk_delete_behavior_incident_action))
            .route("/bulk", web::patch().to(bulk_update_behavior_incident_action)),
    )
    .service(
        web::scope("/incident-evidence")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::BehaviorIncidentManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(create_behavior_incident_evidence))
            .route("/{id}", web::get().to(get_behavior_incident_evidence_by_id))
            .route("", web::get().to(get_all_behavior_incident_evidence))
            .route("/{id}", web::delete().to(delete_behavior_incident_evidence))
            .route("/bulk", web::delete().to(bulk_delete_behavior_incident_evidence)),
    )
    .service(
        web::scope("/incident-followups")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::BehaviorIncidentManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(create_behavior_incident_followup))
            .route("/{id}", web::get().to(get_behavior_incident_followup_by_id))
            .route("", web::get().to(get_all_behavior_incident_followup))
            .route("/{id}", web::delete().to(delete_behavior_incident_followup))
            .route("/bulk", web::delete().to(bulk_delete_behavior_incident_followup)),
    );
}
