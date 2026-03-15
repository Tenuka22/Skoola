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
    service => AttendanceAuditLogService,
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
