pub mod academic;
pub mod admin_db_crud;
pub mod auth;
pub mod behavior_management;
pub mod curriculum_management;
pub mod exams;
pub mod messaging;
pub mod resources;
pub mod resource_management;
pub mod staff;
pub mod students;
pub mod system;

use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .configure(|c| admin_db_crud::configure(c))
            .configure(|c| auth::configure_admin(c))
            .configure(|c| academic::configure(c))
            .configure(|c| exams::configure(c))
            .configure(|c| resources::configure(c))
            .configure(|c| resource_management::configure(c))
            .configure(|c| staff::configure(c))
            .configure(|c| students::configure(c))
            .configure(|c| system::configure(c))
            .configure(|c| messaging::configure(c))
            .configure(|c| curriculum_management::configure(c))
            .configure(|c| behavior_management::configure(c)),
    );

    cfg.configure(|c| auth::configure(c));
}
