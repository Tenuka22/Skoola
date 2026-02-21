use actix_web::web;
use crate::handlers::system::audit;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(audit::get_all_audit_logs);
    cfg.service(audit::get_record_audit_logs);
}
