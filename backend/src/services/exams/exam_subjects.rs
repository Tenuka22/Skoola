use crate::models::exams::exam_subject::{ExamSubject, ExamSubjectQuery, ExamSubjectResponse, CreateExamSubjectRequest};
use crate::schema::exam_subjects;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    ExamSubjectService,
    exam_subjects::table,
    ExamSubject,
    ExamSubjectResponse,
    exam_subjects::id,
    ExamSubjectQuery,
    |q: exam_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, _pattern: String| {
        q
    },
    |q: exam_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(exam_subjects::created_at.desc()),
        }
    }
);

impl ExamSubjectService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateExamSubjectRequest,
    ) -> Result<ExamSubjectResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::EXAM_SUBJECT)?;
        let now = Utc::now().naive_utc();
        let new_item = ExamSubject {
            id,
            exam_id: req.exam_id,
            subject_id: req.subject_id,
            date: req.date,
            time: req.time,
            duration: req.duration,
            max_marks: req.max_marks,
            pass_marks: req.pass_marks,
            created_at: now,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}
