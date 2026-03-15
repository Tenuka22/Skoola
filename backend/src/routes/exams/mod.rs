use crate::database::enums::PermissionEnum;
use crate::handlers::exams::{
    exam_structure_subjects, exam_structures, exam_subjects, exam_types, exams, government_exams,
    grading_criteria, grading_schemes, marking_schemes, school_tests, special_exams,
    zscore,
};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/exams")
            .wrap(Authenticated)
            .service(
                web::scope("/types")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ExamTypeManage })
                    .route("", web::post().to(exam_types::create_exam_type))
                    .route("/{id}", web::get().to(exam_types::get_exam_type_by_id))
                    .route("", web::get().to(exam_types::get_all_exam_type))
                    .route("/{id}", web::put().to(exam_types::update_exam_type))
                    .route("/{id}", web::delete().to(exam_types::delete_exam_type))
                    .route("/bulk", web::delete().to(exam_types::bulk_delete_exam_type))
                    .route("/bulk", web::patch().to(exam_types::bulk_update_exam_type)),
            )
            .service(
                web::scope("/records")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ExamManage })
                    .route("", web::post().to(exams::create_exam))
                    .route("/{id}", web::get().to(exams::get_exam_by_id))
                    .route("", web::get().to(exams::get_all_exam))
                    .route("/{id}", web::put().to(exams::update_exam))
                    .route("/{id}", web::delete().to(exams::delete_exam))
                    .route("/bulk", web::delete().to(exams::bulk_delete_exam))
                    .route("/bulk", web::patch().to(exams::bulk_update_exam)),
            )
            .service(
                web::scope("/structures")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ExamTypeManage })
                    .route("", web::post().to(exam_structures::create_exam_structure))
                    .route("/{id}", web::get().to(exam_structures::get_exam_structure_by_id))
                    .route("", web::get().to(exam_structures::get_all_exam_structure))
                    .route("/{id}", web::put().to(exam_structures::update_exam_structure))
                    .route("/{id}", web::delete().to(exam_structures::delete_exam_structure)),
            )
            .service(
                web::scope("/structure-subjects")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ExamTypeManage })
                    .wrap(Authenticated)
                    .route("", web::post().to(exam_structure_subjects::create_exam_structure_subject))
                    .route("/{id}", web::get().to(exam_structure_subjects::get_exam_structure_subject_by_id))
                    .route("", web::get().to(exam_structure_subjects::get_all_exam_structure_subject))
                    .route("/{id}", web::put().to(exam_structure_subjects::update_exam_structure_subject))
                    .route("/{id}", web::delete().to(exam_structure_subjects::delete_exam_structure_subject))
                    .route("/bulk", web::delete().to(exam_structure_subjects::bulk_delete_exam_structure_subject))
                    .route("/bulk", web::patch().to(exam_structure_subjects::bulk_update_exam_structure_subject)),
            )
            .service(
                web::scope("/subjects")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ExamSubjectManage })
                    .wrap(Authenticated)
                    .route("", web::post().to(exam_subjects::create_exam_subject))
                    .route("/{id}", web::get().to(exam_subjects::get_exam_subject_by_id))
                    .route("", web::get().to(exam_subjects::get_all_exam_subject))
                    .route("/{id}", web::put().to(exam_subjects::update_exam_subject))
                    .route("/{id}", web::delete().to(exam_subjects::delete_exam_subject))
                    .route("/bulk", web::delete().to(exam_subjects::bulk_delete_exam_subject))
                    .route("/bulk", web::patch().to(exam_subjects::bulk_update_exam_subject)),
            )
            .service(
                web::scope("/government-exams")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ExamManage })
                    .route("", web::post().to(government_exams::create_government_exam))
                    .route("/{id}", web::get().to(government_exams::get_government_exam_by_id))
                    .route("", web::get().to(government_exams::get_all_government_exam))
                    .route("/{id}", web::put().to(government_exams::update_government_exam))
                    .route("/{id}", web::delete().to(government_exams::delete_government_exam))
                    .route("/bulk", web::delete().to(government_exams::bulk_delete_government_exam))
                    .route("/bulk", web::patch().to(government_exams::bulk_update_government_exam)),
            )
            .service(
                web::scope("/government-exam-subjects")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ExamManage })
                    .route("", web::post().to(government_exams::create_government_exam_subject))
                    .route("/{id}", web::get().to(government_exams::get_government_exam_subject_by_id))
                    .route("", web::get().to(government_exams::get_all_government_exam_subject))
                    .route("/{id}", web::put().to(government_exams::update_government_exam_subject))
                    .route("/{id}", web::delete().to(government_exams::delete_government_exam_subject))
                    .route("/bulk", web::delete().to(government_exams::bulk_delete_government_exam_subject))
                    .route("/bulk", web::patch().to(government_exams::bulk_update_government_exam_subject)),
            )
            .service(
                web::scope("/school-tests")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ExamManage })
                    .route("", web::post().to(school_tests::create_school_test))
                    .route("/{id}", web::get().to(school_tests::get_school_test_by_id))
                    .route("", web::get().to(school_tests::get_all_school_test))
                    .route("/{id}", web::put().to(school_tests::update_school_test))
                    .route("/{id}", web::delete().to(school_tests::delete_school_test))
                    .route("/bulk", web::delete().to(school_tests::bulk_delete_school_test))
                    .route("/bulk", web::patch().to(school_tests::bulk_update_school_test)),
            )
            .service(
                web::scope("/school-test-subjects")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ExamManage })
                    .route("", web::post().to(school_tests::create_school_test_subject))
                    .route("/{id}", web::get().to(school_tests::get_school_test_subject_by_id))
                    .route("", web::get().to(school_tests::get_all_school_test_subject))
                    .route("/{id}", web::put().to(school_tests::update_school_test_subject))
                    .route("/{id}", web::delete().to(school_tests::delete_school_test_subject))
                    .route("/bulk", web::delete().to(school_tests::bulk_delete_school_test_subject))
                    .route("/bulk", web::patch().to(school_tests::bulk_update_school_test_subject)),
            )
            .service(
                web::scope("/marking-schemes")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ExamManage })
                    .route("", web::post().to(marking_schemes::create_marking_scheme))
                    .route("/{id}", web::get().to(marking_schemes::get_marking_scheme_by_id))
                    .route("", web::get().to(marking_schemes::get_all_marking_scheme))
                    .route("/{id}", web::put().to(marking_schemes::update_marking_scheme))
                    .route("/{id}", web::delete().to(marking_schemes::delete_marking_scheme))
                    .route("/bulk", web::delete().to(marking_schemes::bulk_delete_marking_scheme))
                    .route("/bulk", web::patch().to(marking_schemes::bulk_update_marking_scheme)),
            )
            .service(
                web::scope("/marking-scheme-parts")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ExamManage })
                    .route("", web::post().to(marking_schemes::create_marking_scheme_part))
                    .route("/{id}", web::get().to(marking_schemes::get_marking_scheme_part_by_id))
                    .route("", web::get().to(marking_schemes::get_all_marking_scheme_part))
                    .route("/{id}", web::put().to(marking_schemes::update_marking_scheme_part))
                    .route("/{id}", web::delete().to(marking_schemes::delete_marking_scheme_part))
                    .route("/bulk", web::delete().to(marking_schemes::bulk_delete_marking_scheme_part))
                    .route("/bulk", web::patch().to(marking_schemes::bulk_update_marking_scheme_part)),
            )
            .service(
                web::scope("/grading-schemes")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ExamManage })
                    .route("", web::post().to(grading_schemes::create_grading_scheme))
                    .route("/{id}", web::get().to(grading_schemes::get_grading_scheme_by_id))
                    .route("", web::get().to(grading_schemes::get_all_grading_scheme))
                    .route("/{id}", web::put().to(grading_schemes::update_grading_scheme))
                    .route("/{id}", web::delete().to(grading_schemes::delete_grading_scheme))
                    .route("/bulk", web::delete().to(grading_schemes::bulk_delete_grading_scheme))
                    .route("/bulk", web::patch().to(grading_schemes::bulk_update_grading_scheme)),
            )
            .service(
                web::scope("/grading-criteria")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ExamManage })
                    .route("", web::post().to(grading_criteria::create_grading_criterion))
                    .route("/{id}", web::get().to(grading_criteria::get_grading_criterion_by_id))
                    .route("", web::get().to(grading_criteria::get_all_grading_criterion))
                    .route("/{id}", web::put().to(grading_criteria::update_grading_criterion))
                    .route("/{id}", web::delete().to(grading_criteria::delete_grading_criterion))
                    .route("/bulk", web::delete().to(grading_criteria::bulk_delete_grading_criterion))
                    .route("/bulk", web::patch().to(grading_criteria::bulk_update_grading_criterion)),
            )
    );

    special_exams::config(cfg);
    zscore::config(cfg);
}
