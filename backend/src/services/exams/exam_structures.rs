use crate::errors::APIError;
use crate::models::exams::exam_structure::*;
use crate::schema::{exam_structure_subjects, exam_structures};
use crate::AppState;
use actix_web::web;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use chrono::Utc;
use diesel::prelude::*;
use crate::impl_admin_entity_service;

impl_admin_entity_service!(
    ExamStructureService,
    exam_structures::table,
    ExamStructure,
    ExamStructure,
    exam_structures::id,
    ExamStructureQuery,
    |q: exam_structures::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(exam_structures::name.like(search))
    },
    |q: exam_structures::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(exam_structures::created_at.desc())
    }
);

impl_admin_entity_service!(
    ExamStructureSubjectService,
    exam_structure_subjects::table,
    ExamStructureSubject,
    ExamStructureSubject,
    exam_structure_subjects::id,
    ExamStructureSubjectQuery,
    |q: exam_structure_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: exam_structure_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(exam_structure_subjects::order_index.asc())
    }
);

impl ExamStructureService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: crate::handlers::exams::exam_structures::CreateExamStructureRequest,
    ) -> Result<ExamStructure, APIError> {
        let mut conn = data.db_pool.get()?;
        let now = Utc::now().naive_utc();
        let new_item = ExamStructure {
            id: generate_prefixed_id(&mut conn, IdPrefix::EXAM_STRUCTURE)?,
            name: req.name,
            scope_type: req.scope_type,
            medium: req.medium,
            description: req.description,
            valid_from: req.valid_from,
            valid_to: req.valid_to,
            is_active: req.is_active.unwrap_or(true),
            created_at: now,
            updated_at: now,
        };
        Self::generic_create(data, new_item).await
    }
}

impl ExamStructureSubjectService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        structure_id: String,
        req: crate::handlers::exams::exam_structures::CreateExamStructureSubjectRequest,
    ) -> Result<ExamStructureSubject, APIError> {
        let mut conn = data.db_pool.get()?;
        let now = Utc::now().naive_utc();
        let new_item = ExamStructureSubject {
            id: generate_prefixed_id(&mut conn, IdPrefix::EXAM_STRUCTURE)?,
            structure_id,
            subject_id: req.subject_id,
            duration_minutes: req.duration_minutes,
            max_marks: req.max_marks,
            pass_marks: req.pass_marks,
            order_index: req.order_index,
            created_at: now,
            updated_at: now,
        };
        Self::generic_create(data, new_item).await
    }
}

// --- specialized services ---

pub async fn bulk_delete_exam_structures(
    pool: web::Data<AppState>,
    ids: Vec<String>,
) -> Result<(), APIError> {
    ExamStructureService::generic_bulk_delete(pool, ids).await
}

pub async fn bulk_update_exam_structures(
    pool: web::Data<AppState>,
    body: crate::handlers::exams::exam_structures::BulkUpdateExamStructuresRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    conn.transaction::<_, APIError, _>(|conn| {
        let target = exam_structures::table.filter(exam_structures::id.eq_any(&body.ids));
        diesel::update(target)
            .set((
                body.name.map(|v| exam_structures::name.eq(v)),
                body.scope_type.map(|v| exam_structures::scope_type.eq(v)),
                body.medium.map(|v| exam_structures::medium.eq(v)),
                body.description.map(|v| exam_structures::description.eq(v)),
                body.valid_from.map(|v| exam_structures::valid_from.eq(v)),
                body.valid_to.map(|v| exam_structures::valid_to.eq(v)),
                body.is_active.map(|v| exam_structures::is_active.eq(v)),
                exam_structures::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;
        Ok(())
    })
}
