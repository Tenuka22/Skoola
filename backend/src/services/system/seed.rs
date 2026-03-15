use crate::models::system::seed::{Seed, SeedQuery, SeedResponse, CreateSeedRequest};
use crate::schema::seeds;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

impl_admin_entity_service!(
    SeedAdminService,
    seeds::table,
    Seed,
    SeedResponse,
    seeds::id,
    SeedQuery,
    |q: seeds::BoxedQuery<'static, diesel::sqlite::Sqlite>, pattern: String| {
        q.filter(seeds::table_name.like(pattern))
    },
    |q: seeds::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(seeds::created_at.desc()),
        }
    }
);

impl SeedAdminService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateSeedRequest,
    ) -> Result<SeedResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::SEED)?;
        let now = Utc::now().naive_utc();
        let new_item = Seed {
            id,
            table_name: req.table_name,
            record_id: req.record_id,
            created_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}
