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
    service => ExamSubjectService
);

