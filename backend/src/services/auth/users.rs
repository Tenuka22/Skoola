use crate::models::auth::user::UserQuery;
use crate::schema::users;
use crate::{
    AppState,
    errors::APIError,
    models::auth::user::{User, UserResponse, UpdateUserRequest},
};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;

impl_admin_entity_service!(
    UserService,
    users::table,
    User,
    UserResponse,
    users::id,
    UserQuery,
    |q: users::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        let pattern = format!("%{}%", search);
        q.filter(users::email.like(pattern))
    },
    |q: users::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("email", "asc") => q.order(users::email.asc()),
            ("email", "desc") => q.order(users::email.desc()),
            _ => q.order(users::created_at.desc()),
        }
    }
);

impl UserService {
    pub async fn update_with_logic(
        pool: web::Data<AppState>,
        id: String,
        req: UpdateUserRequest,
    ) -> Result<UserResponse, APIError> {
        Self::generic_update(pool, id, (req, users::updated_at.eq(Utc::now().naive_utc()))).await
    }
}
