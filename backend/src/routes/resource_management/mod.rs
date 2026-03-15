use crate::database::enums::PermissionEnum;
use crate::handlers::resource_management as resource_handlers;
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/resources")
            .wrap(Authenticated)
            .route(
                "",
                web::post()
                    .to(resource_handlers::create_resource)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceCreate,
                    }),
            )
            .route(
                "",
                web::get()
                    .to(resource_handlers::get_all_resource)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceRead,
                    }),
            )
            .route(
                "/{id}",
                web::get()
                    .to(resource_handlers::get_resource_by_id)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceRead,
                    }),
            )
            .route(
                "/{id}",
                web::put()
                    .to(resource_handlers::update_resource)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceUpdate,
                    }),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(resource_handlers::delete_resource)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceDelete,
                    }),
            )
            .route(
                "/bulk-delete",
                web::post()
                    .to(resource_handlers::bulk_delete_resource)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceDelete,
                    }),
            )
            .route(
                "/bulk-update",
                web::post()
                    .to(resource_handlers::bulk_update_resource)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceUpdate,
                    }),
            ),
    );

    cfg.service(
        web::scope("/resource-assets")
            .wrap(Authenticated)
            .route(
                "",
                web::post()
                    .to(resource_handlers::create_resource_asset)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceCreate,
                    }),
            )
            .route(
                "",
                web::get()
                    .to(resource_handlers::get_all_resource_asset)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceRead,
                    }),
            )
            .route(
                "/{id}",
                web::get()
                    .to(resource_handlers::get_resource_asset_by_id)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceRead,
                    }),
            )
            .route(
                "/{id}",
                web::put()
                    .to(resource_handlers::update_resource_asset)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceUpdate,
                    }),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(resource_handlers::delete_resource_asset)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceDelete,
                    }),
            ),
    );

    cfg.service(
        web::scope("/resource-details")
            .wrap(Authenticated)
            .route(
                "",
                web::post()
                    .to(resource_handlers::create_resource_detail)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceCreate,
                    }),
            )
            .route(
                "",
                web::get()
                    .to(resource_handlers::get_all_resource_detail)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceRead,
                    }),
            )
            .route(
                "/{id}",
                web::get()
                    .to(resource_handlers::get_resource_detail_by_id)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceRead,
                    }),
            )
            .route(
                "/{id}",
                web::put()
                    .to(resource_handlers::update_resource_detail)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceUpdate,
                    }),
            )
            .route(
                "/{id}",
                web::delete()
                    .to(resource_handlers::delete_resource_detail)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceDelete,
                    }),
            ),
    );

    cfg.service(
        web::scope("/resource-bookings")
            .wrap(Authenticated)
            .route(
                "/book",
                web::post()
                    .to(resource_handlers::book_resource)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceBook,
                    }),
            )
            .route(
                "/bookings/{resource_id}",
                web::get()
                    .to(resource_handlers::get_resource_bookings)
                    .wrap(PermissionVerification {
                        required_permission: PermissionEnum::ResourceViewBookings,
                    }),
            ),
    );
}
