pub mod audit;

use apistos::web;
use crate::handlers::system::{activities, hello, school_settings, user};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(|cfg_local| activities::config(cfg_local));
    cfg.configure(|cfg_local| school_settings::config(cfg_local));
    cfg.configure(|cfg_local| audit::configure(cfg_local));

    cfg.route("/", web::get().to(hello::hello));
    cfg.route("/error", web::get().to(hello::hello_error));
    cfg.route("/users/bulk-delete", web::post().to(user::bulk_delete_users_handler)); // New line
}
