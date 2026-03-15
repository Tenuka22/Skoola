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
    service => LessonReviewAdminService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);
