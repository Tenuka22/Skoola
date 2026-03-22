use crate::models::academic::grade_level::{CreateGradeLevelRequest, UpdateGradeLevelRequest, GradeLevelResponse, GradeLevelQuery};
use crate::services::academic::GradeLevelService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "grade_levels",
    entity => GradeLevel,
    response => GradeLevelResponse,
    query => GradeLevelQuery,
    create => CreateGradeLevelRequest,
    update => UpdateGradeLevelRequest,
    service => GradeLevelService
);

