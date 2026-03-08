use crate::models::exams::exam::ExamQuery;
use crate::schema::exams;
use crate::{
    AppState,
    errors::APIError,
    models::exams::exam::{CreateExamRequest, Exam, ExamResponse, UpdateExamRequest},
};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;

impl_admin_entity_service!(
    ExamService,
    exams::table,
    Exam,
    ExamResponse,
    exams::id,
    ExamQuery,
    |q: exams::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        let pattern = format!("%{}%", search);
        q.filter(exams::name.like(pattern))
    },
    |q: exams::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("name", "asc") => q.order(exams::name.asc()),
            ("name", "desc") => q.order(exams::name.desc()),
            _ => q.order(exams::start_date.desc()),
        }
    }
);

impl ExamService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateExamRequest,
    ) -> Result<ExamResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::EXAM)?;
        let new_item = Exam {
            id,
            exam_type_id: req.exam_type_id,
            name: req.name,
            academic_year_id: req.academic_year_id,
            term_id: req.term_id,
            start_date: req.start_date,
            end_date: req.end_date,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }

    pub async fn update_with_logic(
        pool: web::Data<AppState>,
        id: String,
        req: UpdateExamRequest,
    ) -> Result<ExamResponse, APIError> {
        Self::generic_update(pool, id, (req, exams::updated_at.eq(Utc::now().naive_utc()))).await
    }
}
