use crate::models::curriculum_management::lesson_reviews::{LessonReviewResponse, CreateLessonReviewRequest, LessonReviewQuery, UpdateLessonReviewRequest};
use crate::services::curriculum_management::reviews::LessonReviewAdminService;
use crate::create_admin_handlers;

create_admin_handlers!(
    tag => "lesson_reviews",
    entity => LessonReview,
    response => LessonReviewResponse,
    query => LessonReviewQuery,
    create => CreateLessonReviewRequest,
    update => UpdateLessonReviewRequest,
    service => LessonReviewAdminService
);

