use crate::models::academic::terms::{CreateTermRequest, UpdateTermRequest, TermResponse, TermQuery};
use crate::services::academic::terms::TermService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "terms",
    entity => Term,
    response => TermResponse,
    query => TermQuery,
    create => CreateTermRequest,
    update => UpdateTermRequest,
    service => TermService,
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
