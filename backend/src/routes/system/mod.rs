use crate::database::enums::PermissionEnum;
use crate::handlers::system::{activities, attendance, audit, activity_attendance, audit_logs, calendar, seed, emergency, file, school_settings};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(|cfg_local| school_settings::config(cfg_local));
    cfg.configure(|cfg_local| file::configure(cfg_local));

    cfg.service(
        web::scope("/activity-attendance")
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .wrap(Authenticated)
            .route("", web::post().to(activity_attendance::create_activity_attendance))
            .route("/{id}", web::get().to(activity_attendance::get_activity_attendance_by_id))
            .route("", web::get().to(activity_attendance::get_all_activity_attendance))
            .route("/{id}", web::put().to(activity_attendance::update_activity_attendance))
            .route("/{id}", web::delete().to(activity_attendance::delete_activity_attendance))
            .route("/bulk", web::delete().to(activity_attendance::bulk_delete_activity_attendance))
            .route("/bulk", web::patch().to(activity_attendance::bulk_update_activity_attendance)),
    )
    .service(
        web::scope("/attendance-audit-logs")
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .wrap(Authenticated)
            .route("", web::post().to(audit_logs::create_attendance_audit_log))
            .route("/{id}", web::get().to(audit_logs::get_attendance_audit_log_by_id))
            .route("", web::get().to(audit_logs::get_all_attendance_audit_log))
            .route("/{id}", web::put().to(audit_logs::update_attendance_audit_log))
            .route("/{id}", web::delete().to(audit_logs::delete_attendance_audit_log))
            .route("/bulk", web::delete().to(audit_logs::bulk_delete_attendance_audit_log))
            .route("/bulk", web::patch().to(audit_logs::bulk_update_attendance_audit_log)),
    )
    .service(
        web::scope("/school-calendar")
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .wrap(Authenticated)
            .route("", web::post().to(calendar::create_school_calendar))
            .route("/{id}", web::get().to(calendar::get_school_calendar_by_id))
            .route("", web::get().to(calendar::get_all_school_calendar))
            .route("/{id}", web::put().to(calendar::update_school_calendar))
            .route("/{id}", web::delete().to(calendar::delete_school_calendar))
            .route("/bulk", web::delete().to(calendar::bulk_delete_school_calendar))
            .route("/bulk", web::patch().to(calendar::bulk_update_school_calendar)),
    )
    .service(
        web::scope("/seeds")
            .wrap(PermissionVerification { required_permission: PermissionEnum::SystemAdmin })
            .wrap(Authenticated)
            .route("", web::post().to(seed::create_seed))
            .route("/{id}", web::get().to(seed::get_seed_by_id))
            .route("", web::get().to(seed::get_all_seed))
            .route("/{id}", web::put().to(seed::update_seed))
            .route("/{id}", web::delete().to(seed::delete_seed))
            .route("/bulk", web::delete().to(seed::bulk_delete_seed))
            .route("/bulk", web::patch().to(seed::bulk_update_seed)),
    )
    .service(
        web::scope("/attendance-policies")
            .wrap(PermissionVerification { required_permission: PermissionEnum::AttendancePolicyManage })
            .wrap(Authenticated)
            .route("", web::post().to(attendance::create_attendance_policy))
            .route("/{id}", web::get().to(attendance::get_attendance_policy_by_id))
            .route("", web::get().to(attendance::get_all_attendance_policy))
            .route("/{id}", web::put().to(attendance::update_attendance_policy))
            .route("/{id}", web::delete().to(attendance::delete_attendance_policy))
            .route("/bulk", web::delete().to(attendance::bulk_delete_attendance_policy))
            .route("/bulk", web::patch().to(attendance::bulk_update_attendance_policy)),
    );

    cfg.service(
        web::scope("/attendance-excuses")
            .wrap(PermissionVerification { required_permission: PermissionEnum::AttendanceExcuseManage })
            .wrap(Authenticated)
            .route("", web::post().to(attendance::create_attendance_excuse))
            .route("/{id}", web::get().to(attendance::get_attendance_excuse_by_id))
            .route("", web::get().to(attendance::get_all_attendance_excuse))
            .route("/{id}", web::put().to(attendance::update_attendance_excuse))
            .route("/{id}", web::delete().to(attendance::delete_attendance_excuse))
            .route("/bulk", web::delete().to(attendance::bulk_delete_attendance_excuse))
            .route("/bulk", web::patch().to(attendance::bulk_update_attendance_excuse)),
    )
    .service(
        web::scope("/attendance-discrepancies")
            .wrap(PermissionVerification { required_permission: PermissionEnum::AttendanceDiscrepancyManage })
            .wrap(Authenticated)
            .route("", web::post().to(attendance::create_attendance_discrepancy))
            .route("/{id}", web::get().to(attendance::get_attendance_discrepancy_by_id))
            .route("", web::get().to(attendance::get_all_attendance_discrepancy))
            .route("/{id}", web::put().to(attendance::update_attendance_discrepancy))
            .route("/{id}", web::delete().to(attendance::delete_attendance_discrepancy))
            .route("/bulk", web::delete().to(attendance::bulk_delete_attendance_discrepancy))
            .route("/bulk", web::patch().to(attendance::bulk_update_attendance_discrepancy)),
    )
    .service(
        web::scope("/emergency-roll-calls")
            .wrap(PermissionVerification { required_permission: PermissionEnum::EmergencyRollCallManage })
            .wrap(Authenticated)
            .route("", web::post().to(emergency::create_emergency_roll_call))
            .route("/{id}", web::get().to(emergency::get_emergency_roll_call_by_id))
            .route("", web::get().to(emergency::get_all_emergency_roll_call))
            .route("/{id}", web::put().to(emergency::update_emergency_roll_call))
            .route("/{id}", web::delete().to(emergency::delete_emergency_roll_call))
            .route("/bulk", web::delete().to(emergency::bulk_delete_emergency_roll_call))
            .route("/bulk", web::patch().to(emergency::bulk_update_emergency_roll_call)),
    )
    .service(
        web::scope("/audit-logs")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::SystemAdmin,
            })
            .wrap(Authenticated)
            .route("", web::get().to(audit::get_all_audit_log))
            .route("/{id}", web::get().to(audit::get_audit_log_by_id))
            .route(
                "/record/{table_name}/{record_pk}",
                web::get().to(audit::get_record_audit_logs),
            ),
    )
    .configure(|c| activities::config(c));
}
