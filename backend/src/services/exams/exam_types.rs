use crate::models::exams::exam_type::ExamTypeQuery;
use crate::schema::exam_types;
use crate::{
    AppState,
    errors::APIError,
    models::exams::exam_type::{
        CreateExamTypeRequest, ExamType, ExamTypeResponse,
    },
};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    ExamTypeService,
    exam_types::table,
    ExamType,
    ExamTypeResponse,
    exam_types::id,
    ExamTypeQuery,
    |q: exam_types::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        let pattern = format!("%{}%", search);
        q.filter(exam_types::name.like(pattern.clone()).or(exam_types::description.like(pattern)))
    },
    |q: exam_types::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("name", "asc") => q.order(exam_types::name.asc()),
            ("name", "desc") => q.order(exam_types::name.desc()),
            _ => q.order(exam_types::created_at.desc()),
        }
    }
);

impl ExamTypeService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateExamTypeRequest,
    ) -> Result<ExamTypeResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::EXAM_TYPE)?;
        let new_item = ExamType {
            id,
            name: req.name,
            description: req.description,
            weightage: req.weightage.unwrap_or(0.0),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }
}
