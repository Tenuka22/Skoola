use actix_web::web;
use crate::handlers::curriculum_management;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(curriculum_management::create_curriculum_standard);
    cfg.service(curriculum_management::get_curriculum_standard_by_id);
    cfg.service(curriculum_management::get_all_curriculum_standards);
    cfg.service(curriculum_management::update_curriculum_standard);
    cfg.service(curriculum_management::delete_curriculum_standard);
    cfg.service(curriculum_management::create_syllabus_topic);
    cfg.service(curriculum_management::get_syllabus_topic_by_id);
    cfg.service(curriculum_management::get_syllabus_topics_for_standard);
    cfg.service(curriculum_management::update_syllabus_topic);
    cfg.service(curriculum_management::delete_syllabus_topic);
}
