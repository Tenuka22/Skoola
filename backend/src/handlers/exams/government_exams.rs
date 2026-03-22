use crate::models::exams::government_exam::{
    CreateGovernmentExamRequest, UpdateGovernmentExamRequest, GovernmentExam, GovernmentExamQuery,
    CreateGovernmentExamSubjectRequest, UpdateGovernmentExamSubjectRequest, GovernmentExamSubject, GovernmentExamSubjectQuery
};
use crate::services::exams::government_exams::{GovernmentExamService, GovernmentExamSubjectService};
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "government_exams",
    entity => GovernmentExam,
    response => GovernmentExam,
    query => GovernmentExamQuery,
    create => CreateGovernmentExamRequest,
    update => UpdateGovernmentExamRequest,
    service => GovernmentExamService
);

create_admin_handlers!(
    tag => "government_exam_subjects",
    entity => GovernmentExamSubject,
    response => GovernmentExamSubject,
    query => GovernmentExamSubjectQuery,
    create => CreateGovernmentExamSubjectRequest,
    update => UpdateGovernmentExamSubjectRequest,
    service => GovernmentExamSubjectService
);

