use crate::models::exams::exam::{CreateExamRequest, UpdateExamRequest, ExamResponse, ExamQuery};
use crate::services::exams::exams::ExamService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "exams",
    entity => Exam,
    response => ExamResponse,
    query => ExamQuery,
    create => CreateExamRequest,
    update => UpdateExamRequest,
    service => ExamService
);

