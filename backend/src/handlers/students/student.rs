use crate::models::student::student::{CreateStudentRequest, StudentResponse, StudentQuery, Student};
use crate::services::students::student::StudentService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "students",
    entity => Student,
    response => StudentResponse,
    query => StudentQuery,
    create => CreateStudentRequest,
    update => Student,
    service => StudentService,
    methods => {
        create => generic_create,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update,
    }
);

