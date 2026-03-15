use crate::models::system::attendance::{AttendanceExcuse, AttendanceExcuseQuery, AttendanceExcuseResponse, CreateAttendanceExcuseRequest};
use crate::schema::attendance_excuses;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    AttendanceExcusesService,
    attendance_excuses::table,
    AttendanceExcuse,
    AttendanceExcuseResponse,
    attendance_excuses::id,
    AttendanceExcuseQuery,
    |q: attendance_excuses::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(attendance_excuses::attendance_record_id.like(pattern))
    },
    |q: attendance_excuses::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(attendance_excuses::created_at.desc()),
        }
    }
);

impl AttendanceExcusesService {
    pub async fn create_attendance_excuse(
        pool: web::Data<AppState>,
        req: CreateAttendanceExcuseRequest,
    ) -> Result<AttendanceExcuseResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::ATTENDANCE_EXCUSE)?;
        let new_item = AttendanceExcuse {
            id,
            attendance_record_id: req.attendance_record_id,
            excuse_type: req.excuse_type,
            document_url: req.document_url,
            is_verified: req.is_verified,
            verified_by: req.verified_by,
            created_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }
}
