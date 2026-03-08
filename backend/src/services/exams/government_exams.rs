use crate::errors::APIError;
use crate::models::exams::government_exam::{
    GovernmentExam, GovernmentExamSubject, CreateGovernmentExamRequest, UpdateGovernmentExamRequest,
    CreateGovernmentExamSubjectRequest, GovernmentExamQuery, GovernmentExamSubjectQuery,
};
use crate::schema::{government_exam_subjects, government_exams};
use crate::AppState;
use actix_web::web;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use chrono::Utc;
use diesel::prelude::*;

impl_admin_entity_service!(
    GovernmentExamService,
    government_exams::table,
    GovernmentExam,
    GovernmentExam,
    government_exams::id,
    GovernmentExamQuery,
    |q: government_exams::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        let pattern = format!("%{}%", search);
        q.filter(government_exams::name.like(pattern))
    },
    |q: government_exams::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(government_exams::created_at.desc())
    }
);

impl_admin_entity_service!(
    GovernmentExamSubjectService,
    government_exam_subjects::table,
    GovernmentExamSubject,
    GovernmentExamSubject,
    government_exam_subjects::id,
    GovernmentExamSubjectQuery,
    |q: government_exam_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search: String| {
        q
    },
    |q: government_exam_subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(government_exam_subjects::exam_date.asc())
    }
);

impl GovernmentExamService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateGovernmentExamRequest,
    ) -> Result<GovernmentExam, APIError> {
        let mut conn = pool.db_pool.get()?;
        let new_item = GovernmentExam {
            id: generate_prefixed_id(&mut conn, IdPrefix::GOVERNMENT_EXAM)?,
            exam_structure_id: req.exam_structure_id,
            name: req.name,
            authority: req.authority,
            level: req.level,
            exam_year: req.exam_year,
            start_date: req.start_date,
            end_date: req.end_date,
            status: req.status,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }

    pub async fn update_with_logic(
        pool: web::Data<AppState>,
        id: String,
        req: UpdateGovernmentExamRequest,
    ) -> Result<GovernmentExam, APIError> {
        Self::generic_update(pool, id, (req, government_exams::updated_at.eq(Utc::now().naive_utc()))).await
    }
}

impl GovernmentExamSubjectService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateGovernmentExamSubjectRequest,
    ) -> Result<GovernmentExamSubject, APIError> {
        let mut conn = pool.db_pool.get()?;
        let new_item = GovernmentExamSubject {
            id: generate_prefixed_id(&mut conn, IdPrefix::GOVERNMENT_EXAM)?,
            government_exam_id: req.government_exam_id,
            subject_id: req.subject_id,
            exam_date: req.exam_date,
            exam_time: req.exam_time,
            duration_minutes: req.duration_minutes,
            max_marks: req.max_marks,
            pass_marks: req.pass_marks,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        
        Self::generic_create(pool, new_item).await
    }
}
