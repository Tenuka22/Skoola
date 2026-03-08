use crate::models::academic::grade_level::{CreateGradeLevelRequest, UpdateGradeLevelRequest, GradeLevelResponse, GradeLevelQuery};
use crate::services::academic::grade_level::GradeLevelService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "grade_levels",
    entity => GradeLevel,
    response => GradeLevelResponse,
    query => GradeLevelQuery,
    create => CreateGradeLevelRequest,
    update => UpdateGradeLevelRequest,
    service => GradeLevelService,
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
