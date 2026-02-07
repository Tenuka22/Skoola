use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use crate::{
    errors::APIError,
    AppState,
    database::tables::Role,
    schema::roles,
};
use actix_web::web;
use crate::handlers::roles::{RoleQuery, BulkUpdateRolesRequest};

pub async fn get_roles_paginated(
    pool: web::Data<AppState>,
    query: RoleQuery,
) -> Result<(Vec<Role>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = roles::table.into_boxed();
    let mut count_query = roles::table.into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        data_query = data_query.filter(roles::name.like(pattern.clone()));
        count_query = count_query.filter(roles::name.like(pattern));
    }

    if let Some(parent_id) = &query.parent_id {
        data_query = data_query.filter(roles::parent_id.eq(parent_id));
        count_query = count_query.filter(roles::parent_id.eq(parent_id));
    }

    let sort_by = query.sort_by.as_deref().unwrap_or("name");
    let sort_order = query.sort_order.as_deref().unwrap_or("asc");

    data_query = match (sort_by, sort_order) {
        ("name", "asc") => data_query.order(roles::name.asc()),
        ("name", "desc") => data_query.order(roles::name.desc()),
        _ => data_query.order(roles::name.asc()),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let total_roles = count_query.count().get_result(&mut conn)?;
    let total_pages = (total_roles as f64 / limit as f64).ceil() as i64;

    let roles_list: Vec<Role> = data_query
        .limit(limit)
        .offset(offset)
        .load::<Role>(&mut conn)?;

    Ok((roles_list, total_roles, total_pages))
}

pub async fn bulk_delete_roles(
    pool: web::Data<AppState>,
    role_ids: Vec<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::delete(roles::table.filter(roles::id.eq_any(role_ids)))
        .execute(&mut conn)?;
    Ok(())
}

pub async fn bulk_update_roles(
    pool: web::Data<AppState>,
    body: BulkUpdateRolesRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    conn.transaction::<_, APIError, _>(|conn| {
        let target = roles::table.filter(roles::id.eq_any(&body.role_ids));
        
        diesel::update(target)
            .set((
                body.name.map(|n| roles::name.eq(n)),
                body.parent_id.map(|pi| roles::parent_id.eq(Some(pi))),
            ))
            .execute(conn)?;
        
        Ok(())
    })
}