use crate::database::enums::PermissionEnum;
use crate::handlers::staff::staff;
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/staff")
            .wrap(PermissionVerification {
                required_permission: PermissionEnum::StaffManage,
            })
            .wrap(Authenticated)
            .route("", web::post().to(staff::create_staff))
            .route("/{id}", web::get().to(staff::get_staff_by_id))
            .route("", web::get().to(staff::get_all_staff))
            .route("/{id}", web::put().to(staff::update_staff))
            .route("/{id}", web::delete().to(staff::delete_staff))
            .route("/bulk", web::delete().to(staff::bulk_delete_staff))
            .route("/bulk", web::patch().to(staff::bulk_update_staff)),
    );
}
