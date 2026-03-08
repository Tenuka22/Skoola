use crate::models::auth::permission::UserSetQuery;
use crate::schema::user_sets;
use crate::{
    AppState,
    errors::APIError,
    models::auth::permission::{CreateUserSetRequest, UpdateUserSetRequest, UserSet},
};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use diesel::prelude::*;

impl_admin_entity_service!(
    UserSetService,
    user_sets::table,
    UserSet,
    UserSet,
    user_sets::id,
    UserSetQuery,
    |q: user_sets::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        let pattern = format!("%{}%", search);
        q.filter(user_sets::name.like(pattern))
    },
    |q: user_sets::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(user_sets::name.asc())
    }
);

impl UserSetService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateUserSetRequest,
    ) -> Result<UserSet, APIError> {
        let mut conn = pool.db_pool.get()?;
        let new_item = UserSet {
            id: generate_prefixed_id(&mut conn, IdPrefix::PERMISSION_SET)?,
            name: req.name,
            description: req.description,
        };

        Self::generic_create(pool, new_item).await
    }
}
