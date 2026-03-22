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
    service => AttendancePoliciesService
);

create_admin_handlers!(
    tag => "attendance_excuses",
    entity => AttendanceExcuse,
    response => AttendanceExcuseResponse,
    query => AttendanceExcuseQuery,
    create => CreateAttendanceExcuseRequest,
    update => UpdateAttendanceExcuseRequest,
    service => AttendanceExcusesService
);

create_admin_handlers!(
    tag => "attendance_discrepancies",
    entity => AttendanceDiscrepancy,
    response => AttendanceDiscrepancyResponse,
    query => AttendanceDiscrepancyQuery,
    create => CreateAttendanceDiscrepancyRequest,
    update => UpdateAttendanceDiscrepancyRequest,
    service => AttendanceDiscrepanciesService
);

