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
        ..Default::default()}
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
        ..Default::default()}
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

impl From<CreateMarkingSchemeRequest> for MarkingScheme {
    fn from(req: CreateMarkingSchemeRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: req.name,
            subject_id: req.subject_id,
            grade_level_id: req.grade_level_id,
            curriculum_standard_id: req.curriculum_standard_id,
            stream_id: req.stream_id,
            description: req.description,
            valid_from: req.valid_from,
            valid_to: req.valid_to,
            calculation_fn: req.calculation_fn,
            is_active: req.is_active.unwrap_or(true),
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<CreateMarkingSchemePartRequest> for MarkingSchemePart {
    fn from(req: CreateMarkingSchemePartRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            scheme_id: req.scheme_id,
            paper_label: req.paper_label,
            part_label: req.part_label,
            question_label: req.question_label,
            max_marks: req.max_marks,
            weight_ratio: req.weight_ratio,
            structure_json: req.structure_json,
            order_index: req.order_index,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
