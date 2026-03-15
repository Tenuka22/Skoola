use crate::models::system::audit_log::{AttendanceAuditLog, AttendanceAuditLogQuery, AttendanceAuditLogResponse, CreateAttendanceAuditLogRequest};
use crate::schema::attendance_audit_log;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    AttendanceAuditLogService,
    attendance_audit_log::table,
    AttendanceAuditLog,
    AttendanceAuditLogResponse,
    attendance_audit_log::id,
    AttendanceAuditLogQuery,
    |q: attendance_audit_log::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(attendance_audit_log::change_reason.like(pattern))
    },
    |q: attendance_audit_log::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(attendance_audit_log::changed_at.desc()),
        }
    }
);

impl AttendanceAuditLogService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateAttendanceAuditLogRequest,
    ) -> Result<AttendanceAuditLogResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::AUDIT)?;
        let new_item = AttendanceAuditLog {
            id,
            attendance_type: req.attendance_type,
            attendance_record_id: req.attendance_record_id,
            old_status: req.old_status,
            new_status: req.new_status,
            change_reason: req.change_reason,
            changed_by: req.changed_by,
            changed_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }
}
