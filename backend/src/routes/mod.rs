pub mod academic;
pub mod auth;
pub mod behavior_management;
pub mod curriculum_management;
pub mod exams;
pub mod messaging;
pub mod resources;
pub mod staff;
pub mod students;
pub mod system;

use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .configure(|c| auth::configure_admin(c))
            .configure(|c| academic::configure(c))
            .configure(|c| exams::configure(c))
            .configure(|c| resources::configure(c))
            .configure(|c| staff::configure(c))
            .configure(|c| students::configure(c))
            .configure(|c| system::configure(c))
            .configure(|c| messaging::configure(c))
            .configure(|c| curriculum_management::configure(c))
            .configure(|c| behavior_management::configure(c)),
    );

    cfg.configure(|c| auth::configure(c));
}
