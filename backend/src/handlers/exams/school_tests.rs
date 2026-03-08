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
    service => SchoolTestService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => update_with_logic,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "school_test_subjects",
    entity => SchoolTestSubject,
    response => SchoolTestSubject,
    query => SchoolTestSubjectQuery,
    create => CreateSchoolTestSubjectRequest,
    update => UpdateSchoolTestSubjectRequest,
    service => SchoolTestSubjectService,
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
