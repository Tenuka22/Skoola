use crate::handlers::permissions::{PermissionQuery, BulkUpdatePermissionsRequest};
use crate::{AppState, database::tables::Permission, errors::APIError, schema::permissions};
use crate::database::enums::{PermissionSeverity, PermissionEnum};
use actix_web::web;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::AsChangeset; // Keep AsChangeset for the new struct below
use serde::{Deserialize, Serialize}; // Add for the new struct

#[derive(Debug, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = permissions)]
pub struct BulkUpdateChangeset {
    pub name: Option<PermissionEnum>,
    pub description: Option<String>,
    pub safety_level: Option<PermissionSeverity>,
}

pub async fn get_permissions_paginated(
    pool: web::Data<AppState>,
    query: PermissionQuery,
) -> Result<(Vec<Permission>, i64, i64), APIError> {
    let mut conn = pool.db_pool.get()?;
    let mut data_query = permissions::table.into_boxed();
    let mut count_query = permissions::table.into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        data_query = data_query.filter(permissions::name.like(pattern.clone()).or(permissions::description.like(pattern.clone())));
        count_query = count_query.filter(permissions::name.like(pattern.clone()).or(permissions::description.like(pattern)));
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
    permission_ids: Vec<i32>,
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

        let changeset = BulkUpdateChangeset {
            name: body.name,
            description: body.description,
            safety_level: body.safety_level,
        };

        diesel::update(target)
            .set(&changeset)
            .execute(conn)?;

        Ok(())
    })
}