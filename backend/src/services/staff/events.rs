use crate::AppState;
use crate::errors::APIError;
use crate::models::staff::events::{CreateStaffEventRequest, StaffEvent, StaffEventQuery, StaffEventResponse, UpdateStaffEventRequest};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;

use crate::schema::staff_events;

impl_admin_entity_service!(
    StaffEventService,
    staff_events::table,
    StaffEvent,
    StaffEventResponse,
    staff_events::id,
    StaffEventQuery,
    |q: staff_events::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        let pattern = format!("%{}%", search);
        q.filter(staff_events::event_name.like(pattern))
    },
    |q: staff_events::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("event_name", "asc") => q.order(staff_events::event_name.asc()),
            ("event_name", "desc") => q.order(staff_events::event_name.desc()),
            _ => q.order(staff_events::created_at.desc()),
        }
    }
);

impl StaffEventService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateStaffEventRequest,
    ) -> Result<StaffEventResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;
        
        let new_item = StaffEvent {
            id,
            event_name: req.event_name,
            event_type: req.event_type,
            start_date: req.start_date,
            end_date: req.end_date,
            location: req.location,
            organizer: req.organizer,
            counts_as_attendance: req.counts_as_attendance,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }

    pub async fn update_with_logic(
        pool: web::Data<AppState>,
        id: String,
        req: UpdateStaffEventRequest,
    ) -> Result<StaffEventResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let now = Utc::now().naive_utc();
        
        diesel::update(staff_events::table.filter(staff_events::id.eq(&id)))
            .set((
                req,
                staff_events::updated_at.eq(now),
            ))
            .execute(&mut conn)?;

        let updated: StaffEvent = staff_events::table.filter(staff_events::id.eq(&id)).first(&mut conn)?;
        Ok(StaffEventResponse::from(updated))
    }
}
