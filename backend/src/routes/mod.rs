use crate::{
    handlers::{
        auth::{login, refresh, register},
        hello::{hello, hello_error},
        oauth::{github_callback, google_callback},
        profile::{
            change_email, change_password, get_profile, link_github, link_google, update_profile,
        },
        verification::{resend_verification_email, verify_email},
    },
    utils::jwt::Authenticated,
};
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/refresh", web::post().to(refresh))
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
    );
    cfg.route("/", web::get().to(hello));
    cfg.route("/error", web::get().to(hello_error));
}
