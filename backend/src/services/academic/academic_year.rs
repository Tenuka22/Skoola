use crate::models::academic::academic_year::{
    AcademicYear, AcademicYearQuery, AcademicYearResponse, CreateAcademicYearRequest,
    UpdateAcademicYearRequest,
};
use crate::schema::academic_years;
use crate::{
    errors::APIError,
    AppState,
};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;

impl_admin_entity_service!(
    AcademicYearService,
    academic_years::table,
    AcademicYear,
    AcademicYearResponse,
    academic_years::id,
    AcademicYearQuery,
    |q: academic_years::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(academic_years::name.like(search))
    },
    |q: academic_years::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("name", "asc") => q.order(academic_years::name.asc()),
            ("name", "desc") => q.order(academic_years::name.desc()),
            _ => q.order(academic_years::created_at.desc()),
        }
    }
);

impl AcademicYearService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateAcademicYearRequest,
    ) -> Result<AcademicYearResponse, APIError> {
        let mut conn = pool.db_pool.get()?;

        if req.current.unwrap_or(false) {
            diesel::update(academic_years::table)
                .set(academic_years::current.eq(false))
                .execute(&mut conn)?;
        }

        let id = generate_prefixed_id(&mut conn, IdPrefix::ACADEMIC_YEAR)?;
        let new_item = AcademicYear {
            id,
            year_start: req.year_start,
            year_end: req.year_end,
            name: req.name,
            current: req.current.unwrap_or(false),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }

    pub async fn update_with_logic(
        pool: web::Data<AppState>,
        id: String,
        req: UpdateAcademicYearRequest,
    ) -> Result<AcademicYearResponse, APIError> {
        let mut conn = pool.db_pool.get()?;

        if req.current.unwrap_or(false) {
            diesel::update(academic_years::table)
                .set(academic_years::current.eq(false))
                .execute(&mut conn)?;
        }

        Self::generic_update(pool, id, (req, academic_years::updated_at.eq(Utc::now().naive_utc()))).await
    }

    pub async fn set_current(
        pool: web::Data<AppState>,
        id: String,
    ) -> Result<AcademicYearResponse, APIError> {
        let mut conn = pool.db_pool.get()?;

        conn.transaction::<_, APIError, _>(|conn| {
            diesel::update(academic_years::table)
                .set(academic_years::current.eq(false))
                .execute(conn)?;

            diesel::update(academic_years::table.find(&id))
                .set(academic_years::current.eq(true))
                .execute(conn)?;

            Ok(())
        })?;

        Self::generic_get_by_id(pool, id).await
    }

    pub async fn get_current(pool: web::Data<AppState>) -> Result<AcademicYearResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        let item: AcademicYear = academic_years::table
            .filter(academic_years::current.eq(true))
            .first(&mut conn)?;
        Ok(AcademicYearResponse::from(item))
    }
}
