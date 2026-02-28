use crate::database::enums::PermissionEnum;
use crate::handlers::auth::{
    login, logout, oauth, permission_sets, profile, refresh, register, request_password_reset,
    reset_password, role_permissions, role_sets, user_set_permissions, verification,
};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
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
            .route("/google/callback", web::get().to(oauth::google_callback))
            .route("/github/callback", web::get().to(oauth::github_callback))
            .route("/verify-email/{token}", web::get().to(verification::verify_email))
            .route("/resend-verification-email", web::post().to(verification::resend_verification_email))
            .service(
                web::scope("/users")
                    .wrap(Authenticated)
                    .route("", web::get().to(crate::handlers::auth::users::get_all_users))
                    .route("/stats", web::get().to(crate::handlers::auth::users::get_user_stats))
                    .route("/{user_id}", web::delete().to(crate::handlers::auth::users::delete_user))
                    .route("/{user_id}", web::put().to(crate::handlers::auth::users::update_user))
                    .route("/bulk", web::patch().to(crate::handlers::auth::users::bulk_update_users))
                    .service(
                        web::scope("/{user_id}/permissions")
                            .wrap(PermissionVerification { required_permission: PermissionEnum::UserManagePermissions })
                            .route("", web::get().to(crate::handlers::auth::user_permissions::get_user_permissions))
                            .route("", web::post().to(crate::handlers::auth::user_permissions::assign_permission_to_user))
                            .route("", web::delete().to(crate::handlers::auth::user_permissions::unassign_permission_from_user)),
                    )
                    .service(
                        web::scope("/bulk-delete")
                            .wrap(PermissionVerification { required_permission: PermissionEnum::UserDelete })
                            .route("", web::post().to(crate::handlers::auth::users::bulk_delete_users)),
                    ),
            ),
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
            .route("", web::post().to(role_permissions::assign_permission_to_role))
            .route("", web::delete().to(role_permissions::unassign_permission_from_role)),
    )
    .service(
        web::scope("/user-sets/{user_set_id}/permissions")
            .wrap(PermissionVerification { required_permission: PermissionEnum::PermissionSetManage })
            .wrap(Authenticated)
            .route("", web::get().to(user_set_permissions::get_user_set_permissions))
            .route("", web::post().to(user_set_permissions::assign_permission_to_user_set))
            .route("", web::delete().to(user_set_permissions::unassign_permission_from_user_set)),
    )
    .service(
        web::scope("/role-sets")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::RoleManage })
            .route("/", web::get().to(role_sets::get_all_role_sets))
            .route("/", web::post().to(role_sets::create_role_set))
            .route("/{role_set_id}", web::put().to(role_sets::update_role_set))
            .route("/{role_set_id}", web::delete().to(role_sets::delete_role_set))
            .route("/{role_set_id}/roles", web::get().to(role_sets::get_role_set_roles))
            .route("/{role_set_id}/roles", web::post().to(role_sets::assign_role_to_role_set))
            .route("/{role_set_id}/roles", web::delete().to(role_sets::unassign_role_from_role_set)),
    );
}
