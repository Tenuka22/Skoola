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
    service => ActivityAttendanceService
);

