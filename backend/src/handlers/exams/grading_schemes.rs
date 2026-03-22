use crate::models::exams::grading_scheme::{CreateGradingSchemeRequest, UpdateGradingSchemeRequest, GradingScheme, GradingSchemeQuery};
use crate::services::exams::grading_schemes::GradingSchemeService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "grading_schemes",
    entity => GradingScheme,
    response => GradingScheme,
    query => GradingSchemeQuery,
    create => CreateGradingSchemeRequest,
    update => UpdateGradingSchemeRequest,
    service => GradingSchemeService
);

