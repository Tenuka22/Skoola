use crate::models::system::calendar::{SchoolCalendar, SchoolCalendarQuery, SchoolCalendarResponse, CreateSchoolCalendarRequest, UpdateSchoolCalendarRequest};
use crate::schema::school_calendar;
use crate::{AppState, errors::APIError};
use actix_web::web;
use chrono::{Utc, NaiveDate};
use diesel::prelude::*;
use crate::services::admin_db::AsAdminQuery;

pub struct SchoolCalendarService;

impl SchoolCalendarService {
    pub async fn generic_get_by_id(
        pool: web::Data<AppState>,
        id_val: String,
    ) -> Result<SchoolCalendarResponse, APIError> {
        let date = NaiveDate::parse_from_str(&id_val, "%Y-%m-%d")
            .map_err(|_| APIError::bad_request("Invalid date format. Expected YYYY-MM-DD"))?;
        
        let mut conn = pool.db_pool.get()?;
        let item: SchoolCalendar = school_calendar::table.filter(school_calendar::date.eq(date)).first(&mut conn)?;
        Ok(SchoolCalendarResponse::from(item))
    }

    pub async fn generic_create(
        pool: web::Data<AppState>,
        new_item: SchoolCalendar,
    ) -> Result<SchoolCalendarResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        diesel::insert_into(school_calendar::table).values(&new_item).execute(&mut conn)?;
        Ok(SchoolCalendarResponse::from(new_item))
    }

    pub async fn generic_update(
        pool: web::Data<AppState>,
        id_val: String,
        update_data: UpdateSchoolCalendarRequest,
    ) -> Result<SchoolCalendarResponse, APIError> {
        let date = NaiveDate::parse_from_str(&id_val, "%Y-%m-%d")
            .map_err(|_| APIError::bad_request("Invalid date format. Expected YYYY-MM-DD"))?;

        let mut conn = pool.db_pool.get()?;
        diesel::update(school_calendar::table.filter(school_calendar::date.eq(date)))
            .set(update_data)
            .execute(&mut conn)?;
        
        let updated: SchoolCalendar = school_calendar::table.filter(school_calendar::date.eq(date)).first(&mut conn)?;
        Ok(SchoolCalendarResponse::from(updated))
    }

    pub async fn generic_delete(
        pool: web::Data<AppState>,
        id_val: String,
    ) -> Result<(), APIError> {
        let date = NaiveDate::parse_from_str(&id_val, "%Y-%m-%d")
            .map_err(|_| APIError::bad_request("Invalid date format. Expected YYYY-MM-DD"))?;

        let mut conn = pool.db_pool.get()?;
        diesel::delete(school_calendar::table.filter(school_calendar::date.eq(date))).execute(&mut conn)?;
        Ok(())
    }

    pub async fn generic_bulk_delete(
        pool: web::Data<AppState>,
        ids: Vec<String>,
    ) -> Result<(), APIError> {
        let dates: Vec<NaiveDate> = ids.into_iter()
            .map(|id| NaiveDate::parse_from_str(&id, "%Y-%m-%d"))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|_| APIError::bad_request("Invalid date format in bulk delete. Expected YYYY-MM-DD"))?;

        let mut conn = pool.db_pool.get()?;
        diesel::delete(school_calendar::table.filter(school_calendar::date.eq_any(dates))).execute(&mut conn)?;
        Ok(())
    }

    pub async fn generic_bulk_update(
        _pool: web::Data<AppState>,
        _req: crate::services::admin_db::BulkUpdateRequest<UpdateSchoolCalendarRequest>,
    ) -> Result<(), APIError> {
        Err(APIError::bad_request("Bulk update not supported for School Calendar"))
    }

    pub async fn generic_get_all(
        pool: web::Data<AppState>,
        query: SchoolCalendarQuery,
    ) -> Result<(Vec<SchoolCalendarResponse>, i64, i64, Option<String>), APIError> {
        let mut conn = pool.db_pool.get()?;
        
        let mut data_query = school_calendar::table.into_boxed();
        let mut count_query = school_calendar::table.into_boxed();

        let admin_q = query.as_admin_query();

        if let Some(search_term) = &admin_q.search {
            let pattern = format!("%{}%", search_term);
            data_query = data_query.filter(school_calendar::name.like(pattern.clone()));
            count_query = count_query.filter(school_calendar::name.like(pattern));
        }

        let sort_order = admin_q.sort_order.as_deref().unwrap_or("desc");
        
        if sort_order == "asc" {
            data_query = data_query.order(school_calendar::date.asc());
        } else {
            data_query = data_query.order(school_calendar::date.desc());
        }

        let limit = admin_q.limit.unwrap_or(10);
        let page = admin_q.page.unwrap_or(1);
        
        let total = count_query.count().get_result::<i64>(&mut conn)?;
        let total_pages = (total as f64 / limit as f64).ceil() as i64;

        let offset = (page - 1) * limit;
        let list: Vec<SchoolCalendar> = data_query.limit(limit).offset(offset).load::<SchoolCalendar>(&mut conn)?;

        let next_last_id = list.last().map(|item| item.date.to_string());

        Ok((
            list.into_iter().map(SchoolCalendarResponse::from).collect(),
            total,
            total_pages,
            next_last_id,
        ))
    }

    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateSchoolCalendarRequest,
    ) -> Result<SchoolCalendarResponse, APIError> {
        let now = Utc::now().naive_utc();
        let new_item = SchoolCalendar {
            date: req.date,
            day_type: req.day_type,
            name: req.name,
            is_academic_day: req.is_academic_day,
            created_at: now,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}
