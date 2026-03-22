use crate::schema::seeds;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;
use crate::services::admin_db::{AdminQuery, AsAdminQuery};

#[derive(Queryable, Selectable, Insertable, AsChangeset, Serialize, Deserialize, Clone, ApiComponent, JsonSchema)]
#[diesel(table_name = seeds)]
pub struct Seed {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct SeedResponse {
    pub id: String,
    pub table_name: String,
    pub record_id: String,
    pub created_at: NaiveDateTime,
}

impl From<Seed> for SeedResponse {
    fn from(s: Seed) -> Self {
        Self {
            id: s.id,
            table_name: s.table_name,
            record_id: s.record_id,
            created_at: s.created_at,
        }
    }
}

impl From<CreateSeedRequest> for Seed {
    fn from(req: CreateSeedRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            table_name: req.table_name,
            record_id: req.record_id,
            created_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateSeedRequest {
    pub table_name: String,
    pub record_id: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = seeds)]
pub struct UpdateSeedRequest {
    pub table_name: Option<String>,
    pub record_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct SeedQuery {
    pub table_name: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for SeedQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}
