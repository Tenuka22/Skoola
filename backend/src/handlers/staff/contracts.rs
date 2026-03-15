use crate::models::staff::contracts::{CreateStaffContractRequest, StaffContractQuery, StaffContractResponse, UpdateStaffContractRequest};
use crate::services::staff::contracts::StaffContractService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "staff_contracts",
    entity => StaffContract,
    response => StaffContractResponse,
    query => StaffContractQuery,
    create => CreateStaffContractRequest,
    update => UpdateStaffContractRequest,
    service => StaffContractService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => update_with_logic,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete
    }
);
