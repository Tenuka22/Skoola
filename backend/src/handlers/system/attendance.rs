use crate::models::system::attendance::{
    AttendancePolicyResponse, AttendancePolicyQuery,
    CreateAttendancePolicyRequest, UpdateAttendancePolicyRequest,
    AttendanceExcuseResponse, AttendanceExcuseQuery,
    CreateAttendanceExcuseRequest, UpdateAttendanceExcuseRequest,
    AttendanceDiscrepancyResponse, AttendanceDiscrepancyQuery,
    CreateAttendanceDiscrepancyRequest, UpdateAttendanceDiscrepancyRequest,
};
use crate::services::system::attendance_policies::AttendancePoliciesService;
use crate::services::system::attendance_excuses::AttendanceExcusesService;
use crate::services::system::attendance_discrepancies::AttendanceDiscrepanciesService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "attendance_policies",
    entity => AttendancePolicy,
    response => AttendancePolicyResponse,
    query => AttendancePolicyQuery,
    create => CreateAttendancePolicyRequest,
    update => UpdateAttendancePolicyRequest,
    service => AttendancePoliciesService,
    methods => {
        create => create_attendance_policy,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "attendance_excuses",
    entity => AttendanceExcuse,
    response => AttendanceExcuseResponse,
    query => AttendanceExcuseQuery,
    create => CreateAttendanceExcuseRequest,
    update => UpdateAttendanceExcuseRequest,
    service => AttendanceExcusesService,
    methods => {
        create => create_attendance_excuse,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "attendance_discrepancies",
    entity => AttendanceDiscrepancy,
    response => AttendanceDiscrepancyResponse,
    query => AttendanceDiscrepancyQuery,
    create => CreateAttendanceDiscrepancyRequest,
    update => UpdateAttendanceDiscrepancyRequest,
    service => AttendanceDiscrepanciesService,
    methods => {
        create => create_attendance_discrepancy,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);
