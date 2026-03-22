use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::database::enums::GradingSchemeType;

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct GradingSchemeQuery {
    pub search: Option<String>,
    pub grade_level_id: Option<String>,
    pub scheme_type: Option<GradingSchemeType>,
    pub is_default: Option<bool>,
    pub last_id: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for GradingSchemeQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateGradingSchemeRequest {
    pub name: String,
    pub scheme_type: GradingSchemeType,
    pub grade_level_id: Option<String>,
    pub scale_definition: String,
    pub pass_mark: Option<i32>,
    pub is_default: Option<bool>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::grading_schemes)]
pub struct UpdateGradingSchemeRequest {
    pub name: Option<String>,
    pub scheme_type: Option<GradingSchemeType>,
    pub grade_level_id: Option<String>,
    pub scale_definition: Option<String>,
    pub pass_mark: Option<i32>,
    pub is_default: Option<bool>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Queryable, Selectable, Insertable, Serialize, Deserialize, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::grading_schemes)]
pub struct GradingScheme {
    pub id: String,
    pub name: String,
    pub scheme_type: GradingSchemeType,
    pub grade_level_id: Option<String>,
    pub scale_definition: String,
    pub pass_mark: Option<i32>,
    pub is_default: bool,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<CreateGradingSchemeRequest> for GradingScheme {
    fn from(req: CreateGradingSchemeRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: req.name,
            scheme_type: req.scheme_type,
            grade_level_id: req.grade_level_id,
            scale_definition: req.scale_definition,
            pass_mark: req.pass_mark,
            is_default: req.is_default.unwrap_or(false),
            description: req.description,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
