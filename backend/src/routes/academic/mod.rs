use crate::database::enums::PermissionEnum;
use crate::handlers::academic as academic_handlers;
use crate::handlers::academic::academic_year::get_current_academic_year;
use crate::handlers::admin_db_crud;
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
            .route("", web::post().to(academic_handlers::create_academic_year))
            .route("/current", web::get().to(get_current_academic_year))
            .route("/{id}", web::get().to(academic_handlers::get_academic_year_by_id))
            .route("", web::get().to(academic_handlers::get_all_academic_year))
            .route("/{id}", web::put().to(academic_handlers::update_academic_year))
            .route("/{id}", web::delete().to(academic_handlers::delete_academic_year))
            .route("/bulk", web::delete().to(academic_handlers::bulk_delete_academic_year))
            .route("/bulk", web::patch().to(academic_handlers::bulk_update_academic_year)),
    )
    .service(
        web::scope("/terms")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::TermManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(academic_handlers::create_term))
            .route("/{id}", web::get().to(academic_handlers::get_term_by_id))
            .route("", web::get().to(academic_handlers::get_all_term))
            .route("/{id}", web::put().to(academic_handlers::update_term))
            .route("/{id}", web::delete().to(academic_handlers::delete_term))
            .route("/bulk", web::delete().to(academic_handlers::bulk_delete_term))
            .route("/bulk", web::patch().to(academic_handlers::bulk_update_term)),
    )
    .service(
        web::scope("/grade-levels")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::GradeLevelManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(academic_handlers::create_grade_level))
            .route("/{id}", web::get().to(academic_handlers::get_grade_level_by_id))
            .route("", web::get().to(academic_handlers::get_all_grade_level))
            .route("/{id}", web::put().to(academic_handlers::update_grade_level))
            .route("/{id}", web::delete().to(academic_handlers::delete_grade_level))
            .route("/bulk", web::delete().to(academic_handlers::bulk_delete_grade_level))
            .route("/bulk", web::patch().to(academic_handlers::bulk_update_grade_level)),
    )
    .service(
        web::scope("/grade-periods")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::GradeLevelManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(academic_handlers::create_grade_period))
            .route("/{id}", web::get().to(academic_handlers::get_grade_period_by_id))
            .route("", web::get().to(academic_handlers::get_all_grade_period))
            .route("/{id}", web::put().to(academic_handlers::update_grade_period))
            .route("/{id}", web::delete().to(academic_handlers::delete_grade_period))
            .route("/bulk", web::delete().to(academic_handlers::bulk_delete_grade_period))
            .route("/bulk", web::patch().to(academic_handlers::bulk_update_grade_period)),
    )
    .service(
        web::scope("/classes")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::ClassManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(academic_handlers::create_class))
            .route("/{id}", web::get().to(academic_handlers::get_class_by_id))
            .route("", web::get().to(academic_handlers::get_all_class))
            .route("/{id}", web::put().to(academic_handlers::update_class))
            .route("/{id}", web::delete().to(academic_handlers::delete_class))
            .route("/bulk", web::delete().to(academic_handlers::bulk_delete_class))
            .route("/bulk", web::patch().to(academic_handlers::bulk_update_class)),
    )
    .service(
        web::scope("/subjects")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::SubjectManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(academic_handlers::create_subject))
            .route("/{id}", web::get().to(academic_handlers::get_subject_by_id))
            .route("", web::get().to(academic_handlers::get_all_subject))
            .route("/{id}", web::put().to(academic_handlers::update_subject))
            .route("/{id}", web::delete().to(academic_handlers::delete_subject))
            .route("/bulk", web::delete().to(academic_handlers::bulk_delete_subject))
            .route("/bulk", web::patch().to(academic_handlers::bulk_update_subject)),
    )
    .service(
        web::scope("/school-rooms")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::AcademicYearManage, // Placeholder
            })
            .wrap(Authenticated)
            .route("", web::post().to(academic_handlers::create_school_room))
            .route("/{id}", web::get().to(academic_handlers::get_school_room_by_id))
            .route("", web::get().to(academic_handlers::get_all_school_room))
            .route("/{id}", web::put().to(academic_handlers::update_school_room))
            .route("/{id}", web::delete().to(academic_handlers::delete_school_room))
            .route("/bulk", web::delete().to(academic_handlers::bulk_delete_school_room))
            .route("/bulk", web::patch().to(academic_handlers::bulk_update_school_room)),
    )
    .service(
        web::scope("/timetable")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::TimetableManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(academic_handlers::create_timetable))
            .route("/{id}", web::get().to(academic_handlers::get_timetable_by_id))
            .route("", web::get().to(academic_handlers::get_all_timetable))
            .route("/{id}", web::put().to(academic_handlers::update_timetable))
            .route("/{id}", web::delete().to(academic_handlers::delete_timetable))
            .route("/bulk", web::delete().to(academic_handlers::bulk_delete_timetable))
            .route("/bulk", web::patch().to(academic_handlers::bulk_update_timetable)),
    )
    .service(
        web::scope("/al-streams")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::AlStreamManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(academic_handlers::create_al_stream))
            .route("/{id}", web::get().to(academic_handlers::get_al_stream_by_id))
            .route("", web::get().to(academic_handlers::get_all_al_stream))
            .route("/{id}", web::put().to(academic_handlers::update_al_stream))
            .route("/{id}", web::delete().to(academic_handlers::delete_al_stream))
            .route("/bulk", web::delete().to(academic_handlers::bulk_delete_al_stream))
            .route("/bulk", web::patch().to(academic_handlers::bulk_update_al_stream)),
    )
    .service(
        web::scope("/al-stream-optional-groups")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::AlStreamManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(academic_handlers::create_al_stream_optional_group))
            .route("/{id}", web::get().to(academic_handlers::get_al_stream_optional_group_by_id))
            .route("", web::get().to(academic_handlers::get_all_al_stream_optional_group))
            .route("/{id}", web::put().to(academic_handlers::update_al_stream_optional_group))
            .route("/{id}", web::delete().to(academic_handlers::delete_al_stream_optional_group))
            .route("/bulk", web::delete().to(academic_handlers::bulk_delete_al_stream_optional_group))
            .route("/bulk", web::patch().to(academic_handlers::bulk_update_al_stream_optional_group)),
    )
    .service(
        web::scope("/al-stream-required-subjects")
            .wrap(PermissionVerification { required_permission: PermissionEnum::AlStreamManage })
            .wrap(Authenticated)
            .route("", web::post().to(admin_db_crud::create_al_stream_required_subject))
            .route("/{stream_id}/{subject_id}", web::get().to(admin_db_crud::get_al_stream_required_subject_by_id))
            .route("", web::get().to(admin_db_crud::get_all_al_stream_required_subject))
            .route("/{stream_id}/{subject_id}", web::delete().to(admin_db_crud::delete_al_stream_required_subject)),
    )
    .service(
        web::scope("/al-stream-optional-subjects")
            .wrap(PermissionVerification { required_permission: PermissionEnum::AlStreamManage })
            .wrap(Authenticated)
            .route("", web::post().to(admin_db_crud::create_al_stream_optional_subject))
            .route("/{group_id}/{subject_id}", web::get().to(admin_db_crud::get_al_stream_optional_subject_by_id))
            .route("", web::get().to(admin_db_crud::get_all_al_stream_optional_subject))
            .route("/{group_id}/{subject_id}", web::delete().to(admin_db_crud::delete_al_stream_optional_subject)),
    )
    .service(
        web::scope("/grade-subjects")
            .wrap(PermissionVerification { required_permission: PermissionEnum::GradeLevelManage })
            .wrap(Authenticated)
            .route("", web::post().to(admin_db_crud::create_grade_subject))
            .route("/{grade_id}/{subject_id}", web::get().to(admin_db_crud::get_grade_subject_by_id))
            .route("", web::get().to(admin_db_crud::get_all_grade_subject))
            .route("/{grade_id}/{subject_id}", web::delete().to(admin_db_crud::delete_grade_subject)),
    )
    .service(
        web::scope("/class-subject-teachers")
            .wrap(PermissionVerification { required_permission: PermissionEnum::ClassSubjectTeacherManage })
            .wrap(Authenticated)
            .route("", web::post().to(academic_handlers::class_subject_teacher::assign_subject_teacher_to_class))
            .route("/{class_id}/{subject_id}/{academic_year_id}", web::put().to(academic_handlers::class_subject_teacher::update_subject_teacher_assignment))
            .route("/{class_id}/{subject_id}/{teacher_id}/{academic_year_id}", web::delete().to(academic_handlers::class_subject_teacher::remove_subject_teacher_assignment))
            .route("/class/{class_id}/{academic_year_id}", web::get().to(academic_handlers::class_subject_teacher::get_subjects_by_class))
            .route("/teacher/{teacher_id}/{academic_year_id}", web::get().to(academic_handlers::class_subject_teacher::get_classes_by_teacher)),
    )
    .service(
        web::scope("/teacher-assignments")
            .wrap(PermissionVerification { required_permission: PermissionEnum::ClassManage })
            .wrap(Authenticated)
            .route("/classes", web::get().to(academic_handlers::teacher_assignments::get_teacher_classes))
            .route("/subjects", web::get().to(academic_handlers::teacher_assignments::get_teacher_subjects))
            .route("/timetable", web::get().to(academic_handlers::teacher_assignments::get_teacher_timetable)),
    )
    .service(
        web::scope("/substitution-plans")

            .wrap(PermissionVerification {
                required_permission: PermissionEnum::TimetableManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(academic_handlers::create_substitution_plan_handler))
            .route("", web::get().to(academic_handlers::get_substitution_plans_handler))
            .route("/{id}", web::get().to(academic_handlers::get_substitution_plan_by_id))
            .route("/{id}", web::delete().to(academic_handlers::delete_substitution_plan))
            .route("/bulk", web::delete().to(academic_handlers::bulk_delete_substitution_plan)),
    );
}
