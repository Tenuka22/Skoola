use crate::models::exams::exam_structure::{ExamStructureSubject, ExamStructureSubjectQuery, ExamStructureSubjectResponse, CreateExamStructureSubjectRequest};
use crate::schema::exam_structure_subjects;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    ExamStructureSubjectService,
    exam_structure_subjects::table,
    ExamStructureSubject,
    ExamStructureSubjectResponse,
    exam_structure_subjects::id,
    ExamStructureSubjectQuery,
    |q: exam_structure_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, _pattern: String| {
        q // No specific text search implemented yet for this join-like table
    },
    |q: exam_structure_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(exam_structure_subjects::created_at.desc()),
        }
    }
);

impl ExamStructureSubjectService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateExamStructureSubjectRequest,
    ) -> Result<ExamStructureSubjectResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::EXAM_STRUCTURE_SUBJECT)?;
        let now = Utc::now().naive_utc();
        let new_item = ExamStructureSubject {
            id,
            structure_id: req.structure_id,
            subject_id: req.subject_id,
            duration_minutes: req.duration_minutes,
            max_marks: req.max_marks,
            pass_marks: req.pass_marks,
            order_index: req.order_index,
            created_at: now,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}
