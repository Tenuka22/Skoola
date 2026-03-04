use crate::schema::lesson_progress_attachments;
use crate::AppState;
use crate::errors::APIError;
use crate::database::tables::LessonProgressAttachment;
use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;
use chrono::Utc;

pub async fn add_lesson_attachment(
    pool: web::Data<AppState>,
    lesson_progress_id: String,
    file_name: String,
    file_url: String,
    file_type: Option<String>,
) -> Result<LessonProgressAttachment, APIError> {
    let mut conn = pool.db_pool.get()?;
    let id = Uuid::new_v4().to_string();

    let new_attachment = LessonProgressAttachment {
        id: id.clone(),
        lesson_progress_id,
        file_name,
        file_url,
        file_type,
        created_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(lesson_progress_attachments::table)
        .values(&new_attachment)
        .execute(&mut conn)?;

    Ok(new_attachment)
}

pub async fn get_lesson_attachments(
    pool: web::Data<AppState>,
    lp_id: String,
) -> Result<Vec<LessonProgressAttachment>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let list = lesson_progress_attachments::table
        .filter(lesson_progress_attachments::lesson_progress_id.eq(lp_id))
        .select(LessonProgressAttachment::as_select())
        .load::<LessonProgressAttachment>(&mut conn)?;
    Ok(list)
}
