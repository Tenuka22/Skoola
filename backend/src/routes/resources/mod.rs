use crate::handlers::resources::fees;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(|cfg_local| fees::config(cfg_local));
}
