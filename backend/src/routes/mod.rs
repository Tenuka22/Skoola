use crate::handlers::hello::{hello, hello_error};
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(hello));
    cfg.route("/error", web::get().to(hello_error));
}
