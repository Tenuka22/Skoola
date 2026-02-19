use apistos::web;
use crate::handlers::resources::financial_reports;
use crate::utils::jwt::Authenticated;
use crate::database::enums::PermissionEnum;
use crate::utils::permission_verification::PermissionVerification;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/financial-reports")
            .wrap(PermissionVerification { required_permission: PermissionEnum::ViewFinancialReports }) // Assuming a new permission
            .wrap(Authenticated)
            .route("/trial-balance", web::get().to(financial_reports::get_trial_balance)),
    );
}
