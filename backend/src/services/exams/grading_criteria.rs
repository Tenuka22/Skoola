use crate::models::exams::grading_criterion::{GradingCriterion, GradingCriterionQuery, GradingCriterionResponse, CreateGradingCriterionRequest};
use crate::schema::grading_criteria;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use diesel::prelude::*;
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    GradingCriteriaService,
    grading_criteria::table,
    GradingCriterion,
    GradingCriterionResponse,
    grading_criteria::id,
    GradingCriterionQuery,
    |q: grading_criteria::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(grading_criteria::grade.like(pattern))
    },
    |q: grading_criteria::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("grade", "asc") => q.order(grading_criteria::grade.asc()),
            ("grade", "desc") => q.order(grading_criteria::grade.desc()),
            ("min_mark", "asc") => q.order(grading_criteria::min_mark.asc()),
            ("min_mark", "desc") => q.order(grading_criteria::min_mark.desc()),
            _ => q.order(grading_criteria::created_at.desc()),
        }
    }
);

impl GradingCriteriaService {
    pub async fn create_grading_criterion(
        pool: web::Data<AppState>,
        req: CreateGradingCriterionRequest,
    ) -> Result<GradingCriterionResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::GRADING_CRITERION)?;
        let new_item = GradingCriterion {
            id,
            scheme_id: req.scheme_id,
            grade: req.grade,
            min_mark: req.min_mark,
            max_mark: req.max_mark,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }

    pub async fn get_grading_criteria_by_scheme_id(
        pool: web::Data<AppState>,
        scheme_id: String,
    ) -> Result<Vec<GradingCriterionResponse>, APIError> {
        let mut conn = pool.db_pool.get()?;
        let list = grading_criteria::table
            .filter(grading_criteria::scheme_id.eq(scheme_id))
            .load::<GradingCriterion>(&mut conn)?;
        Ok(list.into_iter().map(GradingCriterionResponse::from).collect())
    }
}
