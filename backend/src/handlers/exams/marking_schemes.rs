use crate::models::exams::marking_scheme::{
    CreateMarkingSchemeRequest, UpdateMarkingSchemeRequest, MarkingScheme, MarkingSchemeQuery,
    CreateMarkingSchemePartRequest, UpdateMarkingSchemePartRequest, MarkingSchemePart, MarkingSchemePartQuery
};
use crate::services::exams::marking_schemes::{MarkingSchemeService, MarkingSchemePartService};
use crate::{create_admin_handlers, AppState};
use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

create_admin_handlers!(
    tag => "marking_schemes",
    entity => MarkingScheme,
    response => MarkingScheme,
    query => MarkingSchemeQuery,
    create => CreateMarkingSchemeRequest,
    update => UpdateMarkingSchemeRequest,
    service => MarkingSchemeService,
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

create_admin_handlers!(
    tag => "marking_scheme_parts",
    entity => MarkingSchemePart,
    response => MarkingSchemePart,
    query => MarkingSchemePartQuery,
    create => CreateMarkingSchemePartRequest,
    update => UpdateMarkingSchemePartRequest,
    service => MarkingSchemePartService,
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
