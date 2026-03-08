use crate::AppState;
use crate::errors::APIError;
use crate::models::auth::permission::{CreateUserSetRequest, UserSet, UserSetQuery};
use crate::schema::user_sets;
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;

impl_admin_entity_service!(
    UserSetService,
    user_sets::table,
    UserSet,
    UserSet,
    user_sets::id,
    UserSetQuery,
    |q: user_sets::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(user_sets::name.like(search))
    },
    |q: user_sets::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("name", "asc") => q.order(user_sets::name.asc()),
            ("name", "desc") => q.order(user_sets::name.desc()),
            _ => q.order(user_sets::id.desc()),
        }
    }
);

impl UserSetService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateUserSetRequest,
    ) -> Result<UserSet, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::PERMISSION_SET)?;
        
        let new_item = UserSet {
            id,
            name: req.name,
            description: req.description,
        };

        Self::generic_create(pool, new_item).await
    }
}
