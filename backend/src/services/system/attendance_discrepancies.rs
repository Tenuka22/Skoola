use crate::models::system::attendance::{AttendanceDiscrepancy, AttendanceDiscrepancyQuery, AttendanceDiscrepancyResponse, CreateAttendanceDiscrepancyRequest};
use crate::schema::attendance_discrepancies;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    AttendanceDiscrepanciesService,
    attendance_discrepancies::table,
    AttendanceDiscrepancy,
    AttendanceDiscrepancyResponse,
    attendance_discrepancies::id,
    AttendanceDiscrepancyQuery,
    |q: attendance_discrepancies::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(attendance_discrepancies::student_id.like(pattern))
    },
    |q: attendance_discrepancies::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(attendance_discrepancies::created_at.desc()),
        }
    }
);

impl AttendanceDiscrepanciesService {
    pub async fn create_attendance_discrepancy(
        pool: web::Data<AppState>,
        req: CreateAttendanceDiscrepancyRequest,
    ) -> Result<AttendanceDiscrepancyResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::ATTENDANCE_DISCREPANCY)?;
        let new_item = AttendanceDiscrepancy {
            id,
            student_id: req.student_id,
            date: req.date,
            discrepancy_type: req.discrepancy_type,
            details: req.details,
            severity: req.severity,
            is_resolved: req.is_resolved,
            resolved_by: req.resolved_by,
            created_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }
}
