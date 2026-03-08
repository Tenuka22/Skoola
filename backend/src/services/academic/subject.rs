use crate::models::academic::subject::{
    AssignSubjectToGradeRequest, AssignSubjectToStreamRequest, CreateSubjectRequest,
    EnrollStudentInSubjectRequest, Subject, SubjectEnrollmentResponse, SubjectQuery, SubjectResponse,
    UpdateSubjectRequest,
};
use crate::schema::subjects;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;

impl_admin_entity_service!(
    SubjectService,
    subjects::table,
    Subject,
    SubjectResponse,
    subjects::id,
    SubjectQuery,
    |q: subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(subjects::subject_name_en.like(search.clone())
            .or(subjects::subject_code.like(search)))
    },
    |q: subjects::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("subject_name_en", "asc") => q.order(subjects::subject_name_en.asc()),
            ("subject_name_en", "desc") => q.order(subjects::subject_name_en.desc()),
            _ => q.order(subjects::created_at.desc()),
        }
    }
);

impl SubjectService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateSubjectRequest,
    ) -> Result<SubjectResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::SUBJECT)?;
        let new_item = Subject {
            id,
            subject_code: req.subject_code,
            subject_name_en: req.subject_name_en,
            subject_name_si: req.subject_name_si,
            subject_name_ta: req.subject_name_ta,
            is_core: req.is_core.unwrap_or(true),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }
}
