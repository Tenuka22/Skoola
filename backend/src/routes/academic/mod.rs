use apistos::web;
use crate::handlers::academic::{academic_year, class, class_subject_teacher, grade_level, subject, terms, timetable};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use crate::database::enums::PermissionEnum;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/academic-years")
            .wrap(PermissionVerification { required_permission: PermissionEnum::AcademicYearManage })
            .wrap(Authenticated)
            .route("", web::post().to(academic_year::create_academic_year))
            .route("/{id}", web::get().to(academic_year::get_academic_year_by_id))
            .route("", web::get().to(academic_year::get_all_academic_years))
            .route("/{id}", web::put().to(academic_year::update_academic_year))
            .route("/{id}", web::delete().to(academic_year::delete_academic_year))
            .route("/{id}/set-current", web::put().to(academic_year::set_current_academic_year))
            .route("/bulk", web::delete().to(academic_year::bulk_delete_academic_years))
            .route("/bulk", web::patch().to(academic_year::bulk_update_academic_years)),
    )
    .service(
        web::scope("/terms")
            .wrap(PermissionVerification { required_permission: PermissionEnum::TermManage })
            .wrap(Authenticated)
            .route("", web::post().to(terms::create_term_handler)),
    )
    .service(
        web::scope("/grade-levels")
            .wrap(PermissionVerification { required_permission: PermissionEnum::GradeLevelManage })
            .wrap(Authenticated)
            .route("", web::post().to(grade_level::create_grade_level))
            .route("/{id}", web::get().to(grade_level::get_grade_level_by_id))
            .route("", web::get().to(grade_level::get_all_grade_levels))
            .route("/{id}", web::put().to(grade_level::update_grade_level))
            .route("/{id}", web::delete().to(grade_level::delete_grade_level))
            .route("/bulk", web::delete().to(grade_level::bulk_delete_grade_levels))
            .route("/bulk", web::patch().to(grade_level::bulk_update_grade_levels)),
    )
    .service(
        web::scope("/classes")
            .wrap(PermissionVerification { required_permission: PermissionEnum::ClassManage })
            .wrap(Authenticated)
            .route("", web::post().to(class::create_class))
            .route("/{id}", web::get().to(class::get_class_by_id))
            .route("", web::get().to(class::get_all_classes))
            .route("/{id}", web::put().to(class::update_class))
            .route("/{id}", web::delete().to(class::delete_class))
            .route("/grade/{id}", web::get().to(class::get_classes_by_grade))
            .route("/bulk", web::delete().to(class::bulk_delete_classes))
            .route("/bulk", web::patch().to(class::bulk_update_classes)),
    )
    .service(
        web::scope("/subjects")
            .wrap(PermissionVerification { required_permission: PermissionEnum::SubjectManage })
            .wrap(Authenticated)
            .route("", web::post().to(subject::create_subject))
            .route("/{id}", web::get().to(subject::get_subject_by_id))
            .route("", web::get().to(subject::get_all_subjects))
            .route("/{id}", web::put().to(subject::update_subject))
            .route("/{id}", web::delete().to(subject::delete_subject))
            .route("/grade/{grade_id}", web::get().to(subject::get_subjects_by_grade_handler))
            .route("/stream/{stream_id}", web::get().to(subject::get_subjects_by_stream_handler))
            .route("/assign-to-grade", web::post().to(subject::assign_subject_to_grade_handler))
            .route("/assign-to-stream", web::post().to(subject::assign_subject_to_stream_handler))
            .route("/enroll", web::post().to(subject::enroll_student_in_subject))
            .route("/enrollments/{student_id}/{academic_year_id}", web::get().to(subject::get_student_enrollments))
            .route("/bulk", web::delete().to(subject::bulk_delete_subjects))
            .route("/bulk", web::patch().to(subject::bulk_update_subjects)),
    )
    .service(
        web::scope("/class-subject-teachers")
            .wrap(PermissionVerification { required_permission: PermissionEnum::ClassSubjectTeacherManage })
            .wrap(Authenticated)
            .route("", web::post().to(class_subject_teacher::assign_subject_teacher_to_class))
            .route("/{class_id}/{subject_id}/{academic_year_id}", web::put().to(class_subject_teacher::update_subject_teacher_assignment))
            .route("/{class_id}/{subject_id}/{teacher_id}/{academic_year_id}", web::delete().to(class_subject_teacher::remove_subject_teacher_assignment))
            .route("/class/{class_id}/academic-year/{academic_year_id}/subjects", web::get().to(class_subject_teacher::get_subjects_by_class))
            .route("/teacher/{teacher_id}/academic-year/{academic_year_id}/classes", web::get().to(class_subject_teacher::get_classes_by_teacher)),
    )
    .service(
        web::scope("/timetables")
            .wrap(PermissionVerification { required_permission: PermissionEnum::TimetableManage })
            .wrap(Authenticated)
            .route("", web::post().to(timetable::create_timetable_entry))
            .route("/{id}", web::get().to(timetable::get_timetable_entry_by_id))
            .route("/class/{class_id}/day/{day_of_week}/academic-year/{academic_year_id}", web::get().to(timetable::get_timetable_by_class_and_day))
            .route("/teacher/{teacher_id}/academic-year/{academic_year_id}", web::get().to(timetable::get_timetable_by_teacher))
            .route("/{id}", web::put().to(timetable::update_timetable_entry))
            .route("/{id}", web::delete().to(timetable::delete_timetable_entry)),
    );
}
