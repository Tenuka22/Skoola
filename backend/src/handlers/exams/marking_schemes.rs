use crate::models::exams::marking_scheme::{
    CreateMarkingSchemeRequest, UpdateMarkingSchemeRequest, MarkingScheme, MarkingSchemeQuery,
    CreateMarkingSchemePartRequest, UpdateMarkingSchemePartRequest, MarkingSchemePart, MarkingSchemePartQuery
};
use crate::services::exams::marking_schemes::{MarkingSchemeService, MarkingSchemePartService};
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "marking_schemes",
    entity => MarkingScheme,
    response => MarkingScheme,
    query => MarkingSchemeQuery,
    create => CreateMarkingSchemeRequest,
    update => UpdateMarkingSchemeRequest,
    service => MarkingSchemeService
);

create_admin_handlers!(
    tag => "marking_scheme_parts",
    entity => MarkingSchemePart,
    response => MarkingSchemePart,
    query => MarkingSchemePartQuery,
    create => CreateMarkingSchemePartRequest,
    update => UpdateMarkingSchemePartRequest,
    service => MarkingSchemePartService
);

