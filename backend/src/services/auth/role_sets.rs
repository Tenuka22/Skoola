use crate::AppState;
use crate::errors::APIError;
use crate::models::auth::role::{CreateRoleSetRequest, RoleSet, RoleSetQuery};
use crate::schema::role_sets;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;

impl_admin_entity_service!(
    RoleSetService,
    role_sets::table,
    RoleSet,
    RoleSet,
    role_sets::id,
    RoleSetQuery,
    |q: role_sets::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(role_sets::name.like(search))
    },
    |q: role_sets::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("name", "asc") => q.order(role_sets::name.asc()),
            ("name", "desc") => q.order(role_sets::name.desc()),
            _ => q.order(role_sets::id.desc()),
        }
    }
);

impl RoleSetService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateRoleSetRequest,
    ) -> Result<RoleSet, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::ROLE_SET)?;
        
        let new_item = RoleSet {
            id,
            name: req.name,
            description: req.description,
        };

        Self::generic_create(pool, new_item).await
    }
}
