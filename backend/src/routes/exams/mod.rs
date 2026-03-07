use crate::database::enums::PermissionEnum;
use crate::handlers::exams::{
    exam_structures, government_exams, grading_schemes, marking_schemes, report_cards,
    school_tests, zscore,
};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/exam-structures")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::ExamTypeManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(exam_structures::create_exam_structure))
            .route("/{id}", web::get().to(exam_structures::get_exam_structure_by_id))
            .route("", web::get().to(exam_structures::get_all_exam_structures))
            .route("/{id}", web::put().to(exam_structures::update_exam_structure))
            .route("/{id}", web::delete().to(exam_structures::delete_exam_structure))
            .route("/bulk", web::delete().to(exam_structures::bulk_delete_exam_structures))
            .route("/bulk", web::patch().to(exam_structures::bulk_update_exam_structures))
            .route(
                "/{structure_id}/subjects",
                web::post().to(exam_structures::create_exam_structure_subject),
            )
            .route(
                "/{structure_id}/subjects",
                web::get().to(exam_structures::get_exam_structure_subjects_by_structure),
            )
            .route(
                "/subjects/{id}",
                web::get().to(exam_structures::get_exam_structure_subject_by_id),
            )
            .route(
                "/subjects/{id}",
                web::put().to(exam_structures::update_exam_structure_subject),
            )
            .route(
                "/subjects/{id}",
                web::delete().to(exam_structures::delete_exam_structure_subject),
            ),
    )
    .service(
        web::scope("/government-exams")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::ExamManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(government_exams::create_government_exam))
            .route("/{id}", web::get().to(government_exams::get_government_exam_by_id))
            .route("", web::get().to(government_exams::get_all_government_exams))
            .route("/{id}", web::put().to(government_exams::update_government_exam))
            .route("/{id}", web::delete().to(government_exams::delete_government_exam))
            .route("/bulk", web::delete().to(government_exams::bulk_delete_government_exams))
            .route("/bulk", web::patch().to(government_exams::bulk_update_government_exams))
            .route(
                "/{government_exam_id}/subjects",
                web::post().to(government_exams::create_government_exam_subject),
            )
            .route(
                "/{government_exam_id}/subjects",
                web::get().to(government_exams::get_government_exam_subjects_by_exam),
            )
            .route(
                "/subjects/{id}",
                web::get().to(government_exams::get_government_exam_subject_by_id),
            )
            .route(
                "/subjects/{id}",
                web::put().to(government_exams::update_government_exam_subject),
            )
            .route(
                "/subjects/{id}",
                web::delete().to(government_exams::delete_government_exam_subject),
            ),
    )
    .service(
        web::scope("/school-tests")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::ExamManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(school_tests::create_school_test))
            .route("/{id}", web::get().to(school_tests::get_school_test_by_id))
            .route("", web::get().to(school_tests::get_all_school_tests))
            .route("/{id}", web::put().to(school_tests::update_school_test))
            .route("/{id}", web::delete().to(school_tests::delete_school_test))
            .route("/bulk", web::delete().to(school_tests::bulk_delete_school_tests))
            .route("/bulk", web::patch().to(school_tests::bulk_update_school_tests))
            .route(
                "/{school_test_id}/subjects",
                web::post().to(school_tests::create_school_test_subject),
            )
            .route(
                "/{school_test_id}/subjects",
                web::get().to(school_tests::get_school_test_subjects_by_test),
            )
            .route(
                "/subjects/{id}",
                web::get().to(school_tests::get_school_test_subject_by_id),
            )
            .route(
                "/subjects/{id}",
                web::put().to(school_tests::update_school_test_subject),
            )
            .route(
                "/subjects/{id}",
                web::delete().to(school_tests::delete_school_test_subject),
            ),
    )
    .service(
        web::scope("/marking-schemes")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::ExamManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(marking_schemes::create_marking_scheme))
            .route("/{id}", web::get().to(marking_schemes::get_marking_scheme_by_id))
            .route("", web::get().to(marking_schemes::get_all_marking_schemes))
            .route("/{id}", web::put().to(marking_schemes::update_marking_scheme))
            .route("/{id}", web::delete().to(marking_schemes::delete_marking_scheme))
            .route("/bulk", web::delete().to(marking_schemes::bulk_delete_marking_schemes))
            .route("/bulk", web::patch().to(marking_schemes::bulk_update_marking_schemes))
            .route(
                "/{scheme_id}/parts",
                web::post().to(marking_schemes::create_marking_scheme_part),
            )
            .route(
                "/{scheme_id}/parts",
                web::get().to(marking_schemes::get_marking_scheme_parts_by_scheme),
            )
            .route(
                "/parts/{id}",
                web::get().to(marking_schemes::get_marking_scheme_part_by_id),
            )
            .route(
                "/parts/{id}",
                web::put().to(marking_schemes::update_marking_scheme_part),
            )
            .route(
                "/parts/{id}",
                web::delete().to(marking_schemes::delete_marking_scheme_part),
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
    );

    cfg.configure(|cfg_local| zscore::config(cfg_local));
    cfg.configure(|cfg_local| report_cards::config(cfg_local));
}
