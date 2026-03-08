use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct MarkingSchemeQuery {
    pub search: Option<String>,
    pub subject_id: Option<String>,
    pub grade_level_id: Option<String>,
    pub curriculum_standard_id: Option<String>,
    pub stream_id: Option<String>,
    pub is_active: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for MarkingSchemeQuery {
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

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct MarkingSchemePartQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for MarkingSchemePartQuery {
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

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateMarkingSchemeRequest {
    pub name: String,
    pub subject_id: String,
    pub grade_level_id: Option<String>,
    pub curriculum_standard_id: Option<String>,
    pub stream_id: Option<String>,
    pub description: Option<String>,
    pub valid_from: Option<chrono::NaiveDate>,
    pub valid_to: Option<chrono::NaiveDate>,
    pub calculation_fn: String,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default, AsChangeset)]
#[diesel(table_name = crate::schema::marking_schemes)]
pub struct UpdateMarkingSchemeRequest {
    pub name: Option<String>,
    pub subject_id: Option<String>,
    pub grade_level_id: Option<String>,
    pub curriculum_standard_id: Option<String>,
    pub stream_id: Option<String>,
    pub description: Option<String>,
    pub valid_from: Option<chrono::NaiveDate>,
    pub valid_to: Option<chrono::NaiveDate>,
    pub calculation_fn: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateMarkingSchemePartRequest {
    pub scheme_id: String,
    pub paper_label: String,
    pub part_label: String,
    pub question_label: Option<String>,
    pub max_marks: f32,
    pub weight_ratio: Option<f32>,
    pub structure_json: Option<String>,
    pub order_index: i32,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default, AsChangeset)]
#[diesel(table_name = crate::schema::marking_scheme_parts)]
pub struct UpdateMarkingSchemePartRequest {
    pub scheme_id: Option<String>,
    pub paper_label: Option<String>,
    pub part_label: Option<String>,
    pub question_label: Option<String>,
    pub max_marks: Option<f32>,
    pub weight_ratio: Option<f32>,
    pub structure_json: Option<String>,
    pub order_index: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::marking_schemes)]
pub struct MarkingScheme {
    pub id: String,
    pub name: String,
    pub subject_id: String,
    pub grade_level_id: Option<String>,
    pub curriculum_standard_id: Option<String>,
    pub stream_id: Option<String>,
    pub description: Option<String>,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
    pub calculation_fn: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::marking_schemes)]
pub struct NewMarkingScheme {
    pub id: String,
    pub name: String,
    pub subject_id: String,
    pub grade_level_id: Option<String>,
    pub curriculum_standard_id: Option<String>,
    pub stream_id: Option<String>,
    pub description: Option<String>,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
    pub calculation_fn: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::marking_scheme_parts)]
pub struct MarkingSchemePart {
    pub id: String,
    pub scheme_id: String,
    pub paper_label: String,
    pub part_label: String,
    pub question_label: Option<String>,
    pub max_marks: f32,
    pub weight_ratio: Option<f32>,
    pub structure_json: Option<String>,
    pub order_index: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::marking_scheme_parts)]
pub struct NewMarkingSchemePart {
    pub id: String,
    pub scheme_id: String,
    pub paper_label: String,
    pub part_label: String,
    pub question_label: Option<String>,
    pub max_marks: f32,
    pub weight_ratio: Option<f32>,
    pub structure_json: Option<String>,
    pub order_index: i32,
}
