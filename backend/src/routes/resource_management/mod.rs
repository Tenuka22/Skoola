use apistos::web;
use crate::handlers::resource_management;
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use crate::database::enums::PermissionEnum;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/resources")
            .wrap(Authenticated)
            .service(
                web::resource("")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ResourceCreate })
                    .route(web::post().to(resource_management::create_resource)),
            )
            .service(
                web::resource("/{resource_id}")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ResourceRead })
                    .route(web::get().to(resource_management::get_resource_by_id)),
            )
            .service(
                web::resource("")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ResourceRead })
                    .route(web::get().to(resource_management::get_all_resources)),
            )
            .service(
                web::resource("/{resource_id}")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ResourceUpdate })
                    .route(web::put().to(resource_management::update_resource)),
            )
            .service(
                web::resource("/{resource_id}")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ResourceDelete })
                    .route(web::delete().to(resource_management::delete_resource)),
            )
            .service(
                web::resource("/{resource_id}/bookings")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ResourceViewBookings })
                    .route(web::get().to(resource_management::get_resource_bookings)),
            ),
    )
    .service(
        web::scope("/resource-bookings")
            .wrap(Authenticated)
            .service(
                web::resource("")
                    .wrap(PermissionVerification { required_permission: PermissionEnum::ResourceBook })
                    .route(web::post().to(resource_management::book_resource)),
            ),
    );
}
