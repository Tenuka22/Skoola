use crate::schema::{
    al_stream_grade_levels, al_stream_optional_groups, al_stream_optional_subjects,
    al_stream_required_subjects, al_streams, grade_subjects,
};
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
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
#[diesel(table_name = al_streams)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AlStream {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

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
#[diesel(table_name = al_stream_grade_levels)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(stream_id, grade_level_id))]
pub struct AlStreamGradeLevel {
    pub stream_id: String,
    pub grade_level_id: String,
    pub created_at: NaiveDateTime,
}

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
#[diesel(table_name = grade_subjects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(grade_id, subject_id))]
pub struct GradeSubject {
    pub grade_id: String,
    pub subject_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

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
#[diesel(table_name = al_stream_required_subjects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(stream_id, subject_id))]
pub struct AlStreamRequiredSubject {
    pub stream_id: String,
    pub subject_id: String,
    pub created_at: NaiveDateTime,
}

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
#[diesel(table_name = al_stream_optional_groups)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AlStreamOptionalGroup {
    pub id: String,
    pub stream_id: String,
    pub group_name: String,
    pub min_select: i32,
    pub max_select: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

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
#[diesel(table_name = al_stream_optional_subjects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(group_id, subject_id))]
pub struct AlStreamOptionalSubject {
    pub group_id: String,
    pub subject_id: String,
    pub created_at: NaiveDateTime,
}
