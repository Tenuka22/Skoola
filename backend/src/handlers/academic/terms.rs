use crate::models::academic::terms::{CreateTermRequest, UpdateTermRequest, TermResponse, TermQuery};
use crate::services::academic::TermService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "terms",
    entity => Term,
    response => TermResponse,
    query => TermQuery,
    create => CreateTermRequest,
    update => UpdateTermRequest,
    service => TermService
);

