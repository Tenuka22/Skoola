use crate::{
    database::enums::{PermissionEnum},
    handlers::{
        academic_year,
        auth::{login, logout, refresh, register, request_password_reset, reset_password},
        class, class_subject_teacher, exam_subjects, exam_types, exams, fees, grade_level,
        grading_criteria, grading_schemes,
        hello::{hello, hello_error},
        oauth::{github_callback, google_callback},
        permissions::{
            create_permission, delete_permission, get_permission, get_permissions,
            update_permission,
        },
        profile::{
            change_email, change_password, get_profile, link_github, link_google, update_profile,
        },
        report_cards,
        special_exams,
        staff::{
            create_staff, delete_staff, get_all_staff, get_staff_by_id, update_staff,
            upload_staff_photo,
        },
        staff_attendance::{
            calculate_monthly_attendance_percentage, get_staff_attendance_by_date,
            get_staff_attendance_by_staff_member, mark_bulk_staff_attendance,
            mark_staff_attendance_daily, update_staff_attendance,
        },
        staff_leaves::{apply_for_leave, approve_reject_leave, view_leave_balance},
        student, student_attendance, student_class_assignment, student_guardian, student_marks,
        subject,
        teacher_assignments::{
            assign_class_to_teacher, assign_subject_to_teacher, get_teacher_workload,
        },
        terms, timetable,
        user_permissions::{
            assign_permission_to_user, get_user_permissions, unassign_permission_from_user,
        },
        permission_sets,
        verification::verify_email,
        zscore,
    },
    utils::{jwt::Authenticated, permission_verification::PermissionVerification},
};
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/refresh", web::post().to(refresh))
            .route("/password/request", web::post().to(request_password_reset))
            .route("/password/reset/{token}", web::post().to(reset_password))
            .route("/google/callback", web::get().to(google_callback))
            .route("/github/callback", web::get().to(github_callback))
            .route("/verify-email/{token}", web::get().to(verify_email)),
    )
    .service(
        web::scope("/profile")
            .wrap(Authenticated)
            .route("", web::get().to(get_profile))
            .route("", web::put().to(update_profile))
            .route("/password", web::post().to(change_password))
            .route("/email", web::post().to(change_email))
            .route("/link/google", web::get().to(link_google))
            .route("/link/github", web::get().to(link_github)),
    )
    .service(
        web::scope("/users")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::UserManage,
            })
            .wrap(Authenticated)
            .route("", web::get().to(crate::handlers::users::get_all_users))
            .route("/stats", web::get().to(crate::handlers::users::get_user_stats))
            .route("/bulk", web::delete().to(crate::handlers::users::bulk_delete_users))
            .route("/bulk", web::patch().to(crate::handlers::users::bulk_update_users))
            .route("/{user_id}", web::delete().to(crate::handlers::users::delete_user))
            .route("/{user_id}", web::patch().to(crate::handlers::users::update_user))
            .route("/{user_id}/permissions", web::get().to(get_user_permissions))
            .route(
                "/{user_id}/permissions/{permission_id}",
                web::post().to(assign_permission_to_user),
            )
            .route(
                "/{user_id}/permissions/{permission_id}",
                web::delete().to(unassign_permission_from_user),
            ),
    )
    .service(
        web::scope("/permissions")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::PermissionManage,
            })
            .wrap(Authenticated)
            .route("", web::get().to(get_permissions))
            .route("", web::post().to(create_permission))
            .route("/{permission_id}", web::get().to(get_permission))
            .route("/{permission_id}", web::put().to(update_permission))
            .route("/{permission_id}", web::delete().to(delete_permission)),
    )
    .service(
        web::scope("/permission-sets")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::PermissionSetManage,
            })
            .wrap(Authenticated)
            .route("", web::get().to(permission_sets::get_all_permission_sets))
            .route("", web::post().to(permission_sets::create_permission_set))
            .route(
                "/{permission_set_id}",
                web::get().to(permission_sets::get_permission_set_by_id),
            )
            .route(
                "/{permission_set_id}",
                web::put().to(permission_sets::update_permission_set),
            )
            .route(
                "/{permission_set_id}",
                web::delete().to(permission_sets::delete_permission_set),
            )
            .route(
                "/{permission_set_id}/permissions/{permission_id}",
                web::post().to(permission_sets::assign_permission_to_permission_set),
            )
            .route(
                "/{permission_set_id}/permissions/{permission_id}",
                web::delete().to(permission_sets::unassign_permission_from_permission_set),
            ),
    )
    .service(
        web::scope("/staff")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManage,
            })
            .wrap(Authenticated)
            .route("", web::get().to(get_all_staff))
            .route("/{staff_id}", web::get().to(get_staff_by_id))
            .route("", web::post().to(create_staff))
            .route("/{staff_id}", web::put().to(update_staff))
            .route("/{staff_id}", web::delete().to(delete_staff))
            .route("/{staff_id}/photo", web::post().to(upload_staff_photo))
            .route(
                "/{teacher_id}/classes",
                web::post().to(assign_class_to_teacher),
            )
            .route(
                "/{teacher_id}/subjects",
                web::post().to(assign_subject_to_teacher),
            )
            .route(
                "/{teacher_id}/workload",
                web::get().to(get_teacher_workload),
            )
            .route(
                "/{staff_id}/attendance",
                web::post().to(mark_staff_attendance_daily),
            )
            .route(
                "/attendance/bulk",
                web::post().to(mark_bulk_staff_attendance),
            )
            .route(
                "/attendance/{attendance_id}",
                web::put().to(update_staff_attendance),
            )
            .route(
                "/attendance/date",
                web::get().to(get_staff_attendance_by_date),
            )
            .route(
                "/{staff_id}/attendance/member",
                web::get().to(get_staff_attendance_by_staff_member),
            )
            .route(
                "/{staff_id}/attendance/percentage/{year}/{month}",
                web::get().to(calculate_monthly_attendance_percentage),
            )
            .route("/{staff_id}/leaves", web::post().to(apply_for_leave))
            .route(
                "/leaves/{leave_id}/status",
                web::put().to(approve_reject_leave),
            )
            .route(
                "/{staff_id}/leaves/balance",
                web::get().to(view_leave_balance),
            )
            .route(
                "/{staff_id}/permission-sets",
                web::get().to(crate::handlers::permission_sets::get_staff_permission_sets),
            )
            .route(
                "/{staff_id}/permission-sets/{set_id}",
                web::post().to(crate::handlers::permission_sets::assign_permission_set_to_staff),
            )
            .route(
                "/{staff_id}/permission-sets/{set_id}",
                web::delete().to(crate::handlers::permission_sets::unassign_permission_set_from_staff),
            ),
    )
    .service(
        web::scope("/students")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StudentManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(student::create_student))
            .route("/{student_id}", web::put().to(student::update_student))
            .route("/{student_id}", web::get().to(student::get_student_by_id))
            .route("", web::get().to(student::get_all_students))
            .route("/search", web::get().to(student::search_students))
            .route("/filter", web::get().to(student::filter_students))
            .route("/{student_id}", web::delete().to(student::delete_student))
            .route(
                "/{student_id}/photo",
                web::post().to(student::upload_student_photo),
            )
            .route(
                "/{student_id}/current-class",
                web::get().to(student_class_assignment::get_current_class_of_student),
            )
            .route(
                "/{student_id}/class-history",
                web::get().to(student_class_assignment::get_class_history_of_student),
            ),
    )
    .service(
        web::scope("/students/{student_id}/guardians")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StudentManageGuardians,
            })
            .wrap(Authenticated)
            .route(
                "",
                web::post().to(student_guardian::add_guardian_to_student),
            )
            .route(
                "/{guardian_id}",
                web::put().to(student_guardian::update_guardian_information),
            )
            .route(
                "/{guardian_id}",
                web::delete().to(student_guardian::remove_guardian_from_student),
            )
            .route(
                "",
                web::get().to(student_guardian::get_all_guardians_for_student),
            ),
    )
    .service(
        web::scope("/student-class-assignments")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StudentManageEnrollment,
            })
            .wrap(Authenticated)
            .route(
                "",
                web::post().to(student_class_assignment::assign_student_to_class),
            )
            .route(
                "/{student_id}/{assignment_id}/transfer",
                web::put().to(student_class_assignment::transfer_student_class),
            )
            .route(
                "/bulk",
                web::post().to(student_class_assignment::bulk_assign_students_to_classes),
            )
            .route(
                "/promote",
                web::post().to(student_class_assignment::promote_student_to_next_grade),
            ),
    )
    .service(
        web::scope("/student-attendance")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StudentManageAttendance,
            })
            .wrap(Authenticated)
            .route(
                "/bulk",
                web::post().to(student_attendance::bulk_mark_student_attendance),
            )
            .route(
                "",
                web::post().to(student_attendance::mark_individual_student_attendance),
            )
            .route(
                "/{attendance_id}",
                web::put().to(student_attendance::update_student_attendance),
            )
            .route(
                "/class/{class_id}/date/{date}",
                web::get().to(student_attendance::get_attendance_by_class_and_date),
            )
            .route(
                "/student/{student_id}",
                web::get().to(student_attendance::get_attendance_by_student),
            )
            .route(
                "/student/{student_id}/percentage",
                web::get().to(student_attendance::calculate_student_attendance_percentage),
            )
            .route(
                "/report",
                web::get().to(student_attendance::generate_attendance_report),
            )
            .route(
                "/low-attendance",
                web::get().to(student_attendance::get_students_with_low_attendance),
            )
            .route(
                "/notifications/absent",
                web::post().to(student_attendance::send_absence_notifications),
            ),
    )
    .service(
        web::scope("/student-marks")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StudentManageMarks,
            })
            .wrap(Authenticated)
            .route("", web::post().to(student_marks::create_student_mark))
            .route("/{id}", web::get().to(student_marks::get_student_mark_by_id))
            .route("", web::get().to(student_marks::get_all_student_marks))
            .route(
                "/student/{student_id}",
                web::get().to(student_marks::get_student_marks_by_student_id),
            )
            .route(
                "/exam/{exam_id}/class/{class_id}",
                web::get().to(student_marks::get_student_marks_by_exam_and_class),
            )
            .route("/{id}", web::put().to(student_marks::update_student_mark))
            .route("/{id}", web::delete().to(student_marks::delete_student_mark))
            .route("/bulk", web::post().to(student_marks::bulk_create_student_marks)),
    )
    .service(
        web::scope("/academic-years")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::AcademicYearManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(academic_year::create_academic_year))
            .route(
                "/{id}",
                web::get().to(academic_year::get_academic_year_by_id),
            )
            .route("", web::get().to(academic_year::get_all_academic_years))
            .route("/{id}", web::put().to(academic_year::update_academic_year))
            .route(
                "/{id}",
                web::delete().to(academic_year::delete_academic_year),
            )
            .route(
                "/{id}/set-current",
                web::put().to(academic_year::set_current_academic_year),
            )
            .route("/bulk", web::delete().to(academic_year::bulk_delete_academic_years))
            .route("/bulk", web::patch().to(academic_year::bulk_update_academic_years)),
    )
    .service(
        web::scope("/terms")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::TermManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(terms::create_term_handler)),
    )
    .service(
        web::scope("/grade-levels")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::GradeLevelManage,
            })
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
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::ClassManage,
            })
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
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::SubjectManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(subject::create_subject))
            .route("/{id}", web::get().to(subject::get_subject_by_id))
            .route("", web::get().to(subject::get_all_subjects))
            .route("/{id}", web::put().to(subject::update_subject))
            .route("/{id}", web::delete().to(subject::delete_subject))
            .route(
                "/grade/{grade_id}",
                web::get().to(subject::get_subjects_by_grade_handler),
            )
            .route(
                "/stream/{stream_id}",
                web::get().to(subject::get_subjects_by_stream_handler),
            )
            .route(
                "/assign-to-grade",
                web::post().to(subject::assign_subject_to_grade_handler),
            )
            .route(
                "/assign-to-stream",
                web::post().to(subject::assign_subject_to_stream_handler),
            )
            .route("/bulk", web::delete().to(subject::bulk_delete_subjects))
            .route("/bulk", web::patch().to(subject::bulk_update_subjects)),
    )
    .service(
        web::scope("/class-subject-teachers")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::ClassSubjectTeacherManage,
            })
            .wrap(Authenticated)
            .route(
                "",
                web::post().to(class_subject_teacher::assign_subject_teacher_to_class),
            )
            .route(
                "/{class_id}/{subject_id}/{academic_year_id}",
                web::put().to(class_subject_teacher::update_subject_teacher_assignment),
            )
            .route(
                "/{class_id}/{subject_id}/{teacher_id}/{academic_year_id}",
                web::delete().to(class_subject_teacher::remove_subject_teacher_assignment),
            )
            .route(
                "/class/{class_id}/academic-year/{academic_year_id}/subjects",
                web::get().to(class_subject_teacher::get_subjects_by_class),
            )
            .route(
                "/teacher/{teacher_id}/academic-year/{academic_year_id}/classes",
                web::get().to(class_subject_teacher::get_classes_by_teacher),
            ),
    )
    .service(
        web::scope("/timetables")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::TimetableManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(timetable::create_timetable_entry))
            .route("/{id}", web::get().to(timetable::get_timetable_entry_by_id))
            .route(
                "/class/{class_id}/day/{day_of_week}/academic-year/{academic_year_id}",
                web::get().to(timetable::get_timetable_by_class_and_day),
            )
            .route(
                "/teacher/{teacher_id}/academic-year/{academic_year_id}",
                web::get().to(timetable::get_timetable_by_teacher),
            )
            .route("/{id}", web::put().to(timetable::update_timetable_entry))
            .route("/{id}", web::delete().to(timetable::delete_timetable_entry)),
    )
    .service(
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
            .route("/bulk", web::delete().to(exam_types::bulk_delete_exam_types))
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
            ) // New route
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
        apistos::web::scope("/grading-schemes")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::GradingSchemeManage,
            })
            .wrap(Authenticated)
            .route(
                "",
                apistos::web::post().to(grading_schemes::create_grading_scheme_handler),
            )
            .route(
                "",
                apistos::web::get().to(grading_schemes::get_all_grading_schemes_handler),
            )
            .route(
                "/{id}",
                apistos::web::get().to(grading_schemes::get_grading_scheme_by_id_handler),
            )
            .route(
                "/{id}",
                apistos::web::put().to(grading_schemes::update_grading_scheme_handler),
            )
            .route(
                "/{id}",
                apistos::web::delete().to(grading_schemes::delete_grading_scheme_handler),
            )
            .route(
                "/{scheme_id}/assign_grade_level/{grade_level_id}",
                apistos::web::put()
                    .to(grading_schemes::assign_grading_scheme_to_grade_level_handler),
            ),
    )
    .service(
        apistos::web::scope("/grading-criteria")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::GradingCriterionManage,
            })
            .wrap(Authenticated)
            .route(
                "",
                apistos::web::post().to(grading_criteria::create_grading_criterion_handler),
            )
            .route(
                "/{id}",
                apistos::web::get().to(grading_criteria::get_grading_criterion_by_id_handler),
            )
            .route(
                "/{id}",
                apistos::web::put().to(grading_criteria::update_grading_criterion_handler),
            )
            .route(
                "/{id}",
                apistos::web::delete().to(grading_criteria::delete_grading_criterion_handler),
            ),
    )
    .service(
        apistos::web::scope("/grading-schemes/{scheme_id}/criteria")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::GradingSchemeManage,
            })
            .wrap(Authenticated)
            .route(
                "",
                apistos::web::get().to(grading_criteria::get_grading_criteria_by_scheme_id_handler),
            ),
    );

    cfg.configure(zscore::config);
    cfg.configure(special_exams::config);
    cfg.configure(report_cards::config);
    cfg.configure(fees::config);
    cfg.configure(crate::handlers::co_curricular::config);

    // Library Management Routes
    cfg.service(
        apistos::web::scope("/library")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::LibraryManage,
            })
            .wrap(Authenticated)
            // Category routes
            .route(
                "/categories",
                apistos::web::get().to(crate::handlers::library::get_all_categories),
            )
            .route(
                "/categories",
                apistos::web::post().to(crate::handlers::library::create_category),
            )
            .route(
                "/categories/bulk",
                apistos::web::delete().to(crate::handlers::library::bulk_delete_library_categories),
            )
            .route(
                "/categories/bulk",
                apistos::web::patch().to(crate::handlers::library::bulk_update_library_categories),
            )
            // Book routes
            .route(
                "/books",
                apistos::web::get().to(crate::handlers::library::get_all_books),
            )
            .route(
                "/books/bulk",
                apistos::web::delete().to(crate::handlers::library::bulk_delete_library_books),
            )
            .route(
                "/books/bulk",
                apistos::web::patch().to(crate::handlers::library::bulk_update_library_books),
            )
            .route(
                "/books/search",
                apistos::web::get().to(crate::handlers::library::search_books),
            )
            .route(
                "/books/{book_id}",
                apistos::web::get().to(crate::handlers::library::get_book_by_id),
            )
            .route(
                "/books",
                apistos::web::post().to(crate::handlers::library::create_book),
            )
            .route(
                "/books/{book_id}",
                apistos::web::put().to(crate::handlers::library::update_book),
            )
            .route(
                "/books/{book_id}",
                apistos::web::delete().to(crate::handlers::library::delete_book),
            )
            .route(
                "/books/category/{category_id}",
                apistos::web::get().to(crate::handlers::library::get_books_by_category),
            )
            // Issue/Return routes
            .route(
                "/issues",
                apistos::web::post().to(crate::handlers::library::issue_book),
            )
            .route(
                "/issues/{issue_id}",
                apistos::web::get().to(crate::handlers::library::get_issue_by_id),
            )
            .route(
                "/issues/{issue_id}/return",
                apistos::web::post().to(crate::handlers::library::return_book),
            )
            .route(
                "/issues/student/{student_id}",
                apistos::web::get().to(crate::handlers::library::get_issued_books_by_student),
            )
            .route(
                "/issues/staff/{staff_id}",
                apistos::web::get().to(crate::handlers::library::get_issued_books_by_staff),
            )
            .route(
                "/issues/overdue",
                apistos::web::get().to(crate::handlers::library::get_overdue_books),
            )
            // Fine routes
            .route(
                "/fines/{issue_id}/pay",
                apistos::web::post().to(crate::handlers::library::pay_fine),
            )
            .route(
                "/fines/{issue_id}/waive",
                apistos::web::post().to(crate::handlers::library::waive_fine),
            )
            .route(
                "/fines/history",
                apistos::web::get().to(crate::handlers::library::get_fine_history),
            )
            // Settings routes
            .route(
                "/settings",
                apistos::web::get().to(crate::handlers::library::get_library_settings),
            )
            .route(
                "/settings",
                apistos::web::put().to(crate::handlers::library::update_library_settings),
            )
            // Statistics routes
            .route(
                "/stats",
                apistos::web::get().to(crate::handlers::library::get_library_stats),
            )
    );

    cfg.configure(crate::handlers::property::config);
    cfg.configure(crate::handlers::financial::config);

    cfg.route("/", apistos::web::get().to(hello));
    cfg.route("/error", apistos::web::get().to(hello_error));
}
