use crate::errors::APIError;
use crate::models::exams::marking_scheme::{
    CreateMarkingSchemePartRequest, CreateMarkingSchemeRequest,
    MarkingScheme, MarkingSchemePart,
    UpdateMarkingSchemeRequest, MarkingSchemePartQuery, MarkingSchemeQuery,
};
use crate::schema::{marking_scheme_parts, marking_schemes};
use crate::AppState;
use actix_web::web;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use chrono::Utc;
use diesel::prelude::*;

impl_admin_entity_service!(
    MarkingSchemeService,
    marking_schemes::table,
    MarkingScheme,
    MarkingScheme,
    marking_schemes::id,
    MarkingSchemeQuery,
    |q: marking_schemes::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        let pattern = format!("%{}%", search);
        q.filter(marking_schemes::name.like(pattern))
    },
    |q: marking_schemes::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(marking_schemes::created_at.desc())
    }
);

impl_admin_entity_service!(
    MarkingSchemePartService,
    marking_scheme_parts::table,
    MarkingSchemePart,
    MarkingSchemePart,
    marking_scheme_parts::id,
    MarkingSchemePartQuery,
    |q: marking_scheme_parts::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search: String| {
        q
    },
    |q: marking_scheme_parts::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(marking_scheme_parts::order_index.asc())
    }
);

impl MarkingSchemeService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateMarkingSchemeRequest,
    ) -> Result<MarkingScheme, APIError> {
        let mut conn = pool.db_pool.get()?;
        let new_item = MarkingScheme {
            id: generate_prefixed_id(&mut conn, IdPrefix::MARKING_SCHEME)?,
            name: req.name,
            subject_id: req.subject_id,
            grade_level_id: req.grade_level_id,
            curriculum_standard_id: req.curriculum_standard_id,
            stream_id: req.stream_id,
            description: req.description,
            valid_from: req.valid_from,
            valid_to: req.valid_to,
            calculation_fn: req.calculation_fn,
            is_active: req.is_active.unwrap_or(true),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }

    pub async fn update_with_logic(
        pool: web::Data<AppState>,
        id: String,
        req: UpdateMarkingSchemeRequest,
    ) -> Result<MarkingScheme, APIError> {
        Self::generic_update(pool, id, (req, marking_schemes::updated_at.eq(Utc::now().naive_utc()))).await
    }
}

impl MarkingSchemePartService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateMarkingSchemePartRequest,
    ) -> Result<MarkingSchemePart, APIError> {
        let mut conn = pool.db_pool.get()?;
        let new_item = MarkingSchemePart {
            id: generate_prefixed_id(&mut conn, IdPrefix::MARKING_SCHEME)?,
            scheme_id: req.scheme_id,
            paper_label: req.paper_label,
            part_label: req.part_label,
            question_label: req.question_label,
            max_marks: req.max_marks,
            weight_ratio: req.weight_ratio,
            structure_json: req.structure_json,
            order_index: req.order_index,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        
        Self::generic_create(pool, new_item).await
    }
}
