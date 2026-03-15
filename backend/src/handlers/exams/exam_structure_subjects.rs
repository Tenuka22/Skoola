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
    service => ExamStructureSubjectService,
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
