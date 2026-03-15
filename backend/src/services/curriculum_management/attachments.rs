use crate::schema::lesson_progress_attachments;
use crate::AppState;
use crate::errors::APIError;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::models::curriculum_management::lesson_progress_attachment::{
    LessonProgressAttachment,
    LessonProgressAttachmentQuery, LessonProgressAttachmentResponse, CreateLessonProgressAttachmentRequest,
};
use diesel::prelude::*;
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    LessonProgressAttachmentService,
    lesson_progress_attachments::table,
    LessonProgressAttachment,
    LessonProgressAttachmentResponse,
    lesson_progress_attachments::id,
    LessonProgressAttachmentQuery,
    |q: lesson_progress_attachments::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(lesson_progress_attachments::file_name.like(search))
    },
    |q: lesson_progress_attachments::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(lesson_progress_attachments::created_at.desc())
    }
);

impl LessonProgressAttachmentService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateLessonProgressAttachmentRequest,
    ) -> Result<LessonProgressAttachmentResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::ATTACHMENT)?;

        let new_attachment = LessonProgressAttachment {
            id: id.clone(),
            lesson_progress_id: req.lesson_progress_id,
            file_name: req.file_name,
            file_url: req.file_url,
            file_type: req.file_type,
            created_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_attachment).await
    }
}

pub async fn add_lesson_attachment(
    pool: web::Data<AppState>,
    lesson_progress_id: String,
    file_name: String,
    file_url: String,
    file_type: Option<String>,
) -> Result<LessonProgressAttachment, APIError> {
    let mut conn = pool.db_pool.get()?;
    let id = generate_prefixed_id(&mut conn, IdPrefix::ATTACHMENT)?;

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
