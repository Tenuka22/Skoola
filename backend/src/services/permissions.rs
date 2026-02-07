use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use crate::{
    errors::APIError,
    AppState,
    database::tables::Permission,
    schema::permissions,
};
use actix_web::web;
use crate::handlers::permissions::{PermissionQuery, BulkUpdatePermissionsRequest};

pub async fn get_permissions_paginated(
    pool: web::Data<AppState>,
    query: PermissionQuery,
) -> Result<(Vec<Permission>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = permissions::table.into_boxed();
    let mut count_query = permissions::table.into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        data_query = data_query.filter(permissions::name.like(pattern.clone()));
        count_query = count_query.filter(permissions::name.like(pattern));
    }

    let sort_by = query.sort_by.as_deref().unwrap_or("name");
    let sort_order = query.sort_order.as_deref().unwrap_or("asc");

    data_query = match (sort_by, sort_order) {
        ("name", "asc") => data_query.order(permissions::name.asc()),
        ("name", "desc") => data_query.order(permissions::name.desc()),
        _ => data_query.order(permissions::name.asc()),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let total_permissions = count_query.count().get_result(&mut conn)?;
    let total_pages = (total_permissions as f64 / limit as f64).ceil() as i64;

    let permissions_list: Vec<Permission> = data_query
        .limit(limit)
        .offset(offset)
        .load::<Permission>(&mut conn)?;

    Ok((permissions_list, total_permissions, total_pages))
}

pub async fn bulk_delete_permissions(
    pool: web::Data<AppState>,
    permission_ids: Vec<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::delete(permissions::table.filter(permissions::id.eq_any(permission_ids)))
        .execute(&mut conn)?;
    Ok(())
}

pub async fn bulk_update_permissions(
    pool: web::Data<AppState>,
    body: BulkUpdatePermissionsRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    conn.transaction::<_, APIError, _>(|conn| {
        let target = permissions::table.filter(permissions::id.eq_any(&body.permission_ids));
        
        diesel::update(target)
            .set((
                body.name.map(|n| permissions::name.eq(n)),
            ))
            .execute(conn)?;
        
        Ok(())
    })
}