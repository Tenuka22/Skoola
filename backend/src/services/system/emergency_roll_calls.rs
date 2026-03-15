use crate::models::system::emergency::{EmergencyRollCall, EmergencyRollCallQuery, EmergencyRollCallResponse, CreateEmergencyRollCallRequest};
use crate::schema::emergency_roll_calls;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    EmergencyRollCallsService,
    emergency_roll_calls::table,
    EmergencyRollCall,
    EmergencyRollCallResponse,
    emergency_roll_calls::id,
    EmergencyRollCallQuery,
    |q: emergency_roll_calls::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(emergency_roll_calls::event_name.like(pattern))
    },
    |q: emergency_roll_calls::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("event_name", "asc") => q.order(emergency_roll_calls::event_name.asc()),
            ("event_name", "desc") => q.order(emergency_roll_calls::event_name.desc()),
            _ => q.order(emergency_roll_calls::created_at.desc()),
        }
    }
);

impl EmergencyRollCallsService {
    pub async fn create_emergency_roll_call(
        pool: web::Data<AppState>,
        req: CreateEmergencyRollCallRequest,
    ) -> Result<EmergencyRollCallResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::ROLL_CALL)?;
        let new_item = EmergencyRollCall {
            id,
            event_name: req.event_name,
            start_time: req.start_time,
            end_time: req.end_time,
            initiated_by: req.initiated_by,
            status: req.status,
            created_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }
}
