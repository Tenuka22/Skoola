use crate::models::academic::subject::{CreateSubjectRequest, UpdateSubjectRequest, SubjectResponse, SubjectQuery};
use crate::services::academic::SubjectService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "subjects",
    entity => Subject,
    response => SubjectResponse,
    query => SubjectQuery,
    create => CreateSubjectRequest,
    update => UpdateSubjectRequest,
    service => SubjectService
);

