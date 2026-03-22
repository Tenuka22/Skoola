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
    service => ExamTypeService
);

