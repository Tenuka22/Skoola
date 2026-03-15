use crate::database::enums::PermissionEnum;
use crate::handlers::auth::{
    login, logout, oauth, profile, refresh, register, request_password_reset,
    reset_password, users, role_sets, permission_sets, verification,
    user_permissions, role_permissions, user_set_permissions,
    admin_tokens, admin_sessions,
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
            .route("/resend-verification-email", web::post().to(verification::resend_verification_email)),
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
    );
}

pub fn configure_admin(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .wrap(PermissionVerification { required_permission: PermissionEnum::UserManage })
            .wrap(Authenticated)
            .route("/{id}", web::get().to(users::get_user_by_id))
            .route("", web::get().to(users::get_all_user))
            .route("/{id}", web::put().to(users::update_user))
            .route("/{id}", web::delete().to(users::delete_user))
            .route("/bulk", web::delete().to(users::bulk_delete_user))
            .route("/bulk", web::patch().to(users::bulk_update_user))
            .route("/{user_id}/permissions", web::post().to(user_permissions::assign_permission_to_user))
            .route("/{user_id}/permissions", web::delete().to(user_permissions::unassign_permission_from_user))
            .route("/{user_id}/permissions", web::get().to(user_permissions::get_user_permissions)),
    )
    .service(
        web::scope("/auth-tokens")
            .wrap(PermissionVerification { required_permission: PermissionEnum::UserManage })
            .wrap(Authenticated)
            .route("", web::post().to(admin_tokens::create_auth_token))
            .route("/{id}", web::get().to(admin_tokens::get_auth_token_by_id))
            .route("", web::get().to(admin_tokens::get_all_auth_token))
            .route("/{id}", web::put().to(admin_tokens::update_auth_token))
            .route("/{id}", web::delete().to(admin_tokens::delete_auth_token))
            .route("/bulk", web::delete().to(admin_tokens::bulk_delete_auth_token))
            .route("/bulk", web::patch().to(admin_tokens::bulk_update_auth_token)),
    )
    .service(
        web::scope("/sessions")
            .wrap(PermissionVerification { required_permission: PermissionEnum::UserManage })
            .wrap(Authenticated)
            .route("", web::post().to(admin_sessions::create_session))
            .route("/{id}", web::get().to(admin_sessions::get_session_by_id))
            .route("", web::get().to(admin_sessions::get_all_session))
            .route("/{id}", web::put().to(admin_sessions::update_session))
            .route("/{id}", web::delete().to(admin_sessions::delete_session))
            .route("/bulk", web::delete().to(admin_sessions::bulk_delete_session))
            .route("/bulk", web::patch().to(admin_sessions::bulk_update_session)),
    )
    .service(
        web::scope("/role-sets")
            .wrap(PermissionVerification { required_permission: PermissionEnum::RoleManage })
            .wrap(Authenticated)
            .route("", web::get().to(role_sets::get_all_role_set))
            .route("", web::post().to(role_sets::create_role_set))
            .route("/{id}", web::get().to(role_sets::get_role_set_by_id))
            .route("/{id}", web::put().to(role_sets::update_role_set))
            .route("/{id}", web::delete().to(role_sets::delete_role_set))
            .route("/bulk", web::delete().to(role_sets::bulk_delete_role_set))
            .route("/bulk", web::patch().to(role_sets::bulk_update_role_set))
            .route("/{role_id}/permissions", web::post().to(role_permissions::assign_permission_to_role))
            .route("/{role_id}/permissions", web::delete().to(role_permissions::unassign_permission_from_role))
            .route("/{role_id}/permissions", web::get().to(role_permissions::get_role_permissions)),
    )
    .service(
        web::scope("/user-sets")
            .wrap(PermissionVerification { required_permission: PermissionEnum::PermissionSetManage })
            .wrap(Authenticated)
            .route("", web::get().to(permission_sets::get_all_user_set))
            .route("", web::post().to(permission_sets::create_user_set))
            .route("/{id}", web::get().to(permission_sets::get_user_set_by_id))
            .route("/{id}", web::put().to(permission_sets::update_user_set))
            .route("/{id}", web::delete().to(permission_sets::delete_user_set))
            .route("/bulk", web::delete().to(permission_sets::bulk_delete_user_set))
            .route("/bulk", web::patch().to(permission_sets::bulk_update_user_set))
            .route("/{user_set_id}/permissions", web::post().to(user_set_permissions::assign_permission_to_user_set))
            .route("/{user_set_id}/permissions", web::delete().to(user_set_permissions::unassign_permission_from_user_set))
            .route("/{user_set_id}/permissions", web::get().to(user_set_permissions::get_user_set_permissions)),
    );
}
