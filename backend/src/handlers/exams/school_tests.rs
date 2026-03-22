use crate::models::exams::school_test::{
    CreateSchoolTestRequest, UpdateSchoolTestRequest, SchoolTest, SchoolTestQuery,
    CreateSchoolTestSubjectRequest, UpdateSchoolTestSubjectRequest, SchoolTestSubject, SchoolTestSubjectQuery
};
use crate::services::exams::school_tests::{SchoolTestService, SchoolTestSubjectService};
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "school_tests",
    entity => SchoolTest,
    response => SchoolTest,
    query => SchoolTestQuery,
    create => CreateSchoolTestRequest,
    update => UpdateSchoolTestRequest,
    service => SchoolTestService
);

create_admin_handlers!(
    tag => "school_test_subjects",
    entity => SchoolTestSubject,
    response => SchoolTestSubject,
    query => SchoolTestSubjectQuery,
    create => CreateSchoolTestSubjectRequest,
    update => UpdateSchoolTestSubjectRequest,
    service => SchoolTestSubjectService
);

