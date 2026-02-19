use apistos::web;
use crate::handlers::auth::{login, logout, oauth, permission_sets, profile, refresh, register, request_password_reset, reset_password, role_permissions, user_set_permissions, verification};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use crate::database::enums::PermissionEnum;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            .route("/refresh", web::post().to(refresh))
            .route("/password/request", web::post().to(request_password_reset))
            .route("/password/reset/{token}", web::post().to(reset_password))
            .route("/google/callback", web::get().to(oauth::google_callback))
            .route("/github/callback", web::get().to(oauth::github_callback))
            .route("/verify-email/{token}", web::get().to(verification::verify_email)),
    )
    .service(
        web::scope("/profile")
            .wrap(Authenticated)
            .route("", web::get().to(profile::get_profile))
            .route("", web::put().to(profile::update_profile))
            .route("/password", web::post().to(profile::change_password))
            .route("/email", web::post().to(profile::change_email))
            .route("/link/google", web::get().to(profile::link_google))
            .route("/link/github", web::get().to(profile::link_github)),
    )
    .service(
        web::scope("/user-sets")
            .wrap(Authenticated)
            .route("/", web::get().to(permission_sets::get_all_permission_sets))
            .route("/", web::post().to(permission_sets::create_permission_set))
            .route("/{permission_set_id}", web::put().to(permission_sets::update_permission_set))
            .route("/{permission_set_id}", web::delete().to(permission_sets::delete_permission_set))
            .route("/{permission_set_id}/users", web::get().to(permission_sets::get_user_set_members)),
    )
    .service(
        web::scope("/roles/{role_id}/permissions")
            .wrap(PermissionVerification { required_permission: PermissionEnum::RoleAssignPermissions })
            .wrap(Authenticated)
            .route("", web::get().to(role_permissions::get_role_permissions))
            .route("/{permission}", web::post().to(role_permissions::assign_permission_to_role))
            .route("/{permission}", web::delete().to(role_permissions::unassign_permission_from_role)),
    )
    .service(
        web::scope("/user-sets/{user_set_id}/permissions")
            .wrap(PermissionVerification { required_permission: PermissionEnum::PermissionSetManage })
            .wrap(Authenticated)
            .route("", web::get().to(user_set_permissions::get_user_set_permissions))
            .route("/{permission}", web::post().to(user_set_permissions::assign_permission_to_user_set))
            .route("/{permission}", web::delete().to(user_set_permissions::unassign_permission_from_user_set)),
    );
}
