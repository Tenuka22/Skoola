use actix_web::web;
use crate::handlers::behavior_management;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(behavior_management::create_behavior_incident_type);
    cfg.service(behavior_management::get_behavior_incident_type_by_id);
    cfg.service(behavior_management::get_all_behavior_incident_types);
    cfg.service(behavior_management::update_behavior_incident_type);
    cfg.service(behavior_management::delete_behavior_incident_type);
    cfg.service(behavior_management::record_behavior_incident);
    cfg.service(behavior_management::get_student_behavior_incidents);
    cfg.service(behavior_management::get_behavior_incident_by_id);
    cfg.service(behavior_management::update_behavior_incident);
    cfg.service(behavior_management::delete_behavior_incident);
}
