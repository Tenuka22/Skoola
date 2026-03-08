use crate::models::exams::exam_type::{CreateExamTypeRequest, UpdateExamTypeRequest, ExamTypeResponse, ExamTypeQuery};
use crate::services::exams::exam_types::ExamTypeService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "exam_types",
    entity => ExamType,
    response => ExamTypeResponse,
    query => ExamTypeQuery,
    create => CreateExamTypeRequest,
    update => UpdateExamTypeRequest,
    service => ExamTypeService,
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
