use crate::models::academic::class::{Class, ClassQuery, ClassResponse, CreateClassRequest, UpdateClassRequest};
use crate::schema::classes;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;

impl_admin_entity_service!(
    ClassService,
    classes::table,
    Class,
    ClassResponse,
    classes::id,
    ClassQuery,
    |q: classes::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| {
        q
    },
    |q: classes::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(classes::created_at.desc())
    }
);

impl ClassService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateClassRequest,
    ) -> Result<ClassResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::CLASS)?;
        let new_item = Class {
            id,
            grade_id: req.grade_id,
            academic_year_id: req.academic_year_id,
            class_teacher_id: req.class_teacher_id,
            medium: req.medium,
            room_id: req.room_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }
}
