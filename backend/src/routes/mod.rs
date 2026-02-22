pub mod auth;
pub mod academic;
pub mod exams;
pub mod resources;
pub mod staff;
pub mod students;
pub mod system;
pub mod messaging;
pub mod resource_management;
pub mod curriculum_management;
pub mod behavior_management;

use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(|c| auth::configure(c));
    cfg.configure(|c| academic::configure(c));
    cfg.configure(|c| exams::configure(c));
    cfg.configure(|c| resources::configure(c));
    cfg.configure(|c| staff::configure(c));
    cfg.configure(|c| students::configure(c));
    cfg.configure(|c| system::configure(c));
    cfg.configure(|c| messaging::configure(c));
    cfg.configure(|c| resource_management::configure(c));
    cfg.configure(|c| curriculum_management::configure(c));
    cfg.configure(|c| behavior_management::configure(c));
}
