use crate::models::staff::events::{CreateStaffEventRequest, StaffEventQuery, StaffEventResponse, UpdateStaffEventRequest};
use crate::services::staff::events::StaffEventService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "staff_events",
    entity => StaffEvent,
    response => StaffEventResponse,
    query => StaffEventQuery,
    create => CreateStaffEventRequest,
    update => UpdateStaffEventRequest,
    service => StaffEventService
);

