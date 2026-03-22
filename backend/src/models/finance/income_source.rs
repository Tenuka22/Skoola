use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;
use crate::services::admin_db::{AdminQuery, AsAdminQuery};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::income_sources)]
pub struct IncomeSource {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::income_sources)]
pub struct CreateIncomeSourceRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::income_sources)]
pub struct UpdateIncomeSourceRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct IncomeSourceQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for IncomeSourceQuery {
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

impl From<CreateIncomeSourceRequest> for IncomeSource {
    fn from(req: CreateIncomeSourceRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: req.name,
            description: req.description,
            created_at: now,
            updated_at: now,
        }
    }
}
