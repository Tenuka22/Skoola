use crate::database::enums::EducationLevel;
use crate::schema::grade_levels;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = grade_levels)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct GradeLevel {
    pub id: String,
    pub grade_number: i32,
    pub grade_name: String,
    pub education_level: EducationLevel,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset, JsonSchema, ApiComponent,
)]
#[diesel(table_name = grade_levels)]
pub struct CreateGradeLevelRequest {
    pub id: String,
    pub grade_number: i32,
    pub grade_name: String,
    pub education_level: EducationLevel,
}

impl From<CreateGradeLevelRequest> for GradeLevel {
    fn from(req: CreateGradeLevelRequest) -> Self {
        GradeLevel {
            id: req.id,
            grade_number: req.grade_number,
            grade_name: req.grade_name,
            education_level: req.education_level,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = grade_levels)]
pub struct UpdateGradeLevelRequest {
    pub grade_number: Option<i32>,
    pub grade_name: Option<String>,
    pub education_level: Option<EducationLevel>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct GradeLevelQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for GradeLevelQuery {
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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct GradeLevelResponse {
    pub id: String,
    pub grade_number: i32,
    pub grade_name: String,
    pub education_level: EducationLevel,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<GradeLevel> for GradeLevelResponse {
    fn from(grade_level: GradeLevel) -> Self {
        GradeLevelResponse {
            id: grade_level.id,
            grade_number: grade_level.grade_number,
            grade_name: grade_level.grade_name,
            education_level: grade_level.education_level,
            created_at: grade_level.created_at,
            updated_at: grade_level.updated_at,
        }
    }
}
