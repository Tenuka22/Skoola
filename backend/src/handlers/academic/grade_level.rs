use crate::models::academic::grade_level::{CreateGradeLevelRequest, UpdateGradeLevelRequest, GradeLevelResponse, GradeLevel, GradeLevelQuery};
use crate::services::academic::grade_level::GradeLevelService;
use crate::{create_admin_handlers, AppState};
use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
