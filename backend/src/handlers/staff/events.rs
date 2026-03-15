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
    service => StaffEventService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => update_with_logic,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);
