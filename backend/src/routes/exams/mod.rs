use crate::database::enums::PermissionEnum;
use crate::handlers::exams::{
    exam_subjects, exam_types, exams, grading_criteria, grading_schemes, report_cards,
    special_exams, zscore,
};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/exam-types")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::ExamTypeManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(exam_types::create_exam_type))
            .route("/{id}", web::get().to(exam_types::get_exam_type_by_id))
            .route("", web::get().to(exam_types::get_all_exam_types))
            .route("/{id}", web::put().to(exam_types::update_exam_type))
            .route("/{id}", web::delete().to(exam_types::delete_exam_type))
            .route(
                "/bulk",
                web::delete().to(exam_types::bulk_delete_exam_types),
            )
            .route("/bulk", web::patch().to(exam_types::bulk_update_exam_types)),
    )
    .service(
        web::scope("/exams")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::ExamManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(exams::create_exam))
            .route("/{id}", web::get().to(exams::get_exam_by_id))
            .route("", web::get().to(exams::get_all_exams))
            .route(
                "/term/{term_id}",
                web::get().to(exams::get_exams_by_term_id),
            )
            .route("/{id}", web::put().to(exams::update_exam))
            .route("/{id}", web::delete().to(exams::delete_exam))
            .route("/bulk", web::delete().to(exams::bulk_delete_exams))
            .route("/bulk", web::patch().to(exams::bulk_update_exams)),
    )
    .service(
        web::scope("/exam-subjects")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::ExamSubjectManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(exam_subjects::create_exam_subject))
            .route(
                "/{exam_id}/{subject_id}",
                web::get().to(exam_subjects::get_exam_subject_by_ids),
            )
            .route("", web::get().to(exam_subjects::get_all_exam_subjects))
            .route(
                "/exam/{exam_id}",
                web::get().to(exam_subjects::get_exam_subjects_by_exam_id),
            )
            .route(
                "/subject/{subject_id}",
                web::get().to(exam_subjects::get_exam_subjects_by_subject_id),
            )
            .route(
                "/schedule/academic-year/{academic_year_id}/term/{term_id}",
                web::get().to(exam_subjects::get_exam_schedule),
            )
            .route(
                "/{exam_id}/{subject_id}",
                web::put().to(exam_subjects::update_exam_subject),
            )
            .route(
                "/{exam_id}/{subject_id}",
                web::delete().to(exam_subjects::delete_exam_subject),
            ),
    )
    .service(
        web::scope("/grading-schemes")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::GradingSchemeManage,
            })
            .wrap(Authenticated)
            .route(
                "",
                web::post().to(grading_schemes::create_grading_scheme_handler),
            )
            .route(
                "",
                web::get().to(grading_schemes::get_all_grading_schemes_handler),
            )
            .route(
                "/{id}",
                web::get().to(grading_schemes::get_grading_scheme_by_id_handler),
            )
            .route(
                "/{id}",
                web::put().to(grading_schemes::update_grading_scheme_handler),
            )
            .route(
                "/{id}",
                web::delete().to(grading_schemes::delete_grading_scheme_handler),
            )
            .route(
                "/{scheme_id}/assign_grade_level/{grade_level_id}",
                web::put().to(grading_schemes::assign_grading_scheme_to_grade_level_handler),
            ),
    )
    .service(
        web::scope("/grading-criteria")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::GradingCriterionManage,
            })
            .wrap(Authenticated)
            .route(
                "",
                web::post().to(grading_criteria::create_grading_criterion_handler),
            )
            .route(
                "/{id}",
                web::get().to(grading_criteria::get_grading_criterion_by_id_handler),
            )
            .route(
                "/{id}",
                web::put().to(grading_criteria::update_grading_criterion_handler),
            )
            .route(
                "/{id}",
                web::delete().to(grading_criteria::delete_grading_criterion_handler),
            ),
    )
    .service(
        web::scope("/grading-schemes/{scheme_id}/criteria")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::GradingSchemeManage,
            })
            .wrap(Authenticated)
            .route(
                "",
                web::get().to(grading_criteria::get_grading_criteria_by_scheme_id_handler),
            ),
    );

    cfg.configure(|cfg_local| zscore::config(cfg_local));
    cfg.configure(|cfg_local| special_exams::config(cfg_local));
    cfg.configure(|cfg_local| report_cards::config(cfg_local));
}
