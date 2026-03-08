use crate::models::academic::subject::{CreateSubjectRequest, UpdateSubjectRequest, SubjectResponse, SubjectQuery};
use crate::services::academic::subject::SubjectService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "subjects",
    entity => Subject,
    response => SubjectResponse,
    query => SubjectQuery,
    create => CreateSubjectRequest,
    update => UpdateSubjectRequest,
    service => SubjectService,
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
