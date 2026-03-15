use crate::models::student::student::{CreateStudentRequest, UpdateStudentRequest, StudentResponse, StudentQuery};
use crate::services::students::student::StudentService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "students",
    entity => Student,
    response => StudentResponse,
    query => StudentQuery,
    create => CreateStudentRequest,
    update => UpdateStudentRequest,
    service => StudentService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => update_with_logic,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => bulk_update_students
    }
);
