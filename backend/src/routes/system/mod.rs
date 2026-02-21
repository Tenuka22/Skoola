pub mod audit;

use apistos::web;
use crate::handlers::system::{activities, hello, school_settings};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(|cfg_local| activities::config(cfg_local));
    cfg.configure(|cfg_local| school_settings::config(cfg_local));
    cfg.configure(audit::configure);

    cfg.route("/", web::get().to(hello::hello));
    cfg.route("/error", web::get().to(hello::hello_error));
}
