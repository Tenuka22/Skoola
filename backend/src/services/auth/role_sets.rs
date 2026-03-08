use crate::models::auth::role::RoleSetQuery;
use crate::schema::role_sets;
use crate::{
    AppState,
    errors::APIError,
    database::tables::RoleSet,
    models::auth::role::{CreateRoleSetRequest, UpdateRoleSetRequest},
};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use diesel::prelude::*;

impl_admin_entity_service!(
    RoleSetService,
    role_sets::table,
    RoleSet,
    RoleSet,
    role_sets::id,
    RoleSetQuery,
    |q: role_sets::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        let pattern = format!("%{}%", search);
        q.filter(role_sets::name.like(pattern))
    },
    |q: role_sets::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(role_sets::name.asc())
    }
);

impl RoleSetService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateRoleSetRequest,
    ) -> Result<RoleSet, APIError> {
        let mut conn = pool.db_pool.get()?;
        let new_item = RoleSet {
            id: generate_prefixed_id(&mut conn, IdPrefix::ROLE_SET)?,
            name: req.name,
            description: req.description,
        };

        Self::generic_create(pool, new_item).await
    }
}
