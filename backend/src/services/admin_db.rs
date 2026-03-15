use crate::AppState;
use crate::errors::APIError;
use actix_web::web;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Deserialize, Clone, Default)]
#[derive(JsonSchema, ApiComponent)]
pub struct AdminQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

pub trait AsAdminQuery {
    fn as_admin_query(&self) -> AdminQuery;
}

impl AsAdminQuery for AdminQuery {
    fn as_admin_query(&self) -> AdminQuery {
        self.clone()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(ApiComponent, JsonSchema)]
pub struct PaginatedResponse<T: JsonSchema + ApiComponent> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub next_last_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(JsonSchema, ApiComponent)]
pub struct BulkIdRequest {
    pub ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(JsonSchema, ApiComponent)]
pub struct BulkIdRequestI32 {
    pub ids: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(JsonSchema, ApiComponent)]
pub struct BulkUpdateRequest<T: JsonSchema + ApiComponent> {
    pub updates: Vec<EntityUpdate<T>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(JsonSchema, ApiComponent)]
pub struct BulkUpdateRequestI32<T: JsonSchema + ApiComponent> {
    pub updates: Vec<EntityUpdateI32<T>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(JsonSchema, ApiComponent)]
pub struct BulkCreateRequest<T: JsonSchema + ApiComponent> {
    pub items: Vec<T>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(JsonSchema, ApiComponent)]
pub struct EntityUpdate<T: JsonSchema + ApiComponent> {
    pub id: String,
    pub data: T,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(JsonSchema, ApiComponent)]
pub struct EntityUpdateI32<T: JsonSchema + ApiComponent> {
    pub id: i32,
    pub data: T,
}

pub struct AdminEntityDbActions;

impl AdminEntityDbActions {
    pub fn get_conn(pool: &web::Data<AppState>) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, APIError> {
        pool.db_pool.get().map_err(APIError::from)
    }
}

#[macro_export]
macro_rules! impl_admin_entity_service_i32 {
    ($service_name:ident, $table:path, $model:ty, $resp:ty, $id_col:path, $query_type:ty, $search_logic:expr, $sort_logic:expr) => {
        pub struct $service_name;

        impl $service_name {
            pub async fn generic_get_by_id(
                pool: actix_web::web::Data<crate::AppState>,
                id_val: i32,
            ) -> Result<$resp, crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                let item: $model = $table.filter($id_col.eq(id_val)).first(&mut conn)?;
                Ok(<$resp>::from(item))
            }

            pub async fn generic_create(
                pool: actix_web::web::Data<crate::AppState>,
                new_item: $model,
            ) -> Result<$resp, crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&new_item).execute(&mut conn)?;
                Ok(<$resp>::from(new_item))
            }

            pub async fn generic_bulk_create(
                pool: actix_web::web::Data<crate::AppState>,
                new_items: Vec<$model>,
            ) -> Result<Vec<$resp>, crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&new_items).execute(&mut conn)?;
                Ok(new_items.into_iter().map(<$resp>::from).collect())
            }

            pub async fn generic_update<U>(
                pool: actix_web::web::Data<crate::AppState>,
                id_val: i32,
                update_data: U,
            ) -> Result<$resp, crate::errors::APIError>
            where
                U: diesel::AsChangeset<Target = <$table as diesel::associations::HasTable>::Table> + Send + 'static,
                <$table as diesel::associations::HasTable>::Table: diesel::query_builder::IntoUpdateTarget,
                <U as diesel::AsChangeset>::Changeset: diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>,
            {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::update($table.filter($id_col.eq(&id_val)))
                    .set(update_data)
                    .execute(&mut conn)?;
                let updated: $model = $table.filter($id_col.eq(id_val)).first(&mut conn)?;
                Ok(<$resp>::from(updated))
            }

            pub async fn generic_bulk_update<U>(
                pool: actix_web::web::Data<crate::AppState>,
                req: crate::services::admin_db::BulkUpdateRequestI32<U>,
            ) -> Result<(), crate::errors::APIError>
            where
                U: diesel::AsChangeset<Target = <$table as diesel::associations::HasTable>::Table> + Send + 'static + schemars::JsonSchema + apistos::ApiComponent,
                <$table as diesel::associations::HasTable>::Table: diesel::query_builder::IntoUpdateTarget,
                <U as diesel::AsChangeset>::Changeset: diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>,
            {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                conn.transaction::<_, crate::errors::APIError, _>(|conn| {
                    for update in req.updates {
                        diesel::update($table.filter($id_col.eq(update.id)))
                            .set(update.data)
                            .execute(conn)?;
                    }
                    Ok(())
                })?;
                Ok(())
            }

            pub async fn generic_delete(
                pool: actix_web::web::Data<crate::AppState>,
                id_val: i32,
            ) -> Result<(), crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::delete($table.filter($id_col.eq(id_val))).execute(&mut conn)?;
                Ok(())
            }

            pub async fn generic_bulk_delete(
                pool: actix_web::web::Data<crate::AppState>,
                ids: Vec<i32>,
            ) -> Result<(), crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::delete($table.filter($id_col.eq_any(ids))).execute(&mut conn)?;
                Ok(())
            }

            pub async fn generic_get_all(
                pool: actix_web::web::Data<crate::AppState>,
                query: $query_type,
            ) -> Result<(Vec<$resp>, i64, i64, Option<i32>), crate::errors::APIError> {
                use diesel::prelude::*;
                use crate::services::admin_db::AsAdminQuery;
                let mut conn = pool.db_pool.get()?;
                
                let mut data_query = $table.into_boxed();
                let mut count_query = $table.into_boxed();

                let admin_q = query.as_admin_query();

                if let Some(search_term) = &admin_q.search {
                    let pattern = format!("%{}%", search_term);
                    data_query = ($search_logic)(data_query, pattern.clone());
                    count_query = ($search_logic)(count_query, pattern);
                }

                let sort_by = admin_q.sort_by.as_deref().unwrap_or("created_at");
                let sort_order = admin_q.sort_order.as_deref().unwrap_or("desc");
                
                data_query = ($sort_logic)(data_query, sort_by, sort_order);

                let limit = admin_q.limit.unwrap_or(10);
                let page = admin_q.page.unwrap_or(1);
                
                let total = count_query.count().get_result::<i64>(&mut conn)?;
                let total_pages = (total as f64 / limit as f64).ceil() as i64;

                let offset = (page - 1) * limit;
                let list: Vec<$model> = data_query.limit(limit).offset(offset).load::<$model>(&mut conn)?;

                Ok((
                    list.into_iter().map(<$resp>::from).collect(),
                    total,
                    total_pages,
                    None,
                ))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_admin_entity_service_id {
    (
        $service_name:ident,
        $table:path,
        $model:ty,
        $resp:ty,
        $id_type:ty,
        $id_col:path,
        $query_type:ty,
        $search_logic:expr,
        $sort_logic:expr
    ) => {
        pub struct $service_name;

        impl $service_name {
            pub async fn generic_get_by_id(
                pool: actix_web::web::Data<crate::AppState>,
                id_val: $id_type,
            ) -> Result<$resp, crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                let item: $model = $table.filter($id_col.eq(id_val)).first(&mut conn)?;
                Ok(<$resp>::from(item))
            }

            pub async fn generic_create(
                pool: actix_web::web::Data<crate::AppState>,
                new_item: $model,
            ) -> Result<$resp, crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&new_item).execute(&mut conn)?;
                Ok(<$resp>::from(new_item))
            }

            pub async fn generic_bulk_create(
                pool: actix_web::web::Data<crate::AppState>,
                new_items: Vec<$model>,
            ) -> Result<Vec<$resp>, crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&new_items).execute(&mut conn)?;
                Ok(new_items.into_iter().map(<$resp>::from).collect())
            }

            pub async fn generic_update<U>(
                pool: actix_web::web::Data<crate::AppState>,
                id_val: $id_type,
                update_data: U,
            ) -> Result<$resp, crate::errors::APIError>
            where
                U: diesel::AsChangeset<Target = <$table as diesel::associations::HasTable>::Table>
                    + Send
                    + 'static,
                <$table as diesel::associations::HasTable>::Table:
                    diesel::query_builder::IntoUpdateTarget,
                <U as diesel::AsChangeset>::Changeset:
                    diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>,
                $id_type: Clone,
            {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::update($table.filter($id_col.eq(id_val.clone())))
                    .set(update_data)
                    .execute(&mut conn)?;
                let updated: $model = $table.filter($id_col.eq(id_val)).first(&mut conn)?;
                Ok(<$resp>::from(updated))
            }

            pub async fn generic_delete(
                pool: actix_web::web::Data<crate::AppState>,
                id_val: $id_type,
            ) -> Result<(), crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::delete($table.filter($id_col.eq(id_val))).execute(&mut conn)?;
                Ok(())
            }

            pub async fn generic_bulk_delete(
                pool: actix_web::web::Data<crate::AppState>,
                ids: Vec<$id_type>,
            ) -> Result<(), crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::delete($table.filter($id_col.eq_any(ids))).execute(&mut conn)?;
                Ok(())
            }

            pub async fn generic_get_all(
                pool: actix_web::web::Data<crate::AppState>,
                query: $query_type,
            ) -> Result<(Vec<$resp>, i64, i64, Option<$id_type>), crate::errors::APIError> {
                use diesel::prelude::*;
                use crate::services::admin_db::AsAdminQuery;
                let mut conn = pool.db_pool.get()?;

                let mut data_query = $table.into_boxed();
                let mut count_query = $table.into_boxed();

                let admin_q = query.as_admin_query();

                if let Some(search_term) = &admin_q.search {
                    let pattern = format!("%{}%", search_term);
                    data_query = ($search_logic)(data_query, pattern.clone());
                    count_query = ($search_logic)(count_query, pattern);
                }

                let sort_by = admin_q.sort_by.as_deref().unwrap_or("created_at");
                let sort_order = admin_q.sort_order.as_deref().unwrap_or("desc");

                data_query = ($sort_logic)(data_query, sort_by, sort_order);

                let limit = admin_q.limit.unwrap_or(10);
                let page = admin_q.page.unwrap_or(1);

                let total = count_query.count().get_result::<i64>(&mut conn)?;
                let total_pages = (total as f64 / limit as f64).ceil() as i64;

                let offset = (page - 1) * limit;
                let list: Vec<$model> =
                    data_query.limit(limit).offset(offset).load::<$model>(&mut conn)?;

                Ok((
                    list.into_iter().map(<$resp>::from).collect(),
                    total,
                    total_pages,
                    None,
                ))
            }
        }
    };
}

#[macro_export]
macro_rules! impl_admin_entity_service {
    ($service_name:ident, $table:path, $model:ty, $resp:ty, $id_col:path, $query_type:ty, $search_logic:expr, $sort_logic:expr) => {
        $crate::impl_admin_entity_service!($service_name, $table, $model, $resp, $id_col, id, $query_type, $search_logic, $sort_logic);
    };
    ($service_name:ident, $table:path, $model:ty, $resp:ty, $id_col:path, $id_field:ident, $query_type:ty, $search_logic:expr, $sort_logic:expr) => {
        pub struct $service_name;

        impl $service_name {
            pub async fn generic_get_by_id(
                pool: actix_web::web::Data<crate::AppState>,
                id_val: String,
            ) -> Result<$resp, crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                let item: $model = $table.filter($id_col.eq(id_val)).first(&mut conn)?;
                Ok(<$resp>::from(item))
            }

            pub async fn generic_create(
                pool: actix_web::web::Data<crate::AppState>,
                new_item: $model,
            ) -> Result<$resp, crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&new_item).execute(&mut conn)?;
                Ok(<$resp>::from(new_item))
            }

            pub async fn generic_bulk_create(
                pool: actix_web::web::Data<crate::AppState>,
                new_items: Vec<$model>,
            ) -> Result<Vec<$resp>, crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&new_items).execute(&mut conn)?;
                Ok(new_items.into_iter().map(<$resp>::from).collect())
            }

            pub async fn generic_update<U>(
                pool: actix_web::web::Data<crate::AppState>,
                id_val: String,
                update_data: U,
            ) -> Result<$resp, crate::errors::APIError>
            where
                U: diesel::AsChangeset<Target = <$table as diesel::associations::HasTable>::Table> + Send + 'static,
                <$table as diesel::associations::HasTable>::Table: diesel::query_builder::IntoUpdateTarget,
                <U as diesel::AsChangeset>::Changeset: diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>,
            {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::update($table.filter($id_col.eq(&id_val)))
                    .set(update_data)
                    .execute(&mut conn)?;
                let updated: $model = $table.filter($id_col.eq(id_val)).first(&mut conn)?;
                Ok(<$resp>::from(updated))
            }

            pub async fn generic_delete(
                pool: actix_web::web::Data<crate::AppState>,
                id_val: String,
            ) -> Result<(), crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::delete($table.filter($id_col.eq(id_val))).execute(&mut conn)?;
                Ok(())
            }

            pub async fn generic_bulk_delete(
                pool: actix_web::web::Data<crate::AppState>,
                ids: Vec<String>,
            ) -> Result<(), crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::delete($table.filter($id_col.eq_any(ids))).execute(&mut conn)?;
                Ok(())
            }

            pub async fn generic_bulk_update<U>(
                pool: actix_web::web::Data<crate::AppState>,
                req: crate::services::admin_db::BulkUpdateRequest<U>,
            ) -> Result<(), crate::errors::APIError>
            where
                U: diesel::AsChangeset<Target = <$table as diesel::associations::HasTable>::Table> + Send + 'static + schemars::JsonSchema + apistos::ApiComponent,
                <$table as diesel::associations::HasTable>::Table: diesel::query_builder::IntoUpdateTarget,
                <U as diesel::AsChangeset>::Changeset: diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>,
            {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                conn.transaction::<_, crate::errors::APIError, _>(|conn| {
                    for update in req.updates {
                        diesel::update($table.filter($id_col.eq(update.id)))
                            .set(update.data)
                            .execute(conn)?;
                    }
                    Ok(())
                })?;
                Ok(())
            }

            pub async fn generic_get_all(
                pool: actix_web::web::Data<crate::AppState>,
                query: $query_type,
            ) -> Result<(Vec<$resp>, i64, i64, Option<String>), crate::errors::APIError> {
                use diesel::prelude::*;
                use crate::services::admin_db::AsAdminQuery;
                let mut conn = pool.db_pool.get()?;
                
                let mut data_query = $table.into_boxed();
                let mut count_query = $table.into_boxed();

                let admin_q = query.as_admin_query();

                if let Some(search_term) = &admin_q.search {
                    let pattern = format!("%{}%", search_term);
                    data_query = ($search_logic)(data_query, pattern.clone());
                    count_query = ($search_logic)(count_query, pattern);
                }

                let sort_by = admin_q.sort_by.as_deref().unwrap_or("created_at");
                let sort_order = admin_q.sort_order.as_deref().unwrap_or("desc");
                
                if let Some(last_id_val) = admin_q.last_id.clone() {
                    if sort_order == "asc" {
                        data_query = data_query.filter($id_col.gt(last_id_val));
                    } else {
                        data_query = data_query.filter($id_col.lt(last_id_val));
                    }
                }

                data_query = ($sort_logic)(data_query, sort_by, sort_order);

                let limit = admin_q.limit.unwrap_or(10);
                let page = admin_q.page.unwrap_or(1);
                
                let total = count_query.count().get_result::<i64>(&mut conn)?;
                let total_pages = (total as f64 / limit as f64).ceil() as i64;

                let list: Vec<$model> = if admin_q.last_id.is_some() {
                    data_query.limit(limit).load::<$model>(&mut conn)?
                } else {
                    let offset = (page - 1) * limit;
                    data_query.limit(limit).offset(offset).load::<$model>(&mut conn)?
                };

                let next_last_id = list.last().map(|item| item.$id_field.clone());

                Ok((
                    list.into_iter().map(<$resp>::from).collect(),
                    total,
                    total_pages,
                    next_last_id,
                ))
            }
        }
    };
}

#[macro_export]
macro_rules! apply_admin_query {
    ($query:expr, $admin_query:expr, $search_logic:expr, $sort_logic:expr) => {{
        let mut q = $query;
        if let Some(search_term) = &$admin_query.search {
            let search_fn = $search_logic;
            q = search_fn(q, search_term);
        }
        
        let sort_by = $admin_query.sort_by.as_deref().unwrap_or("created_at");
        let sort_order = $admin_query.sort_order.as_deref().unwrap_or("desc");
        
        let sort_fn = $sort_logic;
        q = sort_fn(q, sort_by, sort_order);
        q
    }};
}
