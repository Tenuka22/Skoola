use crate::schema::student_mark_entries;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;

#[derive(
    Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent
)]
#[diesel(table_name = student_mark_entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentMarkEntry {
    pub id: String,
    pub student_mark_id: String,
    pub marking_scheme_part_id: String,
    pub marks_awarded: f32,
    pub max_marks: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct StudentMarkEntryQuery {
    pub search: Option<String>,
    pub student_mark_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StudentMarkEntryQuery {
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
pub struct CreateStudentMarkEntryRequest {
    pub student_mark_id: String,
    pub marking_scheme_part_id: String,
    pub marks_awarded: f32,
    pub max_marks: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = student_mark_entries)]
pub struct UpdateStudentMarkEntryRequest {
    pub marks_awarded: Option<f32>,
    pub max_marks: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct StudentMarkEntryResponse {
    pub id: String,
    pub student_mark_id: String,
    pub marking_scheme_part_id: String,
    pub marks_awarded: f32,
    pub max_marks: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StudentMarkEntry> for StudentMarkEntryResponse {
    fn from(entry: StudentMarkEntry) -> Self {
        StudentMarkEntryResponse {
            id: entry.id,
            student_mark_id: entry.student_mark_id,
            marking_scheme_part_id: entry.marking_scheme_part_id,
            marks_awarded: entry.marks_awarded,
            max_marks: entry.max_marks,
            created_at: entry.created_at,
            updated_at: entry.updated_at,
        }
    }
}
