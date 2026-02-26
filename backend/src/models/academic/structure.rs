use crate::schema::{grade_streams, grade_subjects, stream_subjects, streams};
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
#[diesel(table_name = streams)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Stream {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
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
#[diesel(table_name = grade_streams)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(grade_id, stream_id))]
pub struct GradeStream {
    pub grade_id: String,
    pub stream_id: String,
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
#[diesel(table_name = stream_subjects)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(primary_key(stream_id, subject_id))]
pub struct StreamSubject {
    pub stream_id: String,
    pub subject_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
