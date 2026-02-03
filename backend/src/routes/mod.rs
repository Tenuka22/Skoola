use crate::{
    database::tables::RoleEnum,
    handlers::{
        auth::{login, logout, refresh, register, request_password_reset, reset_password},
        hello::{hello, hello_error},
        oauth::{github_callback, google_callback},
        permissions::{
            create_permission, delete_permission, get_permission, get_permissions,
            update_permission,
        },
        profile::{
            change_email, change_password, get_profile, link_github, link_google, update_profile,
        },
        role_permissions::{assign_permission_to_role, unassign_permission_from_role},
        roles::{create_role, delete_role, get_role, get_roles, update_role},
        staff::{create_staff, delete_staff, get_all_staff, get_staff_by_id, update_staff, upload_staff_photo},
        staff_roles::{assign_role_to_staff, get_staff_roles, remove_role_from_staff, update_staff_roles},
        teacher_assignments::{assign_class_to_teacher, assign_subject_to_teacher, get_teacher_workload},
        verification::{resend_verification_email, verify_email},
        staff_attendance::{mark_staff_attendance_daily, mark_bulk_staff_attendance, update_staff_attendance, get_staff_attendance_by_date, get_staff_attendance_by_staff_member, calculate_monthly_attendance_percentage},
        staff_leaves::{apply_for_leave, approve_reject_leave},
    },
    utils::{jwt::Authenticated, roles::RoleVerification},
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
            .route("/verify-email/{token}", web::get().to(verify_email))
            .route("/resend-verification", web::post().to(resend_verification_email)),
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
        web::scope("/roles")
            .wrap(RoleVerification {
                required_role: RoleEnum::FullAdmin,
            })
            .wrap(Authenticated)
            .route("", web::get().to(get_roles))
            .route("", web::post().to(create_role))
            .route("/{role_id}", web::get().to(get_role))
            .route("/{role_id}", web::put().to(update_role))
            .route("/{role_id}", web::delete().to(delete_role))
            .route(
                "/{role_id}/permissions/{permission_id}",
                web::post().to(assign_permission_to_role),
            )
            .route(
                "/{role_id}/permissions/{permission_id}",
                web::delete().to(unassign_permission_from_role),
            ),
    )
    .service(
        web::scope("/permissions")
            .wrap(RoleVerification {
                required_role: RoleEnum::FullAdmin,
            })
            .wrap(Authenticated)
            .route("", web::get().to(get_permissions))
            .route("", web::post().to(create_permission))
            .route("/{permission_id}", web::get().to(get_permission))
            .route("/{permission_id}", web::put().to(update_permission))
            .route("/{permission_id}", web::delete().to(delete_permission)),
    )
    .service(
        web::scope("/staff")
            .wrap(RoleVerification {
                required_role: RoleEnum::Admin,
            })
            .wrap(Authenticated)
            .route("", web::get().to(get_all_staff))
            .route("/{staff_id}", web::get().to(get_staff_by_id))
            .route("", web::post().to(create_staff))
            .route("/{staff_id}", web::put().to(update_staff))
            .route("/{staff_id}", web::delete().to(delete_staff))
            .route("/{staff_id}/photo", web::post().to(upload_staff_photo))
            .route(
                "/{staff_id}/roles",
                web::post().to(assign_role_to_staff),
            )
            //.route(
            //    "/{staff_id}/roles",
            //    apistos::web::put().to(update_staff_roles),
            //)
            .route(
                "/{staff_id}/roles/{role_id}",
                web::delete().to(remove_role_from_staff),
            )
            .route(
                "/{staff_id}/roles",
                web::get().to(get_staff_roles),
            )
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
            .route(
                "/{staff_id}/leaves",
                web::post().to(apply_for_leave),
            )
            .route(
                "/leaves/{leave_id}/status",
                web::put().to(approve_reject_leave),
            ),
    );
    cfg.route("/", web::get().to(hello));
    cfg.route("/error", web::get().to(hello_error));
}
