use crate::models::staff::staff::{CreateStaffRequest, UpdateStaffRequest, StaffResponse, StaffQuery};
use crate::services::staff::staff::StaffService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "staff",
    entity => Staff,
    response => StaffResponse,
    query => StaffQuery,
    create => CreateStaffRequest,
    update => UpdateStaffRequest,
    service => StaffService,
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
