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

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = al_streams)]
pub struct CreateAlStreamRequest {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub version_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_active: bool,
}

impl From<CreateAlStreamRequest> for AlStream {
    fn from(req: CreateAlStreamRequest) -> Self {
        AlStream {
            id: req.id,
            name: req.name,
            description: req.description,
            version_name: req.version_name,
            start_date: req.start_date,
            end_date: req.end_date,
            is_active: req.is_active,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = al_streams)]
pub struct UpdateAlStreamRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub version_name: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct AlStreamQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for AlStreamQuery {
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
pub struct AlStreamResponse {
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

impl From<AlStream> for AlStreamResponse {
    fn from(al_stream: AlStream) -> Self {
        AlStreamResponse {
            id: al_stream.id,
            name: al_stream.name,
            description: al_stream.description,
            version_name: al_stream.version_name,
            start_date: al_stream.start_date,
            end_date: al_stream.end_date,
            is_active: al_stream.is_active,
            created_at: al_stream.created_at,
            updated_at: al_stream.updated_at,
        }
    }
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

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct GradeSubjectQuery {
    pub grade_id: Option<String>,
    pub subject_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for GradeSubjectQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: None,
            sort_by: None,
            sort_order: None,
            page: None,
            limit: None,
            last_id: None,
        ..Default::default()}
    }
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

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct AlStreamRequiredSubjectQuery {
    pub stream_id: Option<String>,
    pub subject_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for AlStreamRequiredSubjectQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: None,
            sort_by: None,
            sort_order: None,
            page: None,
            limit: None,
            last_id: None,
        ..Default::default()}
    }
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

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = al_stream_optional_groups)]
pub struct CreateAlStreamOptionalGroupRequest {
    pub id: String,
    pub stream_id: String,
    pub group_name: String,
    pub min_select: i32,
    pub max_select: Option<i32>,
}

impl From<CreateAlStreamOptionalGroupRequest> for AlStreamOptionalGroup {
    fn from(req: CreateAlStreamOptionalGroupRequest) -> Self {
        AlStreamOptionalGroup {
            id: req.id,
            stream_id: req.stream_id,
            group_name: req.group_name,
            min_select: req.min_select,
            max_select: req.max_select,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = al_stream_optional_groups)]
pub struct UpdateAlStreamOptionalGroupRequest {
    pub stream_id: Option<String>,
    pub group_name: Option<String>,
    pub min_select: Option<i32>,
    pub max_select: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct AlStreamOptionalGroupQuery {
    pub stream_id: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for AlStreamOptionalGroupQuery {
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
pub struct AlStreamOptionalGroupResponse {
    pub id: String,
    pub stream_id: String,
    pub group_name: String,
    pub min_select: i32,
    pub max_select: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<AlStreamOptionalGroup> for AlStreamOptionalGroupResponse {
    fn from(group: AlStreamOptionalGroup) -> Self {
        AlStreamOptionalGroupResponse {
            id: group.id,
            stream_id: group.stream_id,
            group_name: group.group_name,
            min_select: group.min_select,
            max_select: group.max_select,
            created_at: group.created_at,
            updated_at: group.updated_at,
        }
    }
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

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct AlStreamOptionalSubjectQuery {
    pub group_id: Option<String>,
    pub subject_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for AlStreamOptionalSubjectQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: None,
            sort_by: None,
            sort_order: None,
            page: None,
            limit: None,
            last_id: None,
        ..Default::default()}
    }
}
