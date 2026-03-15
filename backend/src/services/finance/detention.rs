use crate::models::finance::detention::*;
use crate::schema::detention_balances;
use crate::{AppState, errors::APIError};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    DetentionBalanceService,
    detention_balances::table,
    DetentionBalance,
    DetentionBalanceResponse,
    detention_balances::student_id,
    student_id,
    DetentionBalanceQuery,
    |q: detention_balances::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(detention_balances::student_id.like(pattern))
    },
    |q: detention_balances::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(detention_balances::updated_at.desc()),
        }
    }
);

impl DetentionBalanceService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateDetentionBalanceRequest,
    ) -> Result<DetentionBalanceResponse, APIError> {
        let now = Utc::now().naive_utc();
        let new_item = DetentionBalance {
            student_id: req.student_id,
            total_hours_assigned: req.total_hours_assigned,
            total_hours_served: req.total_hours_served,
            remaining_hours: req.remaining_hours,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}
