use crate::database::enums::PermissionEnum;
use crate::handlers::system::audit;
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/audit")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::SystemAdmin,
            })
            .wrap(Authenticated)
            .route("/logs", web::get().to(audit::get_all_audit_logs))
            .route(
                "/logs/{table_name}/{record_pk}",
                web::get().to(audit::get_record_audit_logs),
            ),
    );
}
