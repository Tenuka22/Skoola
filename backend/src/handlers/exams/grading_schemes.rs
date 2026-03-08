use crate::models::exams::grading_scheme::{CreateGradingSchemeRequest, UpdateGradingSchemeRequest, GradingScheme, GradingSchemeQuery};
use crate::services::exams::grading_schemes::GradingSchemeService;
use crate::{create_admin_handlers, AppState};
use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

create_admin_handlers!(
    tag => "grading_schemes",
    entity => GradingScheme,
    response => GradingScheme,
    query => GradingSchemeQuery,
    create => CreateGradingSchemeRequest,
    update => UpdateGradingSchemeRequest,
    service => GradingSchemeService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => update_with_logic,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);
