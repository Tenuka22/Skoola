use crate::database::enums::PermissionEnum;
use crate::handlers::curriculum_management::{
    appeals, reports, reviews, unit_allocations, topics, ai_notes,
    create_curriculum_standard, get_all_curriculum_standard, get_curriculum_standard_by_id, update_curriculum_standard, delete_curriculum_standard,
    bulk_delete_curriculum_standard, bulk_update_curriculum_standard,
    create_syllabus_topic, get_all_syllabus_topic, get_syllabus_topic_by_id, update_syllabus_topic, delete_syllabus_topic,
    bulk_delete_syllabus_topic, bulk_update_syllabus_topic, get_syllabus_topics_for_standard,
    create_lesson_progress, get_all_lesson_progress, get_lesson_progress_by_id, update_lesson_progress, delete_lesson_progress, bulk_delete_lesson_progress, bulk_update_lesson_progress,
    create_lesson_material, get_all_lesson_material, get_lesson_material_by_id, delete_lesson_material, bulk_delete_lesson_material,
    create_ai_processed_note, get_all_ai_processed_note, get_ai_processed_note_by_id, delete_ai_processed_note, bulk_delete_ai_processed_note,
    create_ai_processed_note_section, get_all_ai_processed_note_section, get_ai_processed_note_section_by_id, delete_ai_processed_note_section, bulk_delete_ai_processed_note_section
};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/curriculum-topics")
            .wrap(PermissionVerification { required_permission: PermissionEnum::CurriculumManage })
            .wrap(Authenticated)
            .route("", web::post().to(topics::create_curriculum_topic))
            .route("/{id}", web::get().to(topics::get_curriculum_topic_by_id))
            .route("", web::get().to(topics::get_all_curriculum_topic))
            .route("/{id}", web::put().to(topics::update_curriculum_topic))
            .route("/{id}", web::delete().to(topics::delete_curriculum_topic))
            .route("/bulk", web::delete().to(topics::bulk_delete_curriculum_topic)),
    )
    .service(
        web::scope("/admin/lesson-reviews")
            .wrap(PermissionVerification { required_permission: PermissionEnum::CurriculumManage })
            .wrap(Authenticated)
            .route("", web::post().to(reviews::create_lesson_review))
            .route("/{id}", web::get().to(reviews::get_lesson_review_by_id))
            .route("", web::get().to(reviews::get_all_lesson_review))
            .route("/{id}", web::put().to(reviews::update_lesson_review))
            .route("/{id}", web::delete().to(reviews::delete_lesson_review))
            .route("/bulk", web::delete().to(reviews::bulk_delete_lesson_review))
            .route("/bulk", web::patch().to(reviews::bulk_update_lesson_review)),
    )
    .service(
        web::scope("/admin/ai-notes")
            .wrap(PermissionVerification { required_permission: PermissionEnum::CurriculumManage })
            .wrap(Authenticated)
            .route("", web::post().to(ai_notes::create_ai_processed_note))
            .route("/{id}", web::get().to(ai_notes::get_ai_processed_note_by_id))
            .route("", web::get().to(ai_notes::get_all_ai_processed_note))
            .route("/{id}", web::delete().to(ai_notes::delete_ai_processed_note))
            .route("/bulk", web::delete().to(ai_notes::bulk_delete_ai_processed_note)),
    )
    .service(
        web::scope("/admin/ai-note-sections")
            .wrap(PermissionVerification { required_permission: PermissionEnum::CurriculumManage })
            .wrap(Authenticated)
            .route("", web::post().to(ai_notes::create_ai_processed_note_section))
            .route("/{id}", web::get().to(ai_notes::get_ai_processed_note_section_by_id))
            .route("", web::get().to(ai_notes::get_all_ai_processed_note_section))
            .route("/{id}", web::delete().to(ai_notes::delete_ai_processed_note_section))
            .route("/bulk", web::delete().to(ai_notes::bulk_delete_ai_processed_note_section)),
    );

    cfg.service(
        web::scope("/curriculum-standards")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::CurriculumManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(create_curriculum_standard))
            .route("/{id}", web::get().to(get_curriculum_standard_by_id))
            .route("", web::get().to(get_all_curriculum_standard))
            .route("/{id}", web::put().to(update_curriculum_standard))
            .route("/{id}", web::delete().to(delete_curriculum_standard))
            .route("/bulk", web::delete().to(bulk_delete_curriculum_standard))
            .route("/bulk", web::patch().to(bulk_update_curriculum_standard)),
    )
    .service(
        web::scope("/lesson-progress-attachments")
            .wrap(PermissionVerification { required_permission: PermissionEnum::CurriculumManage })
            .wrap(Authenticated)
            .route("", web::post().to(crate::handlers::curriculum_management::attachments::create_lesson_progress_attachment))
            .route("/{id}", web::get().to(crate::handlers::curriculum_management::attachments::get_lesson_progress_attachment_by_id))
            .route("", web::get().to(crate::handlers::curriculum_management::attachments::get_all_lesson_progress_attachment))
            .route("/{id}", web::put().to(crate::handlers::curriculum_management::attachments::update_lesson_progress_attachment))
            .route("/{id}", web::delete().to(crate::handlers::curriculum_management::attachments::delete_lesson_progress_attachment))
            .route("/bulk", web::delete().to(crate::handlers::curriculum_management::attachments::bulk_delete_lesson_progress_attachment)),
    )
    .service(
        web::scope("/lesson-appeals")
            .wrap(PermissionVerification { required_permission: PermissionEnum::CurriculumManage })
            .wrap(Authenticated)
            .route("", web::post().to(appeals::submit_practical_appeal))
            .route("/{id}", web::post().to(appeals::review_practical_appeal)),
    )
    .service(
        web::scope("/syllabus-topics")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::SyllabusManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(create_syllabus_topic))
            .route("/{id}", web::get().to(get_syllabus_topic_by_id))
            .route("", web::get().to(get_all_syllabus_topic))
            .route("/standard/{standard_id}", web::get().to(get_syllabus_topics_for_standard))
            .route("/{id}", web::put().to(update_syllabus_topic))
            .route("/{id}", web::delete().to(delete_syllabus_topic))
            .route("/bulk", web::delete().to(bulk_delete_syllabus_topic))
            .route("/bulk", web::patch().to(bulk_update_syllabus_topic)),
    )
    .service(
        web::scope("/lesson-progress")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::CurriculumManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(create_lesson_progress))
            .route("/{id}", web::get().to(get_lesson_progress_by_id))
            .route("", web::get().to(get_all_lesson_progress))
            .route("/{id}", web::put().to(update_lesson_progress))
            .route("/{id}", web::delete().to(delete_lesson_progress))
            .route("/bulk", web::delete().to(bulk_delete_lesson_progress))
            .route("/bulk", web::patch().to(bulk_update_lesson_progress)),
    )
    .service(
        web::scope("/lesson-materials")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::CurriculumManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(create_lesson_material))
            .route("/{id}", web::get().to(get_lesson_material_by_id))
            .route("", web::get().to(get_all_lesson_material))
            .route("/{id}", web::delete().to(delete_lesson_material))
            .route("/bulk", web::delete().to(bulk_delete_lesson_material)),
    )
    .service(
        web::scope("/ai-notes")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::CurriculumManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(create_ai_processed_note))
            .route("/{id}", web::get().to(get_ai_processed_note_by_id))
            .route("", web::get().to(get_all_ai_processed_note))
            .route("/{id}", web::delete().to(delete_ai_processed_note))
            .route("/bulk", web::delete().to(bulk_delete_ai_processed_note)),
    )
    .service(
        web::scope("/ai-note-sections")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::CurriculumManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(create_ai_processed_note_section))
            .route("/{id}", web::get().to(get_ai_processed_note_section_by_id))
            .route("", web::get().to(get_all_ai_processed_note_section))
            .route("/{id}", web::delete().to(delete_ai_processed_note_section))
            .route("/bulk", web::delete().to(bulk_delete_ai_processed_note_section)),
    )
    .service(
        web::scope("/curriculum-appeals")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::CurriculumManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(appeals::submit_practical_appeal))
            .route("/{id}", web::post().to(appeals::review_practical_appeal)),
    )
    .service(
        web::scope("/curriculum-attachments")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::CurriculumManage,
            })
            .wrap(Authenticated)
            .route("/{id}", web::post().to(crate::handlers::curriculum_management::attachments::upload_lesson_attachment)),
    )
    .service(
        web::scope("/curriculum-reports")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::CurriculumManage,
            })
            .wrap(Authenticated)
            .route("/pacing/{class_id}", web::get().to(reports::get_class_pacing_report))
            .route("/catch-up-notify/{student_id}", web::post().to(reports::trigger_catch_up_notifications)),
    )
    .service(
        web::scope("/unit-allocations")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::CurriculumManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(unit_allocations::create_unit_allocation))
            .route("/class/{class_id}", web::get().to(unit_allocations::get_unit_allocations_by_class)),
    );
}
