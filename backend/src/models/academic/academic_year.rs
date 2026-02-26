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

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = academic_years)]
pub struct UpdateAcademicYearRequest {
    pub year_start: Option<i32>,
    pub year_end: Option<i32>,
    pub name: Option<String>,
    pub current: Option<bool>,
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
