use crate::models::system::attendance::{AttendancePolicy, AttendancePolicyQuery, AttendancePolicyResponse, CreateAttendancePolicyRequest};
use crate::schema::attendance_policies;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;

impl_admin_entity_service!(
    AttendancePoliciesService,
    attendance_policies::table,
    AttendancePolicy,
    AttendancePolicyResponse,
    attendance_policies::id,
    AttendancePolicyQuery,
    |q: attendance_policies::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(attendance_policies::name.like(pattern))
    },
    |q: attendance_policies::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("name", "asc") => q.order(attendance_policies::name.asc()),
            ("name", "desc") => q.order(attendance_policies::name.desc()),
            _ => q.order(attendance_policies::id.desc()),
        }
    }
);

impl AttendancePoliciesService {
    pub async fn create_attendance_policy(
        pool: web::Data<AppState>,
        req: CreateAttendancePolicyRequest,
    ) -> Result<AttendancePolicyResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::ATTENDANCE_POLICY)?;
        let new_item = AttendancePolicy {
            id,
            name: req.name,
            rule_type: req.rule_type,
            threshold: req.threshold,
            consequence_type: req.consequence_type,
            consequence_value: req.consequence_value,
            is_active: req.is_active,
        };

        Self::generic_create(pool, new_item).await
    }
}
