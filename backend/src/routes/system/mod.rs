use crate::handlers::system::school_settings;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(|cfg_local| school_settings::config(cfg_local));
}
