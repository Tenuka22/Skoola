use crate::models::system::emergency::{
    EmergencyRollCallResponse, CreateEmergencyRollCallRequest, UpdateEmergencyRollCallRequest, EmergencyRollCallQuery,
};
use crate::services::system::emergency_roll_calls::EmergencyRollCallsService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "emergency_roll_calls",
    entity => EmergencyRollCall,
    response => EmergencyRollCallResponse,
    query => EmergencyRollCallQuery,
    create => CreateEmergencyRollCallRequest,
    update => UpdateEmergencyRollCallRequest,
    service => EmergencyRollCallsService,
    methods => {
        create => create_emergency_roll_call,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);
