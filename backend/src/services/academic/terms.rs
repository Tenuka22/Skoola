use crate::models::academic::terms::{CreateTermRequest, Term, TermQuery, TermResponse, UpdateTermRequest};
use crate::schema::terms;
use crate::{AppState, errors::APIError};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;

impl_admin_entity_service!(
    TermService,
    terms::table,
    Term,
    TermResponse,
    terms::id,
    TermQuery,
    |q: terms::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| {
        q
    },
    |q: terms::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(terms::term_number.asc())
    }
);

impl TermService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateTermRequest,
    ) -> Result<TermResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::TERM)?;
        let new_item = Term {
            id,
            academic_year_id: req.academic_year_id,
            term_number: req.term_number,
            name: req.name,
            start_date: req.start_date,
            end_date: req.end_date,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }
}
