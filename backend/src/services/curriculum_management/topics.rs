use crate::models::curriculum_management::curriculum_topic::{CurriculumTopicQuery, CurriculumTopicResponse, NewCurriculumTopic, CurriculumTopic};
use crate::schema::curriculum_topics;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    CurriculumTopicAdminService,
    curriculum_topics::table,
    CurriculumTopic,
    CurriculumTopicResponse,
    curriculum_topics::id,
    CurriculumTopicQuery,
    |q: curriculum_topics::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(curriculum_topics::topic_name.like(pattern))
    },
    |q: curriculum_topics::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(curriculum_topics::created_at.desc()),
        }
    }
);

impl CurriculumTopicAdminService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: NewCurriculumTopic, // NewCurriculumTopic is basically the create request
    ) -> Result<CurriculumTopicResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::CURRICULUM_TOPIC)?;
        let now = Utc::now().naive_utc();
        let new_item = CurriculumTopic {
            id,
            curriculum_standard_id: req.curriculum_standard_id,
            parent_id: req.parent_id,
            topic_name: req.topic_name,
            full_time_hours: req.full_time_hours,
            extra_time_hours: req.extra_time_hours,
            practical_hours: req.practical_hours,
            order_index: req.order_index,
            created_at: now,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}
