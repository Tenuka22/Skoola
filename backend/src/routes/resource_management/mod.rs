use actix_web::web;
use crate::handlers::resource_management;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(resource_management::create_resource);
    cfg.service(resource_management::get_resource_by_id);
    cfg.service(resource_management::get_all_resources);
    cfg.service(resource_management::update_resource);
    cfg.service(resource_management::delete_resource);
    cfg.service(resource_management::book_resource);
    cfg.service(resource_management::get_resource_bookings);
}
