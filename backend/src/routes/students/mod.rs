use crate::database::enums::PermissionEnum;
use crate::handlers::students::{student, student_attendance, student_class_assignment, student_guardian, details};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/student-contacts")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManage })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_contact))
            .route("/{id}", web::get().to(details::get_student_contact_by_id))
            .route("", web::get().to(details::get_all_student_contact))
            .route("/{id}", web::put().to(details::update_student_contact))
            .route("/{id}", web::delete().to(details::delete_student_contact))
            .route("/bulk", web::delete().to(details::bulk_delete_student_contact)),
    )
    .service(
        web::scope("/student-media")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManage })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_media))
            .route("/{id}", web::get().to(details::get_student_media_by_id))
            .route("", web::get().to(details::get_all_student_media))
            .route("/{id}", web::put().to(details::update_student_media))
            .route("/{id}", web::delete().to(details::delete_student_media))
            .route("/bulk", web::delete().to(details::bulk_delete_student_media)),
    )
    .service(
        web::scope("/students")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StudentManage,
            })
            .wrap(Authenticated)
            .route("", web::get().to(student::get_all_student))
            .route("/{id}", web::get().to(student::get_student_by_id))
            .route("/{id}", web::put().to(student::update_student))
            .route("/{id}", web::delete().to(student::delete_student))
            .route("/bulk", web::delete().to(student::bulk_delete_student))
            .route("/bulk", web::patch().to(student::bulk_update_student)),
    )
    .service(
        web::scope("/student-allergies")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManage })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_allergy))
            .route("/{id}", web::get().to(details::get_student_allergy_by_id))
            .route("", web::get().to(details::get_all_student_allergy))
            .route("/{id}", web::put().to(details::update_student_allergy))
            .route("/{id}", web::delete().to(details::delete_student_allergy)),
    )
    .service(
        web::scope("/student-birth-certificates")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManage })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_birth_certificate))
            .route("/{id}", web::get().to(details::get_student_birth_certificate_by_id))
            .route("", web::get().to(details::get_all_student_birth_certificate))
            .route("/{id}", web::put().to(details::update_student_birth_certificate))
            .route("/{id}", web::delete().to(details::delete_student_birth_certificate)),
    )
    .service(
        web::scope("/student-emergency-contacts")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManage })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_emergency_contact))
            .route("/{id}", web::get().to(details::get_student_emergency_contact_by_id))
            .route("", web::get().to(details::get_all_student_emergency_contact))
            .route("/{id}", web::put().to(details::update_student_emergency_contact))
            .route("/{id}", web::delete().to(details::delete_student_emergency_contact)),
    )
    .service(
        web::scope("/student-fees")
            .wrap(PermissionVerification { required_permission: PermissionEnum::FinanceRead })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_fee))
            .route("/{id}", web::get().to(details::get_student_fee_by_id))
            .route("", web::get().to(details::get_all_student_fee))
            .route("/{id}", web::put().to(details::update_student_fee))
            .route("/{id}", web::delete().to(details::delete_student_fee)),
    )
    .service(
        web::scope("/student-mark-entries")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManageMarks })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_mark_entry))
            .route("/{id}", web::get().to(details::get_student_mark_entry_by_id))
            .route("", web::get().to(details::get_all_student_mark_entry))
            .route("/{id}", web::put().to(details::update_student_mark_entry))
            .route("/{id}", web::delete().to(details::delete_student_mark_entry)),
    )
    .service(
        web::scope("/student-medical-conditions")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManage })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_medical_condition))
            .route("/{id}", web::get().to(details::get_student_medical_condition_by_id))
            .route("", web::get().to(details::get_all_student_medical_condition))
            .route("/{id}", web::put().to(details::update_student_medical_condition))
            .route("/{id}", web::delete().to(details::delete_student_medical_condition)),
    )
    .service(
        web::scope("/student-medications")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManage })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_medication))
            .route("/{id}", web::get().to(details::get_student_medication_by_id))
            .route("", web::get().to(details::get_all_student_medication))
            .route("/{id}", web::put().to(details::update_student_medication))
            .route("/{id}", web::delete().to(details::delete_student_medication)),
    )
    .service(
        web::scope("/student-missed-lessons")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManageAttendance })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_missed_lesson))
            .route("/{id}", web::get().to(details::get_student_missed_lesson_by_id))
            .route("", web::get().to(details::get_all_student_missed_lesson))
            .route("/{id}", web::put().to(details::update_student_missed_lesson))
            .route("/{id}", web::delete().to(details::delete_student_missed_lesson)),
    )
    .service(
        web::scope("/student-nics")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManage })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_nic))
            .route("/{id}", web::get().to(details::get_student_nic_by_id))
            .route("", web::get().to(details::get_all_student_nic))
            .route("/{id}", web::put().to(details::update_student_nic))
            .route("/{id}", web::delete().to(details::delete_student_nic)),
    )
    .service(
        web::scope("/student-status")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManage })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_status_record))
            .route("/{id}", web::get().to(details::get_student_status_record_by_id))
            .route("", web::get().to(details::get_all_student_status_record))
            .route("/{id}", web::put().to(details::update_student_status_record))
            .route("/{id}", web::delete().to(details::delete_student_status_record)),
    )
    .service(
        web::scope("/student-period-attendance-records")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StudentManageAttendance })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_period_attendance))
            .route("/{id}", web::get().to(details::get_student_period_attendance_by_id))
            .route("", web::get().to(details::get_all_student_period_attendance))
            .route("/{id}", web::put().to(details::update_student_period_attendance))
            .route("/{id}", web::delete().to(details::delete_student_period_attendance)),
    )
    .service(
        web::scope("/student-medical-info")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StudentManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_medical_info))
            .route("/{id}", web::get().to(details::get_student_medical_info_by_id))
            .route("", web::get().to(details::get_all_student_medical_info))
            .route("/{id}", web::put().to(details::update_student_medical_info))
            .route("/{id}", web::delete().to(details::delete_student_medical_info)),
    )
    .service(
        web::scope("/student-previous-schools")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StudentManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_previous_school))
            .route("/{id}", web::get().to(details::get_student_previous_school_by_id))
            .route("", web::get().to(details::get_all_student_previous_school))
            .route("/{id}", web::put().to(details::update_student_previous_school))
            .route("/{id}", web::delete().to(details::delete_student_previous_school)),
    )
    .service(
        web::scope("/student-class-assignments")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StudentManageEnrollment,
            })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_student_class_assignment))
            .route("/{id}", web::get().to(details::get_student_class_assignment_by_id))
            .route("", web::get().to(details::get_all_student_class_assignment))
            .route("/{id}", web::put().to(details::update_student_class_assignment))
            .route("/{id}", web::delete().to(details::delete_student_class_assignment)),
    )
    .service(
        web::scope("/student-attendance")

            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StudentManageAttendance,
            })
            .wrap(Authenticated)
            .route("/bulk-mark", web::post().to(student_attendance::bulk_mark_student_attendance))
            .route("/mark", web::post().to(student_attendance::mark_individual_student_attendance))
            .route("/{id}", web::put().to(student_attendance::update_student_attendance))
            .route("/class/{class_id}/date/{date}", web::get().to(student_attendance::get_attendance_by_class_and_date))
            .route("/student/{student_id}", web::get().to(student_attendance::get_attendance_by_student))
            .route("/percentage/{id}", web::get().to(student_attendance::calculate_student_attendance_percentage))
            .route("/report", web::get().to(student_attendance::generate_attendance_report))
            .route("/low-attendance", web::get().to(student_attendance::get_students_with_low_attendance))
            .route("/notify-absence", web::post().to(student_attendance::send_absence_notifications))
            .route("/emergency-roll-call", web::post().to(student_attendance::initiate_emergency_roll_call))
            .route("/emergency-status/{roll_call_id}/{user_id}", web::put().to(student_attendance::update_emergency_status))
            .route("/complete-emergency/{roll_call_id}", web::post().to(student_attendance::complete_emergency_roll_call))
            .route("/sync-pre-approved/{date}", web::post().to(student_attendance::sync_pre_approved_absences))
            .route("/sync-school-business/{date}", web::post().to(student_attendance::sync_school_business))
            .route("/run-discrepancy-check/{date}", web::post().to(student_attendance::run_discrepancy_check))
            .route("/enriched-list/{class_id}/{date}", web::get().to(student_attendance::get_enriched_student_list))
            .route("/mark-period", web::post().to(student_attendance::mark_period_attendance))
            .route("/issue-exit-pass", web::post().to(student_attendance::issue_exit_pass))
            .route("/evaluate-policies/{id}", web::post().to(student_attendance::evaluate_policies))
            .route("/submit-excuse", web::post().to(student_attendance::submit_excuse))
            .route("/verify-excuse/{id}", web::post().to(student_attendance::verify_excuse)),
    )
    .service(
        web::scope("/student-enrollment")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StudentManageEnrollment,
            })
            .wrap(Authenticated)
            .route("/assign", web::post().to(student_class_assignment::assign_student_to_class))
            .route("/transfer/{student_id}/{old_assignment_id}", web::post().to(student_class_assignment::transfer_student_class))
            .route("/bulk-assign", web::post().to(student_class_assignment::bulk_assign_students_to_classes))
            .route("/promote", web::post().to(student_class_assignment::promote_student_to_next_grade)),
    )
    .service(
        web::scope("/student-guardians")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StudentManageGuardians,
            })
            .wrap(Authenticated)
            .route("/{student_id}", web::post().to(student_guardian::add_guardian_to_student))
            .route("/{student_id}/{guardian_id}", web::put().to(student_guardian::update_guardian_information))
            .route("/{student_id}/{guardian_id}", web::delete().to(student_guardian::remove_guardian_from_student))
            .route("/{student_id}", web::get().to(student_guardian::get_all_guardians_for_student)),
    );
}
