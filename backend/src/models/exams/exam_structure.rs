use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::database::enums::{ExamScopeType, Medium};
use crate::services::admin_db::{AdminQuery, AsAdminQuery};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::exam_structures)]
pub struct ExamStructure {
    pub id: String,
    pub name: String,
    pub scope_type: ExamScopeType,
    pub medium: Option<Medium>,
    pub description: Option<String>,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ExamStructureQuery {
    pub search: Option<String>,
    pub scope_type: Option<ExamScopeType>,
    pub is_active: Option<bool>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for ExamStructureQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::exam_structure_subjects)]
pub struct ExamStructureSubject {
    pub id: String,
    pub structure_id: String,
    pub subject_id: String,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
    pub order_index: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct ExamStructureSubjectResponse {
    pub id: String,
    pub structure_id: String,
    pub subject_id: String,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
    pub order_index: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ExamStructureSubject> for ExamStructureSubjectResponse {
    fn from(s: ExamStructureSubject) -> Self {
        Self {
            id: s.id,
            structure_id: s.structure_id,
            subject_id: s.subject_id,
            duration_minutes: s.duration_minutes,
            max_marks: s.max_marks,
            pass_marks: s.pass_marks,
            order_index: s.order_index,
            created_at: s.created_at,
            updated_at: s.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateExamStructureSubjectRequest {
    pub structure_id: String,
    pub subject_id: String,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
    pub order_index: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = crate::schema::exam_structure_subjects)]
pub struct UpdateExamStructureSubjectRequest {
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
    pub order_index: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ExamStructureSubjectQuery {
    pub structure_id: Option<String>,
    pub subject_id: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for ExamStructureSubjectQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

impl From<CreateExamStructureSubjectRequest> for ExamStructureSubject {
    fn from(req: CreateExamStructureSubjectRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            structure_id: req.structure_id,
            subject_id: req.subject_id,
            duration_minutes: req.duration_minutes,
            max_marks: req.max_marks,
            pass_marks: req.pass_marks,
            order_index: req.order_index,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
