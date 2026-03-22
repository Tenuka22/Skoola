use crate::database::enums::PermissionEnum;
use crate::handlers::staff::{assignments, contracts, details, events, rewards, staff, staff_attendance, staff_leaves};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/staff")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManage,
            })
            .wrap(Authenticated)
            .route("", web::get().to(staff::get_all_staff))
            .route("/{id}", web::get().to(staff::get_staff_by_id))
            .route("/{id}", web::delete().to(staff::delete_staff))
            .route("/bulk", web::delete().to(staff::bulk_delete_staff)),
    )
    .service(
        web::scope("/staff-contracts")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StaffManage })
            .wrap(Authenticated)
            .route("", web::post().to(contracts::create_staff_contract))
            .route("/{id}", web::get().to(contracts::get_staff_contract_by_id))
            .route("", web::get().to(contracts::get_all_staff_contract))
            .route("/{id}", web::put().to(contracts::update_staff_contract))
            .route("/{id}", web::delete().to(contracts::delete_staff_contract))
            .route("/bulk", web::delete().to(contracts::bulk_delete_staff_contract)),
    )
    .service(
        web::scope("/staff-events")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StaffManage })
            .wrap(Authenticated)
            .route("", web::post().to(events::create_staff_event))
            .route("/{id}", web::get().to(events::get_staff_event_by_id))
            .route("", web::get().to(events::get_all_staff_event))
            .route("/{id}", web::put().to(events::update_staff_event))
            .route("/{id}", web::delete().to(events::delete_staff_event))
            .route("/bulk", web::delete().to(events::bulk_delete_staff_event)),
    )
    .service(
        web::scope("/staff-leaves")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StaffManageLeaves })
            .wrap(Authenticated)
            .route("/apply/{staff_id}", web::post().to(staff_leaves::apply_for_leave))
            .route("/review/{leave_id}", web::post().to(staff_leaves::approve_reject_leave))
            .route("/balance/{staff_id}", web::get().to(staff_leaves::view_leave_balance)),
    )
    .service(
        web::scope("/staff-contacts")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StaffManage })
            .wrap(Authenticated)
            .route("", web::post().to(staff::create_staff_contact))
            .route("/{id}", web::get().to(staff::get_staff_contact_by_id))
            .route("", web::get().to(staff::get_all_staff_contact))
            .route("/{id}", web::put().to(staff::update_staff_contact))
            .route("/{id}", web::delete().to(staff::delete_staff_contact))
            .route("/bulk", web::delete().to(staff::bulk_delete_staff_contact)),
    )
    .service(
        web::scope("/staff-media")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StaffManage })
            .wrap(Authenticated)
            .route("", web::post().to(staff::create_staff_media))
            .route("/{id}", web::get().to(staff::get_staff_media_by_id))
            .route("", web::get().to(staff::get_all_staff_media))
            .route("/{id}", web::put().to(staff::update_staff_media))
            .route("/{id}", web::delete().to(staff::delete_staff_media))
            .route("/bulk", web::delete().to(staff::bulk_delete_staff_media)),
    )
    .service(
        web::scope("/staff-reward-snapshots")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StaffManage })
            .wrap(Authenticated)
            .route("", web::post().to(staff::create_staff_reward_snapshot))
            .route("/{id}", web::get().to(staff::get_staff_reward_snapshot_by_id))
            .route("", web::get().to(staff::get_all_staff_reward_snapshot))
            .route("/{id}", web::put().to(staff::update_staff_reward_snapshot))
            .route("/{id}", web::delete().to(staff::delete_staff_reward_snapshot))
            .route("/bulk", web::delete().to(staff::bulk_delete_staff_reward_snapshot)),
    )
    .service(
        web::scope("/staff-employment-status")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StaffManage })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_staff_employment_status))
            .route("/{id}", web::get().to(details::get_staff_employment_status_by_id))
            .route("", web::get().to(details::get_all_staff_employment_status))
            .route("/{id}", web::put().to(details::update_staff_employment_status))
            .route("/{id}", web::delete().to(details::delete_staff_employment_status))
            .route("/bulk", web::delete().to(details::bulk_delete_staff_employment_status)),
    )
    .service(
        web::scope("/staff-identity")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StaffManage })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_staff_identity))
            .route("/{id}", web::get().to(details::get_staff_identity_by_id))
            .route("", web::get().to(details::get_all_staff_identity))
            .route("/{id}", web::put().to(details::update_staff_identity))
            .route("/{id}", web::delete().to(details::delete_staff_identity))
            .route("/bulk", web::delete().to(details::bulk_delete_staff_identity)),
    )
    .service(
        web::scope("/staff-departments")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_staff_department))
            .route("/{id}", web::get().to(details::get_staff_department_by_id))
            .route("", web::get().to(details::get_all_staff_department))
            .route("/{id}", web::put().to(details::update_staff_department))
            .route("/{id}", web::delete().to(details::delete_staff_department)),
    )
    .service(
        web::scope("/staff-qualifications")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_staff_qualification))
            .route("/{id}", web::get().to(details::get_staff_qualification_by_id))
            .route("", web::get().to(details::get_all_staff_qualification))
            .route("/{id}", web::put().to(details::update_staff_qualification))
            .route("/{id}", web::delete().to(details::delete_staff_qualification)),
    )
    .service(
        web::scope("/staff-employment-history")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_staff_employment_history))
            .route("/{id}", web::get().to(details::get_staff_employment_history_by_id))
            .route("", web::get().to(details::get_all_staff_employment_history))
            .route("/{id}", web::delete().to(details::delete_staff_employment_history)),
    )
    .service(
        web::scope("/teacher-teaching-history")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_teacher_teaching_history))
            .route("/{id}", web::get().to(details::get_teacher_teaching_history_by_id))
            .route("", web::get().to(details::get_all_teacher_teaching_history))
            .route("/{id}", web::delete().to(details::delete_teacher_teaching_history)),
    )
    .service(
        web::scope("/staff-cvs")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_staff_cv))
            .route("/{id}", web::get().to(details::get_staff_cv_by_id))
            .route("", web::get().to(details::get_all_staff_cv))
            .route("/{id}", web::delete().to(details::delete_staff_cv)),
    )
    .service(
        web::scope("/staff-documents")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_staff_document))
            .route("/{id}", web::get().to(details::get_staff_document_by_id))
            .route("", web::get().to(details::get_all_staff_document))
            .route("/{id}", web::delete().to(details::delete_staff_document)),
    )
    .service(
        web::scope("/staff-notes")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_staff_note))
            .route("/{id}", web::get().to(details::get_staff_note_by_id))
            .route("", web::get().to(details::get_all_staff_note))
            .route("/{id}", web::delete().to(details::delete_staff_note)),
    )
    .service(
        web::scope("/staff-overtime")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_staff_overtime))
            .route("/{id}", web::get().to(details::get_staff_overtime_by_id))
            .route("", web::get().to(details::get_all_staff_overtime))
            .route("/{id}", web::delete().to(details::delete_staff_overtime)),
    )
    .service(
        web::scope("/staff-skills")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(details::create_staff_skill))
            .route("/{id}", web::get().to(details::get_staff_skill_by_id))
            .route("", web::get().to(details::get_all_staff_skill))
            .route("/{id}", web::delete().to(details::delete_staff_skill)),
    )
    .service(
        web::scope("/teacher-class-assignments")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(assignments::create_teacher_class_assignment))
            .route("/{id}", web::get().to(assignments::get_teacher_class_assignment_by_id))
            .route("", web::get().to(assignments::get_all_teacher_class_assignment))
            .route("/{id}", web::delete().to(assignments::delete_teacher_class_assignment)),
    )
    .service(
        web::scope("/teacher-subject-assignments")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(assignments::create_teacher_subject_assignment))
            .route("/{id}", web::get().to(assignments::get_teacher_subject_assignment_by_id))
            .route("", web::get().to(assignments::get_all_teacher_subject_assignment))
            .route("/{id}", web::delete().to(assignments::delete_teacher_subject_assignment)),
    )
    .service(
        web::scope("/teacher-period-attendance-records")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManageAttendance,
            })
            .wrap(Authenticated)
            .route("", web::post().to(assignments::create_teacher_period_attendance))
            .route("/{id}", web::get().to(assignments::get_teacher_period_attendance_by_id))
            .route("", web::get().to(assignments::get_all_teacher_period_attendance))
            .route("/{id}", web::delete().to(assignments::delete_teacher_period_attendance)),
    )
    .service(
        web::scope("/substitutions")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManageAttendance,
            })
            .wrap(Authenticated)
            .route("", web::post().to(assignments::create_substitution))
            .route("/{id}", web::get().to(assignments::get_substitution_by_id))
            .route("", web::get().to(assignments::get_all_substitution))
            .route("/{id}", web::delete().to(assignments::delete_substitution)),
    )
    .service(
        web::scope("/staff-attendance")

            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManageAttendance,
            })
            .wrap(Authenticated)
            .route("/mark-period", web::post().to(staff_attendance::mark_teacher_period_attendance))
            .route("/daily/{staff_id}", web::post().to(staff_attendance::mark_staff_attendance_daily))
            .route("/bulk", web::post().to(staff_attendance::mark_bulk_staff_attendance))
            .route("/{attendance_id}", web::put().to(staff_attendance::update_staff_attendance))
            .route("/sync-leaves/{date}", web::post().to(staff_attendance::sync_leaves))
            .route("/date", web::get().to(staff_attendance::get_staff_attendance_by_date))
            .route("/staff/{staff_id}", web::get().to(staff_attendance::get_staff_attendance_by_staff_member))
            .route("/my-substitutions", web::get().to(staff_attendance::get_my_substitutions))
            .route("/monthly-percentage/{staff_id}/{year}/{month}", web::get().to(staff_attendance::calculate_monthly_attendance_percentage))
            .route("/suggest-substitute", web::post().to(staff_attendance::suggest_substitute))
            .route("/create-substitution", web::post().to(staff_attendance::create_substitution))
            .route("/record-lesson-progress", web::post().to(staff_attendance::record_lesson_progress))
            .route("/lesson-progress/{class_id}/{subject_id}", web::get().to(staff_attendance::get_lesson_progress)),
    )
    .service(
        web::scope("/staff-leaves")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManageLeaves,
            })
            .wrap(Authenticated)
            .route("/apply/{staff_id}", web::post().to(staff_leaves::apply_for_leave))
            .route("/approve-reject/{leave_id}", web::post().to(staff_leaves::approve_reject_leave))
            .route("/balance/{staff_id}", web::get().to(staff_leaves::view_leave_balance)),
    )
    .service(
        web::scope("/staff-rewards")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManage,
            })
            .wrap(Authenticated)
            .route("/balance/{teacher_id}", web::get().to(rewards::get_teacher_reward_balance))
            .route("/history/{teacher_id}", web::get().to(rewards::get_teacher_reward_history)),
    );
}
