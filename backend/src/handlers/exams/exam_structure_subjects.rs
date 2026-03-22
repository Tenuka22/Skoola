use crate::models::exams::exam_structure::{ExamStructureSubjectResponse, CreateExamStructureSubjectRequest, UpdateExamStructureSubjectRequest, ExamStructureSubjectQuery};
use crate::services::exams::exam_structure_subjects::ExamStructureSubjectService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "exam_structure_subjects",
    entity => ExamStructureSubject,
    response => ExamStructureSubjectResponse,
    query => ExamStructureSubjectQuery,
    create => CreateExamStructureSubjectRequest,
    update => UpdateExamStructureSubjectRequest,
    service => ExamStructureSubjectService
);

