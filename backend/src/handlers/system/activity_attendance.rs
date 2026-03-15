use crate::models::system::activity::{ActivityAttendanceResponse, CreateActivityAttendanceRequest, UpdateActivityAttendanceRequest, ActivityAttendanceQuery};
use crate::services::system::activity_attendance::ActivityAttendanceService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "activity_attendance",
    entity => ActivityAttendance,
    response => ActivityAttendanceResponse,
    query => ActivityAttendanceQuery,
    create => CreateActivityAttendanceRequest,
    update => UpdateActivityAttendanceRequest,
    service => ActivityAttendanceService,
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
