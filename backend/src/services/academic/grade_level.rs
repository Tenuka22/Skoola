use crate::models::academic::grade_level::{
    CreateGradeLevelRequest, GradeLevel, GradeLevelQuery, GradeLevelResponse,
};
use crate::schema::grade_levels;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    GradeLevelService,
    grade_levels::table,
    GradeLevel,
    GradeLevelResponse,
    grade_levels::id,
    GradeLevelQuery,
    |q: grade_levels::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(grade_levels::grade_name.like(search))
    },
    |q: grade_levels::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("grade_name", "asc") => q.order(grade_levels::grade_name.asc()),
            ("grade_name", "desc") => q.order(grade_levels::grade_name.desc()),
            _ => q.order(grade_levels::created_at.desc()),
        }
    }
);

impl GradeLevelService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateGradeLevelRequest,
    ) -> Result<GradeLevelResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::GRADE_LEVEL)?;
        let new_item = GradeLevel {
            id,
            grade_number: req.grade_number,
            grade_name: req.grade_name,
            education_level: req.education_level,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }
}
