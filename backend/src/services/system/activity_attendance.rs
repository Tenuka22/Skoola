use crate::models::system::activity::{ActivityAttendance, ActivityAttendanceQuery, ActivityAttendanceResponse, CreateActivityAttendanceRequest};
use crate::schema::activity_attendance;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    ActivityAttendanceService,
    activity_attendance::table,
    ActivityAttendance,
    ActivityAttendanceResponse,
    activity_attendance::id,
    ActivityAttendanceQuery,
    |q: activity_attendance::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(activity_attendance::remarks.like(pattern))
    },
    |q: activity_attendance::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(activity_attendance::created_at.desc()),
        }
    }
);

impl ActivityAttendanceService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateActivityAttendanceRequest,
    ) -> Result<ActivityAttendanceResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::ATTENDANCE)?;
        let now = Utc::now().naive_utc();
        let new_item = ActivityAttendance {
            id,
            activity_id: req.activity_id,
            user_id: req.user_id,
            status: req.status,
            check_in_time: req.check_in_time,
            check_out_time: req.check_out_time,
            remarks: req.remarks,
            marked_by: req.marked_by,
            created_at: now,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}
