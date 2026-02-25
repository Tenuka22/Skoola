use apistos::web;
use crate::handlers::staff::{staff, staff_attendance, staff_leaves};
use crate::handlers::auth::permission_sets;
use crate::handlers::academic::teacher_assignments;
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use crate::database::enums::PermissionEnum;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/staff")
            .wrap(PermissionVerification { required_permission: PermissionEnum::StaffManage })
            .wrap(Authenticated)
            .route("", web::get().to(staff::get_all_staff))
            .route("/{staff_id}", web::get().to(staff::get_staff_by_id))
            .route("", web::post().to(staff::create_staff))
            .route("/{staff_id}", web::put().to(staff::update_staff))
            .route("/{staff_id}", web::delete().to(staff::delete_staff))
            .route("/bulk", web::delete().to(staff::bulk_delete_staff))
            .route("/bulk", web::patch().to(staff::bulk_update_staff))
            .route("/{staff_id}/photo", web::post().to(staff::upload_staff_photo))
            .route("/{teacher_id}/classes", web::post().to(teacher_assignments::assign_class_to_teacher))
            .route("/{teacher_id}/subjects", web::post().to(teacher_assignments::assign_subject_to_teacher))
            .route("/{teacher_id}/workload", web::get().to(teacher_assignments::get_teacher_workload))
            .route("/{staff_id}/attendance", web::post().to(staff_attendance::mark_staff_attendance_daily))
            .route("/attendance/bulk", web::post().to(staff_attendance::mark_bulk_staff_attendance))
            .route("/attendance/{attendance_id}", web::put().to(staff_attendance::update_staff_attendance))
            .route("/attendance/date/{date}", web::get().to(staff_attendance::get_staff_attendance_by_date))
            .route("/{staff_id}/attendance/member", web::get().to(staff_attendance::get_staff_attendance_by_staff_member))
            .route("/{staff_id}/attendance/percentage/{year}/{month}", web::get().to(staff_attendance::calculate_monthly_attendance_percentage))
            .route("/attendance/sync-leaves/{date}", web::post().to(staff_attendance::sync_leaves))
            .route("/substitute/suggest", web::post().to(staff_attendance::suggest_substitute))
            .route("/substitute/create", web::post().to(staff_attendance::create_substitution))
            .route("/substitute/my", web::get().to(staff_attendance::get_my_substitutions))
            .route("/lesson-progress", web::post().to(staff_attendance::record_lesson_progress))
            .route("/lesson-progress/{class_id}/{subject_id}", web::get().to(staff_attendance::get_lesson_progress))
            .route("/{staff_id}/leaves", web::post().to(staff_leaves::apply_for_leave))
            .route("/leaves/{leave_id}/status", web::put().to(staff_leaves::approve_reject_leave))
            .route("/{staff_id}/leaves/balance", web::get().to(staff_leaves::view_leave_balance))
            .route("/{staff_id}/permission-sets", web::get().to(permission_sets::get_staff_permission_sets))
            .route("/{staff_id}/permission-sets/{set_id}", web::post().to(permission_sets::assign_permission_set_to_staff))
            .route("/{staff_id}/permission-sets/{set_id}", web::delete().to(permission_sets::unassign_permission_set_from_staff)),
    );
}
