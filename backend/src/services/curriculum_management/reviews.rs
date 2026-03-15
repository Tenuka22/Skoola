use crate::models::curriculum_management::lesson_reviews::{LessonReview, LessonReviewQuery, LessonReviewResponse, CreateLessonReviewRequest};
use crate::schema::lesson_reviews;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    LessonReviewAdminService,
    lesson_reviews::table,
    LessonReview,
    LessonReviewResponse,
    lesson_reviews::id,
    LessonReviewQuery,
    |q: lesson_reviews::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(lesson_reviews::feedback_text.like(pattern))
    },
    |q: lesson_reviews::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(lesson_reviews::created_at.desc()),
        }
    }
);

impl LessonReviewAdminService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateLessonReviewRequest,
    ) -> Result<LessonReviewResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::LESSON_REVIEW)?;
        let now = Utc::now().naive_utc();
        let new_item = LessonReview {
            id,
            lesson_progress_id: req.lesson_progress_id,
            reviewer_type: req.reviewer_type,
            reviewer_id: req.reviewer_id,
            clarity_rating: req.clarity_rating,
            feedback_text: req.feedback_text,
            created_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}
