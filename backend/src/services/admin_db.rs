use crate::AppState;
use crate::errors::APIError;
use actix_web::web;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::sqlite::SqliteConnection;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;
use std::collections::HashMap;

/// Advanced query parameters for filtering, sorting, pagination, and joins
#[derive(Debug, Deserialize, Clone, Default)]
#[derive(JsonSchema, ApiComponent)]
pub struct AdminQuery {
    /// Full-text search across all searchable fields
    pub search: Option<String>,
    
    /// Field to sort by (e.g., "name", "created_at")
    pub sort_by: Option<String>,
    
    /// Sort order: "asc" or "desc"
    pub sort_order: Option<String>,
    
    /// Page number (1-indexed)
    pub page: Option<i64>,
    
    /// Number of items per page
    pub limit: Option<i64>,
    
    /// Cursor-based pagination: last seen ID
    pub last_id: Option<String>,
    
    /// Field-specific filters (e.g., {"status": "active", "grade": "10"..Default::default()})
    #[serde(default)]
    pub filters: HashMap<String, String>,
    
    /// Range filters for numeric/date fields (e.g., {"age": {"gte": 18, "lte": 65}})
    #[serde(default)]
    pub range_filters: HashMap<String, RangeFilter>,
    
    /// IN-clause filters (e.g., {"status": ["active", "pending"]})
    #[serde(default)]
    pub in_filters: HashMap<String, Vec<String>>,
    
    /// Fields to include in response (projection)
    pub select: Option<String>,
    
    /// Fields to exclude from response
    pub exclude: Option<String>,
    
    /// Related tables to join (comma-separated)
    pub join: Option<String>,
    
    /// Custom ORDER BY clause (raw SQL for complex sorting)
    pub order_raw: Option<String>,
    
    /// Custom WHERE clause (raw SQL for complex filtering - use with caution)
    pub where_raw: Option<String>,
    
    /// Group by fields for aggregations
    pub group_by: Option<String>,
    
    /// Aggregate functions to apply (e.g., "COUNT(*) as count, SUM(amount) as total")
    pub aggregate: Option<String>,
    
    /// Having clause for aggregated results
    pub having: Option<String>,
    
    /// Distinct flag
    pub distinct: Option<bool>,
    
    /// Soft delete handling - include deleted records
    pub include_deleted: Option<bool>,
}

/// Range filter for numeric or date comparisons
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[derive(JsonSchema, ApiComponent)]
pub struct RangeFilter {
    /// Greater than or equal
    pub gte: Option<String>,
    /// Less than or equal
    pub lte: Option<String>,
    /// Greater than
    pub gt: Option<String>,
    /// Less than
    pub lt: Option<String>,
}

pub trait AsAdminQuery {
    fn as_admin_query(&self) -> AdminQuery;
}

impl AsAdminQuery for AdminQuery {
    fn as_admin_query(&self) -> AdminQuery {
        self.clone()
    }
}

/// Paginated response wrapper
#[derive(Debug, Serialize, Deserialize, Clone)]
#[derive(ApiComponent, JsonSchema)]
pub struct PaginatedResponse<T: JsonSchema + ApiComponent> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub next_last_id: Option<String>,
    pub has_next: bool,
    pub has_prev: bool,
}

/// Bulk ID request for batch operations
#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(JsonSchema, ApiComponent)]
pub struct BulkIdRequest<ID: JsonSchema + ApiComponent = String> {
    pub ids: Vec<ID>,
}

/// Bulk ID request for batch operations with i32 IDs
pub type BulkIdRequestI32 = BulkIdRequest<i32>;

/// Bulk update request with entity updates
#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(JsonSchema, ApiComponent)]
pub struct BulkUpdateRequest<T: JsonSchema + ApiComponent, ID: JsonSchema + ApiComponent = String> {
    pub updates: Vec<EntityUpdate<T, ID>>,
}

/// Bulk update request with i32 IDs
pub type BulkUpdateRequestI32<T> = BulkUpdateRequest<T, i32>;

/// Bulk create request
#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(JsonSchema, ApiComponent)]
pub struct BulkCreateRequest<T: JsonSchema + ApiComponent> {
    pub items: Vec<T>,
}

/// Single entity update
#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(JsonSchema, ApiComponent)]
pub struct EntityUpdate<T: JsonSchema + ApiComponent, ID: JsonSchema + ApiComponent = String> {
    pub id: ID,
    pub data: T,
}

/// Single entity update with i32 ID
pub type EntityUpdateI32<T> = EntityUpdate<T, i32>;

/// Join configuration for related tables
#[derive(Debug, Deserialize, Serialize, Clone)]
#[derive(JsonSchema, ApiComponent)]
pub struct JoinConfig {
    /// Table name to join
    pub table: String,
    /// Join type: "inner", "left", "right"
    pub join_type: String,
    /// Join condition (ON clause)
    pub on: String,
    /// Fields to select from joined table
    pub select: Option<String>,
}

/// Advanced query builder for complex queries
#[derive(Debug, Clone)]
pub struct AdvancedQuery {
    pub filters: Vec<String>,
    pub params: Vec<serde_json::Value>,
    pub joins: Vec<JoinConfig>,
    pub order_by: Vec<String>,
    pub group_by: Vec<String>,
    pub select: Option<String>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

impl Default for AdvancedQuery {
    fn default() -> Self {
        Self {
            filters: Vec::new(),
            params: Vec::new(),
            joins: Vec::new(),
            order_by: Vec::new(),
            group_by: Vec::new(),
            select: None,
            limit: None,
            offset: None,
        }
    }
}

impl AdvancedQuery {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn with_filter(mut self, condition: String, param: serde_json::Value) -> Self {
        self.filters.push(condition);
        self.params.push(param);
        self
    }
    
    pub fn with_join(mut self, join: JoinConfig) -> Self {
        self.joins.push(join);
        self
    }
    
    pub fn with_order(mut self, order: String) -> Self {
        self.order_by.push(order);
        self
    }
    
    pub fn with_limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    
    pub fn with_offset(mut self, offset: i64) -> Self {
        self.offset = Some(offset);
        self
    }
}

pub struct AdminEntityDbActions;

impl AdminEntityDbActions {
    pub fn get_conn(pool: &web::Data<AppState>) -> Result<PooledConnection<ConnectionManager<SqliteConnection>>, APIError> {
        pool.db_pool.get().map_err(APIError::from)
    }
}

/// Filter operators for dynamic filtering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterOperator {
    Eq,
    Ne,
    Lt,
    Lte,
    Gt,
    Gte,
    Like,
    Ilike,
    In,
    NotIn,
    IsNull,
    IsNotNull,
    Between,
}

impl FilterOperator {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "ne" | "!=" => FilterOperator::Ne,
            "lt" | "<" => FilterOperator::Lt,
            "lte" | "<=" => FilterOperator::Lte,
            "gt" | ">" => FilterOperator::Gt,
            "gte" | ">=" => FilterOperator::Gte,
            "like" => FilterOperator::Like,
            "ilike" => FilterOperator::Ilike,
            "in" => FilterOperator::In,
            "nin" | "not_in" => FilterOperator::NotIn,
            "null" | "is_null" => FilterOperator::IsNull,
            "notnull" | "is_not_null" => FilterOperator::IsNotNull,
            "between" => FilterOperator::Between,
            _ => FilterOperator::Eq,
        }
    }
    
    pub fn to_sql(&self) -> &'static str {
        match self {
            FilterOperator::Eq => "=",
            FilterOperator::Ne => "!=",
            FilterOperator::Lt => "<",
            FilterOperator::Lte => "<=",
            FilterOperator::Gt => ">",
            FilterOperator::Gte => ">=",
            FilterOperator::Like => "LIKE",
            FilterOperator::Ilike => "LIKE",
            FilterOperator::In => "IN",
            FilterOperator::NotIn => "NOT IN",
            FilterOperator::IsNull => "IS NULL",
            FilterOperator::IsNotNull => "IS NOT NULL",
            FilterOperator::Between => "BETWEEN",
        }
    }
}

/// Parse filter string in format "field:operator:value" or just "field:value"
pub fn parse_filter(filter_str: &str) -> Option<(String, FilterOperator, String)> {
    let parts: Vec<&str> = filter_str.splitn(3, ':').collect();
    
    if parts.len() == 3 {
        Some((
            parts[0].to_string(),
            FilterOperator::from_str(parts[1]),
            parts[2].to_string(),
        ))
    } else if parts.len() == 2 {
        Some((
            parts[0].to_string(),
            FilterOperator::Eq,
            parts[1].to_string(),
        ))
    } else if parts.len() == 1 && !parts[0].is_empty() {
        Some((parts[0].to_string(), FilterOperator::Eq, String::new()))
    } else {
        None
    }
}

// ============================================================================
// Advanced Service Macros with Full CRUD, Filtering, Sorting, Joins Support
// ============================================================================

/// Main macro for implementing admin entity services with String ID
/// Supports: filtering, sorting, pagination, joins, field selection, aggregations
/// 
/// This macro has two variants:
/// 1. With explicit id_field: ($service_name, $table, $model, $resp, $id_col, $id_field, $query_type, $search_logic, $sort_logic)
/// 2. Without id_field (defaults to "id"): ($service_name, $table, $model, $resp, $id_col, $query_type, $search_logic, $sort_logic)
#[macro_export]
macro_rules! impl_admin_entity_service {
    // Variant with explicit id_field
    (
        $service_name:ident,
        $table:path,
        $model:ty,
        $resp:ty,
        $id_col:path,
        $id_field:ident,
        $query_type:ty,
        $search_logic:expr,
        $sort_logic:expr
    ) => {
        pub struct $service_name;

        impl $service_name {
            /// Get single entity by ID
            pub async fn generic_get_by_id(
                pool: actix_web::web::Data<crate::AppState>,
                id_val: String,
            ) -> Result<$resp, crate::errors::APIError>
            where
                $resp: From<$model>,
            {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                let item: $model = $table.filter($id_col.eq(id_val)).first(&mut conn)?;
                Ok(<$resp>::from(item))
            }

            /// Create new entity
            pub async fn generic_create<C>(
                pool: actix_web::web::Data<crate::AppState>,
                new_item: C,
            ) -> Result<$resp, crate::errors::APIError>
            where
                $model: From<C>,
                $resp: From<$model>,
                C: Send + 'static,
            {
                use diesel::prelude::*;
                let model_item = <$model>::from(new_item);
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&model_item).execute(&mut conn)?;
                Ok(<$resp>::from(model_item))
            }

            /// Bulk create multiple entities
            pub async fn generic_bulk_create<C>(
                pool: actix_web::web::Data<crate::AppState>,
                new_items: Vec<C>,
            ) -> Result<Vec<$resp>, crate::errors::APIError>
            where
                $model: From<C>,
                $resp: From<$model>,
                C: Send + 'static,
            {
                use diesel::prelude::*;
                let model_items: Vec<$model> = new_items.into_iter().map(<$model>::from).collect();
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&model_items).execute(&mut conn)?;
                Ok(model_items.into_iter().map(<$resp>::from).collect())
            }

            /// Update single entity by ID
            pub async fn generic_update<U>(
                pool: actix_web::web::Data<crate::AppState>,
                id_val: String,
                update_data: U,
            ) -> Result<$resp, crate::errors::APIError>
            where
                U: diesel::AsChangeset<Target = <$table as diesel::associations::HasTable>::Table> + Send + 'static,
                <$table as diesel::associations::HasTable>::Table: diesel::query_builder::IntoUpdateTarget,
                <U as diesel::AsChangeset>::Changeset: diesel::query_builder::QueryFragment<diesel::sqlite::Sqlite>,
                $resp: From<$model>,
            {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::update($table.filter($id_col.eq(&id_val)))
                    .set(update_data)
                    .execute(&mut conn)?;
                let updated: $model = $table.filter($id_col.eq(id_val)).first(&mut conn)?;
                Ok(<$resp>::from(updated))
            }

            /// Delete single entity by ID
            pub async fn generic_delete(
                pool: actix_web::web::Data<crate::AppState>,
                id_val: String,
            ) -> Result<(), crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::delete($table.filter($id_col.eq(id_val))).execute(&mut conn)?;
                Ok(())
            }

            /// Bulk delete multiple entities by IDs
            pub async fn generic_bulk_delete(
                pool: actix_web::web::Data<crate::AppState>,
                ids: Vec<String>,
            ) -> Result<(), crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::delete($table.filter($id_col.eq_any(ids))).execute(&mut conn)?;
                Ok(())
            }

            /// Bulk update multiple entities
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

            /// Get all entities with advanced filtering, sorting, pagination
            pub async fn generic_get_all(
                pool: actix_web::web::Data<crate::AppState>,
                query: $query_type,
            ) -> Result<(Vec<$resp>, i64, i64, Option<String>), crate::errors::APIError>
            where
                $resp: From<$model>,
            {
                use diesel::prelude::*;
                use crate::services::admin_db::AsAdminQuery;
                let mut conn = pool.db_pool.get()?;

                let mut data_query = $table.into_boxed();
                let mut count_query = $table.into_boxed();

                let admin_q = query.as_admin_query();

                // Apply search logic
                if let Some(search_term) = &admin_q.search {
                    let pattern = format!("%{}%", search_term);
                    data_query = ($search_logic)(data_query, pattern.clone());
                    count_query = ($search_logic)(count_query, pattern);
                }

                // Apply field filters
                for (field, value) in &admin_q.filters {
                    match field.as_str() {
                        "status" | "type" | "category" | "name" => {
                            data_query = data_query.filter(
                                diesel::dsl::sql::<diesel::sql_types::Bool>(
                                    &format!("{} = ?", field)
                                ).bind::<diesel::sql_types::Text, _>(value.clone())
                            );
                            count_query = count_query.filter(
                                diesel::dsl::sql::<diesel::sql_types::Bool>(
                                    &format!("{} = ?", field)
                                ).bind::<diesel::sql_types::Text, _>(value.clone())
                            );
                        }
                        _ => {
                            // Generic text comparison
                            data_query = data_query.filter(
                                diesel::dsl::sql::<diesel::sql_types::Bool>(
                                    &format!("LOWER(CAST({} AS TEXT)) = LOWER(?)", field)
                                ).bind::<diesel::sql_types::Text, _>(value.to_lowercase())
                            );
                        }
                    }
                }

                // Apply range filters
                for (field, range) in &admin_q.range_filters {
                    if let Some(gte) = &range.gte {
                        data_query = data_query.filter(
                            diesel::dsl::sql::<diesel::sql_types::Bool>(
                                &format!("{} >= ?", field)
                            ).bind::<diesel::sql_types::Text, _>(gte.clone())
                        );
                        count_query = count_query.filter(
                            diesel::dsl::sql::<diesel::sql_types::Bool>(
                                &format!("{} >= ?", field)
                            ).bind::<diesel::sql_types::Text, _>(gte.clone())
                        );
                    }
                    if let Some(lte) = &range.lte {
                        data_query = data_query.filter(
                            diesel::dsl::sql::<diesel::sql_types::Bool>(
                                &format!("{} <= ?", field)
                            ).bind::<diesel::sql_types::Text, _>(lte.clone())
                        );
                        count_query = count_query.filter(
                            diesel::dsl::sql::<diesel::sql_types::Bool>(
                                &format!("{} <= ?", field)
                            ).bind::<diesel::sql_types::Text, _>(lte.clone())
                        );
                    }
                    if let Some(gt) = &range.gt {
                        data_query = data_query.filter(
                            diesel::dsl::sql::<diesel::sql_types::Bool>(
                                &format!("{} > ?", field)
                            ).bind::<diesel::sql_types::Text, _>(gt.clone())
                        );
                    }
                    if let Some(lt) = &range.lt {
                        data_query = data_query.filter(
                            diesel::dsl::sql::<diesel::sql_types::Bool>(
                                &format!("{} < ?", field)
                            ).bind::<diesel::sql_types::Text, _>(lt.clone())
                        );
                    }
                }

                // Apply IN filters
                for (field, values) in &admin_q.in_filters {
                    if !values.is_empty() {
                        // Escape single quotes in values to prevent SQL injection
                        let escaped_values: Vec<String> = values.iter()
                            .map(|v| format!("'{}'", v.replace('\'', "''")))
                            .collect();
                        let clause = format!("{} IN ({})", field, escaped_values.join(", "));
                        data_query = data_query.filter(diesel::dsl::sql::<diesel::sql_types::Bool>(&clause));
                        count_query = count_query.filter(diesel::dsl::sql::<diesel::sql_types::Bool>(&clause));
                    }
                }

                // Apply sorting
                let sort_by = admin_q.sort_by.as_deref().unwrap_or("created_at");
                let sort_order = admin_q.sort_order.as_deref().unwrap_or("desc");
                data_query = ($sort_logic)(data_query, sort_by, sort_order);

                // Apply cursor-based pagination
                if let Some(last_id_val) = admin_q.last_id.clone() {
                    if sort_order == "asc" {
                        data_query = data_query.filter($id_col.gt(last_id_val));
                    } else {
                        data_query = data_query.filter($id_col.lt(last_id_val));
                    }
                }

                // Apply offset-based pagination
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

    // Variant without explicit id_field - defaults to "id"
    (
        $service_name:ident,
        $table:path,
        $model:ty,
        $resp:ty,
        $id_col:path,
        $query_type:ty,
        $search_logic:expr,
        $sort_logic:expr
    ) => {
        $crate::impl_admin_entity_service!(
            $service_name,
            $table,
            $model,
            $resp,
            $id_col,
            id,
            $query_type,
            $search_logic,
            $sort_logic
        );
    };
}

/// Macro for i32 ID type - standalone implementation
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

            pub async fn generic_create<C>(
                pool: actix_web::web::Data<crate::AppState>,
                new_item: C,
            ) -> Result<$resp, crate::errors::APIError> 
            where
                $model: From<C>,
                $resp: From<$model>,
                C: Send + 'static,
            {
                use diesel::prelude::*;
                let model_item = <$model>::from(new_item);
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&model_item).execute(&mut conn)?;
                Ok(<$resp>::from(model_item))
            }

            pub async fn generic_bulk_create<C>(
                pool: actix_web::web::Data<crate::AppState>,
                new_items: Vec<C>,
            ) -> Result<Vec<$resp>, crate::errors::APIError>
            where
                $model: From<C>,
                $resp: From<$model>,
                C: Send + 'static,
            {
                use diesel::prelude::*;
                let model_items: Vec<$model> = new_items.into_iter().map(|c| <$model>::from(c)).collect();
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&model_items).execute(&mut conn)?;
                Ok(model_items.into_iter().map(|m| <$resp>::from(m)).collect())
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

                for (field, value) in &admin_q.filters {
                    data_query = data_query.filter(
                        diesel::dsl::sql::<diesel::sql_types::Bool>(
                            &format!("LOWER(CAST({} AS TEXT)) = LOWER(?)", field)
                        ).bind::<diesel::sql_types::Text, _>(value.to_lowercase())
                    );
                }

                for (field, range) in &admin_q.range_filters {
                    if let Some(gte) = &range.gte {
                        data_query = data_query.filter(
                            diesel::dsl::sql::<diesel::sql_types::Bool>(
                                &format!("{} >= ?", field)
                            ).bind::<diesel::sql_types::Text, _>(gte.clone())
                        );
                        count_query = count_query.filter(
                            diesel::dsl::sql::<diesel::sql_types::Bool>(
                                &format!("{} >= ?", field)
                            ).bind::<diesel::sql_types::Text, _>(gte.clone())
                        );
                    }
                    if let Some(lte) = &range.lte {
                        data_query = data_query.filter(
                            diesel::dsl::sql::<diesel::sql_types::Bool>(
                                &format!("{} <= ?", field)
                            ).bind::<diesel::sql_types::Text, _>(lte.clone())
                        );
                        count_query = count_query.filter(
                            diesel::dsl::sql::<diesel::sql_types::Bool>(
                                &format!("{} <= ?", field)
                            ).bind::<diesel::sql_types::Text, _>(lte.clone())
                        );
                    }
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

/// Macro for custom ID types (NaiveDate, UUID, etc.)
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

            pub async fn generic_create<C>(
                pool: actix_web::web::Data<crate::AppState>,
                new_item: C,
            ) -> Result<$resp, crate::errors::APIError> 
            where
                $model: From<C>,
                $resp: From<$model>,
                C: Send + 'static,
            {
                use diesel::prelude::*;
                let model_item = <$model>::from(new_item);
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&model_item).execute(&mut conn)?;
                Ok(<$resp>::from(model_item))
            }

            pub async fn generic_bulk_create<C>(
                pool: actix_web::web::Data<crate::AppState>,
                new_items: Vec<C>,
            ) -> Result<Vec<$resp>, crate::errors::APIError>
            where
                $model: From<C>,
                $resp: From<$model>,
                C: Send + 'static,
            {
                use diesel::prelude::*;
                let model_items: Vec<$model> = new_items.into_iter().map(|c| <$model>::from(c)).collect();
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&model_items).execute(&mut conn)?;
                Ok(model_items.into_iter().map(|m| <$resp>::from(m)).collect())
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

                // Apply filters
                for (field, value) in &admin_q.filters {
                    data_query = data_query.filter(
                        diesel::dsl::sql::<diesel::sql_types::Bool>(
                            &format!("LOWER(CAST({} AS TEXT)) = LOWER(?)", field)
                        ).bind::<diesel::sql_types::Text, _>(value.to_lowercase())
                    );
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

/// Macro for composite key entities (many-to-many join tables)
#[macro_export]
macro_rules! impl_admin_entity_service_composite {
    (
        $service_name:ident,
        $table:path,
        $model:ty,
        $resp:ty,
        $( $key_col:ident ),+,
        $query_type:ty,
        $search_logic:expr,
        $sort_logic:expr
    ) => {
        pub struct $service_name;

        impl $service_name {
            pub async fn generic_create<C>(
                pool: actix_web::web::Data<crate::AppState>,
                new_item: C,
            ) -> Result<$resp, crate::errors::APIError> 
            where
                $model: From<C>,
                $resp: From<$model>,
                C: Send + 'static,
            {
                use diesel::prelude::*;
                let model_item = <$model>::from(new_item);
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&model_item).execute(&mut conn)?;
                Ok(<$resp>::from(model_item))
            }

            pub async fn generic_bulk_create<C>(
                pool: actix_web::web::Data<crate::AppState>,
                new_items: Vec<C>,
            ) -> Result<Vec<$resp>, crate::errors::APIError>
            where
                $model: From<C>,
                $resp: From<$model>,
                C: Send + 'static,
            {
                use diesel::prelude::*;
                let model_items: Vec<$model> = new_items.into_iter().map(|c| <$model>::from(c)).collect();
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&model_items).execute(&mut conn)?;
                Ok(model_items.into_iter().map(|m| <$resp>::from(m)).collect())
            }

            pub async fn generic_delete(
                pool: actix_web::web::Data<crate::AppState>,
                $( $key_col: String ),+
            ) -> Result<(), crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                diesel::delete(
                    $table
                    $( .filter($table.col($key_col).eq($key_col)) )+
                ).execute(&mut conn)?;
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

                for (field, value) in &admin_q.filters {
                    data_query = data_query.filter(
                        diesel::dsl::sql::<diesel::sql_types::Bool>(
                            &format!("{} = ?", field)
                        ).bind::<diesel::sql_types::Text, _>(value.clone())
                    );
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

/// Macro for entities with join support
#[macro_export]
macro_rules! impl_admin_entity_service_with_joins {
    (
        $service_name:ident,
        $table:path,
        $model:ty,
        $resp:ty,
        $id_col:path,
        $id_field:ident,
        $query_type:ty,
        $search_logic:expr,
        $sort_logic:expr,
        joins => {
            $( $join_name:ident => {
                table: $join_table:path,
                on: $join_condition:expr,
                join_type: $join_type:tt
            } ),* $(,)?
        }
    ) => {
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

            pub async fn generic_create<C>(
                pool: actix_web::web::Data<crate::AppState>,
                new_item: C,
            ) -> Result<$resp, crate::errors::APIError>
            where
                $model: From<C>,
                $resp: From<$model>,
                C: Send + 'static,
            {
                use diesel::prelude::*;
                let model_item = <$model>::from(new_item);
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&model_item).execute(&mut conn)?;
                Ok(<$resp>::from(model_item))
            }

            pub async fn generic_bulk_create<C>(
                pool: actix_web::web::Data<crate::AppState>,
                new_items: Vec<C>,
            ) -> Result<Vec<$resp>, crate::errors::APIError>
            where
                $model: From<C>,
                $resp: From<$model>,
                C: Send + 'static,
            {
                use diesel::prelude::*;
                let model_items: Vec<$model> = new_items.into_iter().map(<$model>::from).collect();
                let mut conn = pool.db_pool.get()?;
                diesel::insert_into($table).values(&model_items).execute(&mut conn)?;
                Ok(model_items.into_iter().map(<$resp>::from).collect())
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

                for (field, value) in &admin_q.filters {
                    data_query = data_query.filter(
                        diesel::dsl::sql::<diesel::sql_types::Bool>(
                            &format!("{} = ?", field)
                        ).bind::<diesel::sql_types::Text, _>(value.clone())
                    );
                }

                // Apply range filters for foreign key searches
                for (field, range) in &admin_q.range_filters {
                    if let Some(gte) = &range.gte {
                        data_query = data_query.filter(
                            diesel::dsl::sql::<diesel::sql_types::Bool>(
                                &format!("{} >= ?", field)
                            ).bind::<diesel::sql_types::Text, _>(gte.clone())
                        );
                    }
                    if let Some(lte) = &range.lte {
                        data_query = data_query.filter(
                            diesel::dsl::sql::<diesel::sql_types::Bool>(
                                &format!("{} <= ?", field)
                            ).bind::<diesel::sql_types::Text, _>(lte.clone())
                        );
                    }
                }

                let sort_by = admin_q.sort_by.as_deref().unwrap_or("created_at");
                let sort_order = admin_q.sort_order.as_deref().unwrap_or("desc");
                data_query = ($sort_logic)(data_query, sort_by, sort_order);

                if let Some(last_id_val) = admin_q.last_id.clone() {
                    if sort_order == "asc" {
                        data_query = data_query.filter($id_col.gt(last_id_val));
                    } else {
                        data_query = data_query.filter($id_col.lt(last_id_val));
                    }
                }

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

/// Helper macro for applying query filters dynamically
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

/// Macro for foreign key search with join
#[macro_export]
macro_rules! impl_fk_search {
    (
        $service_name:ident,
        $table:path,
        $model:ty,
        $fk_field:ident,
        $fk_table:path,
        $fk_display_field:ident
    ) => {
        impl $service_name {
            pub async fn search_by_fk(
                pool: actix_web::web::Data<crate::AppState>,
                fk_value: String,
            ) -> Result<Vec<$resp>, crate::errors::APIError> {
                use diesel::prelude::*;
                let mut conn = pool.db_pool.get()?;
                
                let results: Vec<$model> = $table
                    .inner_join($fk_table.on($table::dsl::$fk_field.eq($fk_table::dsl::id)))
                    .filter($fk_table::dsl::$fk_display_field.like(format!("%{}%", fk_value)))
                    .load::<$model>(&mut conn)?;
                
                Ok(results.into_iter().map(<$resp>::from).collect())
            }
        }
    };
}
