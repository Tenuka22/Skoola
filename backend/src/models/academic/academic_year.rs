use crate::schema::academic_years;
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
#[diesel(table_name = academic_years)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AcademicYear {
    pub id: String,
    pub year_start: i32,
    pub year_end: i32,
    pub name: String,
    pub current: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset, JsonSchema, ApiComponent,
)]
#[diesel(table_name = academic_years)]
pub struct CreateAcademicYearRequest {
    pub id: String,
    pub year_start: i32,
    pub year_end: i32,
    pub name: String,
    pub current: Option<bool>,
}

impl From<CreateAcademicYearRequest> for AcademicYear {
    fn from(req: CreateAcademicYearRequest) -> Self {
        AcademicYear {
            id: req.id,
            year_start: req.year_start,
            year_end: req.year_end,
            name: req.name,
            current: req.current.unwrap_or(false),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = academic_years)]
pub struct UpdateAcademicYearRequest {
    pub year_start: Option<i32>,
    pub year_end: Option<i32>,
    pub name: Option<String>,
    pub current: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct AcademicYearQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for AcademicYearQuery {
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
pub struct AcademicYearResponse {
    pub id: String,
    pub year_start: i32,
    pub year_end: i32,
    pub name: String,
    pub current: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<AcademicYear> for AcademicYearResponse {
    fn from(academic_year: AcademicYear) -> Self {
        AcademicYearResponse {
            id: academic_year.id,
            year_start: academic_year.year_start,
            year_end: academic_year.year_end,
            name: academic_year.name,
            current: academic_year.current,
            created_at: academic_year.created_at,
            updated_at: academic_year.updated_at,
        }
    }
}
