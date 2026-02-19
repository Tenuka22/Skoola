use apistos::web;
use crate::handlers::students::{student, student_attendance, student_class_assignment, student_guardian, student_marks};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use crate::database::enums::PermissionEnum;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/students")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManage })
            .wrap(Authenticated)
            .route("", web::post().to(student::create_student))
            .route("/{student_id}", web::put().to(student::update_student))
            .route("/{student_id}", web::get().to(student::get_student_by_id))
            .route("", web::get().to(student::get_all_students))
            .route("/{student_id}", web::delete().to(student::delete_student))
            .route("/{student_id}/photo", web::post().to(student::upload_student_photo))
            .route("/{student_id}/current-class", web::get().to(student_class_assignment::get_current_class_of_student))
            .route("/{student_id}/class-history", web::get().to(student_class_assignment::get_class_history_of_student)),
    )
    .service(
        web::scope("/students/{student_id}/guardians")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManageGuardians })
            .wrap(Authenticated)
            .route("", web::post().to(student_guardian::add_guardian_to_student))
            .route("/{guardian_id}", web::put().to(student_guardian::update_guardian_information))
            .route("/{guardian_id}", web::delete().to(student_guardian::remove_guardian_from_student))
            .route("", web::get().to(student_guardian::get_all_guardians_for_student)),
    )
    .service(
        web::scope("/student-class-assignments")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManageEnrollment })
            .wrap(Authenticated)
            .route("", web::post().to(student_class_assignment::assign_student_to_class))
            .route("/{student_id}/{assignment_id}/transfer", web::put().to(student_class_assignment::transfer_student_class))
            .route("/bulk", web::post().to(student_class_assignment::bulk_assign_students_to_classes))
            .route("/promote", web::post().to(student_class_assignment::promote_student_to_next_grade)),
    )
    .service(
        web::scope("/student-attendance")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManageAttendance })
            .wrap(Authenticated)
            .route("/bulk", web::post().to(student_attendance::bulk_mark_student_attendance))
            .route("", web::post().to(student_attendance::mark_individual_student_attendance))
            .route("/{attendance_id}", web::put().to(student_attendance::update_student_attendance))
            .route("/class/{class_id}/date/{date}", web::get().to(student_attendance::get_attendance_by_class_and_date))
            .route("/student/{student_id}", web::get().to(student_attendance::get_attendance_by_student))
            .route("/student/{student_id}/percentage", web::get().to(student_attendance::calculate_student_attendance_percentage))
            .route("/report", web::get().to(student_attendance::generate_attendance_report))
            .route("/low-attendance", web::get().to(student_attendance::get_students_with_low_attendance))
            .route("/notifications/absent", web::post().to(student_attendance::send_absence_notifications))
            .route("/emergency/initiate", web::post().to(student_attendance::initiate_emergency_roll_call))
            .route("/emergency/{roll_call_id}/{user_id}", web::put().to(student_attendance::update_emergency_status))
            .route("/emergency/{roll_call_id}/complete", web::post().to(student_attendance::complete_emergency_roll_call))
            .route("/sync/pre-approved/{date}", web::post().to(student_attendance::sync_pre_approved_absences))
            .route("/sync/school-business/{date}", web::post().to(student_attendance::sync_school_business))
            .route("/check-discrepancies/{date}", web::get().to(student_attendance::run_discrepancy_check))
            .route("/enriched-list/{class_id}/{date}", web::get().to(student_attendance::get_enriched_student_list))
            .route("/period", web::post().to(student_attendance::mark_period_attendance))
            .route("/exit-pass", web::post().to(student_attendance::issue_exit_pass))
            .route("/{student_id}/evaluate-policies", web::post().to(student_attendance::evaluate_policies))
            .route("/excuses", web::post().to(student_attendance::submit_excuse))
            .route("/excuses/{excuse_id}/verify", web::post().to(student_attendance::verify_excuse)),
    )
    .service(
        web::scope("/student-marks")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManageMarks })
            .wrap(Authenticated)
            .route("", web::post().to(student_marks::create_student_mark))
            .route("", web::get().to(student_marks::get_all_student_marks))
            .route("/bulk", web::post().to(student_marks::bulk_create_student_marks))
            .route("/{id}", web::get().to(student_marks::get_student_mark_by_id))
            .route("/student/{student_id}", web::get().to(student_marks::get_student_marks_by_student_id))
            .route("/exam/{exam_id}/class/{class_id}", web::get().to(student_marks::get_student_marks_by_exam_and_class))
            .route("/{id}", web::put().to(student_marks::update_student_mark))
            .route("/{id}", web::delete().to(student_marks::delete_student_mark)),
    );
}
