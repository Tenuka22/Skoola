use crate::database::enums::PermissionEnum;
use crate::handlers::curriculum_management::{
    create_curriculum_standard_v2, get_all_curriculum_standard, get_curriculum_standard_by_id, update_curriculum_standard_v2, delete_curriculum_standard,
    bulk_delete_curriculum_standard, bulk_update_curriculum_standard, bulk_create_curriculum_standard,
    create_syllabus_topic_v2, get_all_syllabus_topic, get_syllabus_topic_by_id, update_syllabus_topic_v2, delete_syllabus_topic,
    bulk_delete_syllabus_topic, bulk_update_syllabus_topic, bulk_create_syllabus_topic, get_syllabus_topics_for_standard
};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/curriculum-standards")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::CurriculumManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(create_curriculum_standard_v2))
            .route("/{id}", web::get().to(get_curriculum_standard_by_id))
            .route("", web::get().to(get_all_curriculum_standard))
            .route("/{id}", web::put().to(update_curriculum_standard_v2))
            .route("/{id}", web::delete().to(delete_curriculum_standard))
            .route("/bulk", web::delete().to(bulk_delete_curriculum_standard))
            .route("/bulk", web::patch().to(bulk_update_curriculum_standard))
            .route("/bulk", web::post().to(bulk_create_curriculum_standard)),
    )
    .service(
        web::scope("/syllabus-topics")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::SyllabusManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(create_syllabus_topic_v2))
            .route("/{id}", web::get().to(get_syllabus_topic_by_id))
            .route("", web::get().to(get_all_syllabus_topic))
            .route("/standard/{standard_id}", web::get().to(get_syllabus_topics_for_standard))
            .route("/{id}", web::put().to(update_syllabus_topic_v2))
            .route("/{id}", web::delete().to(delete_syllabus_topic))
            .route("/bulk", web::delete().to(bulk_delete_syllabus_topic))
            .route("/bulk", web::patch().to(bulk_update_syllabus_topic))
            .route("/bulk", web::post().to(bulk_create_syllabus_topic)),
    );
}
