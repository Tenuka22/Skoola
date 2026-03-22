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
    service => StaffContractService
);

