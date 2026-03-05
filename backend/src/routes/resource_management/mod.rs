use crate::database::enums::PermissionEnum;
use crate::handlers::resource_management;
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/resources")
            .wrap(Authenticated)
            .service(
                web::resource("")
                    .route(
                        web::post()
                            .to(resource_management::create_resource)
                            .wrap(PermissionVerification {
                                required_permission: PermissionEnum::ResourceCreate,
                            }),
                    )
                    .route(
                        web::get()
                            .to(resource_management::get_all_resources)
                            .wrap(PermissionVerification {
                                required_permission: PermissionEnum::ResourceRead,
                            }),
                    ),
            )
            .service(
                web::resource("/{resource_id}")
                    .route(
                        web::get()
                            .to(resource_management::get_resource_by_id)
                            .wrap(PermissionVerification {
                                required_permission: PermissionEnum::ResourceRead,
                            }),
                    )
                    .route(
                        web::put()
                            .to(resource_management::update_resource)
                            .wrap(PermissionVerification {
                                required_permission: PermissionEnum::ResourceUpdate,
                            }),
                    )
                    .route(
                        web::delete()
                            .to(resource_management::delete_resource)
                            .wrap(PermissionVerification {
                                required_permission: PermissionEnum::ResourceDelete,
                            }),
                    ),
            )
            .service(
                web::resource("/{resource_id}/bookings")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceViewBookings,
                    })
                    .route(web::get().to(resource_management::get_resource_bookings)),
            ),
    )
    .service(
        web::scope("/resource-bookings")
            .wrap(Authenticated)
            .service(
                web::resource("")
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceBook,
                    })
                    .route(web::post().to(resource_management::book_resource)),
            ),
    );
}
