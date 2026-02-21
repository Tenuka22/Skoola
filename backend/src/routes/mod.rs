pub mod auth;
pub mod academic;
pub mod exams;
pub mod resources;
pub mod staff;
pub mod students;
pub mod system;
pub mod messaging;

use apistos::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.configure(auth::configure);
    cfg.configure(academic::configure);
    cfg.configure(exams::configure);
    cfg.configure(resources::configure);
    cfg.configure(staff::configure);
    cfg.configure(students::configure);
    cfg.configure(system::configure);
    cfg.configure(messaging::configure);
}
