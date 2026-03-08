use crate::AppState;
use crate::errors::APIError;
use crate::models::staff::staff::{CreateStaffRequest, Staff, StaffQuery, StaffResponse};
use crate::schema::staff;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    StaffService,
    staff::table,
    Staff,
    StaffResponse,
    staff::id,
    StaffQuery,
    |q: staff::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(staff::name.like(search))
    },
    |q: staff::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("name", "asc") => q.order(staff::name.asc()),
            ("name", "desc") => q.order(staff::name.desc()),
            _ => q.order(staff::created_at.desc()),
        }
    }
);

impl StaffService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateStaffRequest,
    ) -> Result<StaffResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;
        
        let new_item = Staff {
            id,
            employee_id: req.employee_id,
            name: req.name,
            dob: req.dob,
            gender: req.gender,
            staff_type: req.staff_type,
            profile_id: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }
}
