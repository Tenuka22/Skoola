use crate::AppState;
use crate::errors::APIError;
use crate::models::exams::grading_scheme::{GradingScheme, CreateGradingSchemeRequest, UpdateGradingSchemeRequest, GradingSchemeQuery};
use crate::schema::grading_schemes;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;

impl_admin_entity_service!(
    GradingSchemeService,
    grading_schemes::table,
    GradingScheme,
    GradingScheme,
    grading_schemes::id,
    GradingSchemeQuery,
    |q: grading_schemes::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        let pattern = format!("%{}%", search);
        q.filter(grading_schemes::name.like(pattern))
    },
    |q: grading_schemes::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(grading_schemes::created_at.desc())
    }
);

impl GradingSchemeService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateGradingSchemeRequest,
    ) -> Result<GradingScheme, APIError> {
        let mut conn = pool.db_pool.get()?;
        let new_item = GradingScheme {
            id: generate_prefixed_id(&mut conn, IdPrefix::GRADING_SCHEME)?,
            name: req.name,
            grade_level_id: req.grade_level_id,
            scheme_type: req.scheme_type,
            scale_definition: req.scale_definition,
            pass_mark: req.pass_mark,
            is_default: req.is_default.unwrap_or(false),
            description: req.description,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }

    pub async fn update_with_logic(
        pool: web::Data<AppState>,
        id: String,
        req: UpdateGradingSchemeRequest,
    ) -> Result<GradingScheme, APIError> {
        Self::generic_update(pool, id, (req, grading_schemes::updated_at.eq(Utc::now().naive_utc()))).await
    }
}
