use crate::models::curriculum_management::curriculum_topic::{CurriculumTopicResponse, NewCurriculumTopic, UpdateCurriculumTopicRequest, CurriculumTopicQuery};
use crate::services::curriculum_management::topics::CurriculumTopicAdminService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "curriculum_topics",
    entity => CurriculumTopic,
    response => CurriculumTopicResponse,
    query => CurriculumTopicQuery,
    create => NewCurriculumTopic,
    update => UpdateCurriculumTopicRequest,
    service => CurriculumTopicAdminService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);
