use crate::schema::{practical_lesson_appeals};
use crate::database::tables::{PracticalLessonAppeal};
use crate::database::enums::AppealStatus;
use crate::AppState;
use crate::errors::APIError;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;

pub async fn submit_appeal(
    pool: web::Data<AppState>,
    lp_id: String,
    reason: String,
    evidence_url: Option<String>,
) -> Result<PracticalLessonAppeal, APIError> {
    let mut conn = pool.db_pool.get()?;
    let id = generate_prefixed_id(&mut conn, IdPrefix::APPEAL)?;

    let new_appeal = PracticalLessonAppeal {
        id: id.clone(),
        lesson_progress_id: lp_id,
        appeal_reason: reason,
        evidence_image_url: evidence_url,
        status: AppealStatus::Pending,
        reviewed_by: None,
        reviewed_at: None,
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(practical_lesson_appeals::table)
        .values(&new_appeal)
        .execute(&mut conn)?;

    Ok(new_appeal)
}

pub async fn review_appeal(
    pool: web::Data<AppState>,
    appeal_id: String,
    admin_id: String,
    new_status: AppealStatus,
) -> Result<PracticalLessonAppeal, APIError> {
    let mut conn = pool.db_pool.get()?;

    diesel::update(practical_lesson_appeals::table.find(&appeal_id))
        .set((
            practical_lesson_appeals::status.eq(new_status),
            practical_lesson_appeals::reviewed_by.eq(admin_id),
            practical_lesson_appeals::reviewed_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    let updated = practical_lesson_appeals::table.find(appeal_id).first::<PracticalLessonAppeal>(&mut conn)?;
    Ok(updated)
}
