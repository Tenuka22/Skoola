use crate::models::system::audit_log::{AttendanceAuditLogResponse, CreateAttendanceAuditLogRequest, UpdateAttendanceAuditLogRequest, AttendanceAuditLogQuery};
use crate::services::system::attendance_audit_log::AttendanceAuditLogService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "attendance_audit_logs",
    entity => AttendanceAuditLog,
    response => AttendanceAuditLogResponse,
    query => AttendanceAuditLogQuery,
    create => CreateAttendanceAuditLogRequest,
    update => UpdateAttendanceAuditLogRequest,
    service => AttendanceAuditLogService
);

