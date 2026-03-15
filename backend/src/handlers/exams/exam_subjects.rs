use crate::models::exams::exam_subject::{ExamSubjectResponse, CreateExamSubjectRequest, UpdateExamSubjectRequest, ExamSubjectQuery};
use crate::services::exams::exam_subjects::ExamSubjectService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "exam_subjects",
    entity => ExamSubject,
    response => ExamSubjectResponse,
    query => ExamSubjectQuery,
    create => CreateExamSubjectRequest,
    update => UpdateExamSubjectRequest,
    service => ExamSubjectService,
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
