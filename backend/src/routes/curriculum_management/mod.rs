use crate::database::enums::PermissionEnum;
use crate::handlers::curriculum_management;
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/curriculum-standards")
            .wrap(Authenticated)
            .service(
                web::resource("")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::CurriculumCreate,
                    })
                    .route(web::post().to(curriculum_management::create_curriculum_standard)),
            )
            .service(
                web::resource("/{standard_id}")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::CurriculumRead,
                    })
                    .route(web::get().to(curriculum_management::get_curriculum_standard_by_id)),
            )
            .service(
                web::resource("")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::CurriculumRead,
                    })
                    .route(web::get().to(curriculum_management::get_all_curriculum_standards)),
            )
            .service(
                web::resource("/{standard_id}")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::CurriculumUpdate,
                    })
                    .route(web::put().to(curriculum_management::update_curriculum_standard)),
            )
            .service(
                web::resource("/{standard_id}")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::CurriculumDelete,
                    })
                    .route(web::delete().to(curriculum_management::delete_curriculum_standard)),
            )
            .service(
                web::resource("/{standard_id}/syllabus")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::SyllabusRead,
                    })
                    .route(web::get().to(curriculum_management::get_syllabus_topics_for_standard)),
            ),
    )
    .service(
        web::scope("/syllabus")
            .wrap(Authenticated)
            .service(
                web::resource("")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::SyllabusCreate,
                    })
                    .route(web::post().to(curriculum_management::create_syllabus_topic)),
            )
            .service(
                web::resource("/{syllabus_id}")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::SyllabusRead,
                    })
                    .route(web::get().to(curriculum_management::get_syllabus_topic_by_id)),
            )
            .service(
                web::resource("/{syllabus_id}")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::SyllabusUpdate,
                    })
                    .route(web::put().to(curriculum_management::update_syllabus_topic)),
            )
            .service(
                web::resource("/{syllabus_id}")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::SyllabusDelete,
                    })
                    .route(web::delete().to(curriculum_management::delete_syllabus_topic)),
            ),
    );
}
