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
    service => EmergencyRollCallsService
);

