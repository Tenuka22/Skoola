use crate::database::enums::PermissionEnum;
use crate::handlers::academic::{
    academic_year, class, grade_level, subject, terms,
};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/academic-years")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::AcademicYearManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(academic_year::create_academic_year))
            .route("/{id}", web::get().to(academic_year::get_academic_year_by_id))
            .route("", web::get().to(academic_year::get_all_academic_year))
            .route("/{id}", web::put().to(academic_year::update_academic_year))
            .route("/{id}", web::delete().to(academic_year::delete_academic_year))
            .route("/bulk", web::delete().to(academic_year::bulk_delete_academic_year))
            .route("/bulk", web::patch().to(academic_year::bulk_update_academic_year)),
    )
    .service(
        web::scope("/terms")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::TermManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(terms::create_term))
            .route("/{id}", web::get().to(terms::get_term_by_id))
            .route("", web::get().to(terms::get_all_term))
            .route("/{id}", web::put().to(terms::update_term))
            .route("/{id}", web::delete().to(terms::delete_term))
            .route("/bulk", web::delete().to(terms::bulk_delete_term))
            .route("/bulk", web::patch().to(terms::bulk_update_term)),
    )
    .service(
        web::scope("/grade-levels")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::GradeLevelManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(grade_level::create_grade_level))
            .route("/{id}", web::get().to(grade_level::get_grade_level_by_id))
            .route("", web::get().to(grade_level::get_all_grade_level))
            .route("/{id}", web::put().to(grade_level::update_grade_level))
            .route("/{id}", web::delete().to(grade_level::delete_grade_level))
            .route("/bulk", web::delete().to(grade_level::bulk_delete_grade_level))
            .route("/bulk", web::patch().to(grade_level::bulk_update_grade_level)),
    )
    .service(
        web::scope("/classes")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::ClassManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(class::create_class))
            .route("/{id}", web::get().to(class::get_class_by_id))
            .route("", web::get().to(class::get_all_class))
            .route("/{id}", web::put().to(class::update_class))
            .route("/{id}", web::delete().to(class::delete_class))
            .route("/bulk", web::delete().to(class::bulk_delete_class))
            .route("/bulk", web::patch().to(class::bulk_update_class)),
    )
    .service(
        web::scope("/subjects")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::SubjectManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(subject::create_subject))
            .route("/{id}", web::get().to(subject::get_subject_by_id))
            .route("", web::get().to(subject::get_all_subject))
            .route("/{id}", web::put().to(subject::update_subject))
            .route("/{id}", web::delete().to(subject::delete_subject))
            .route("/bulk", web::delete().to(subject::bulk_delete_subject))
            .route("/bulk", web::patch().to(subject::bulk_update_subject)),
    );
}
