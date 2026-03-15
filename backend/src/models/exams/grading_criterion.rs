use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateGradingCriterionRequest {
    pub scheme_id: String,
    pub grade: String,
    pub min_mark: i32,
    pub max_mark: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, JsonSchema, ApiComponent, AsChangeset)]
#[diesel(table_name = crate::schema::grading_criteria)]
pub struct GradingCriterion {
    pub id: String,
    pub scheme_id: String,
    pub grade: String,
    pub min_mark: i32,
    pub max_mark: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::grading_criteria)]
pub struct NewGradingCriterion {
    pub id: String,
    pub scheme_id: String,
    pub grade: String,
    pub min_mark: i32,
    pub max_mark: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::grading_criteria)]
pub struct UpdateGradingCriterionRequest {
    pub scheme_id: Option<String>,
    pub grade: Option<String>,
    pub min_mark: Option<i32>,
    pub max_mark: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct GradingCriterionQuery {
    pub scheme_id: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for GradingCriterionQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct GradingCriterionResponse {
    pub id: String,
    pub scheme_id: String,
    pub grade: String,
    pub min_mark: i32,
    pub max_mark: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<GradingCriterion> for GradingCriterionResponse {
    fn from(gc: GradingCriterion) -> Self {
        GradingCriterionResponse {
            id: gc.id,
            scheme_id: gc.scheme_id,
            grade: gc.grade,
            min_mark: gc.min_mark,
            max_mark: gc.max_mark,
            created_at: gc.created_at,
            updated_at: gc.updated_at,
        }
    }
}
