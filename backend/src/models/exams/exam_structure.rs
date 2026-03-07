use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::database::enums::{ExamScopeType, Medium};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, JsonSchema, ApiComponent)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::exam_structures)]
pub struct NewExamStructure {
    pub id: String,
    pub name: String,
    pub scope_type: ExamScopeType,
    pub medium: Option<Medium>,
    pub description: Option<String>,
    pub valid_from: Option<NaiveDate>,
    pub valid_to: Option<NaiveDate>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::exam_structure_subjects)]
pub struct NewExamStructureSubject {
    pub id: String,
    pub structure_id: String,
    pub subject_id: String,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
    pub order_index: Option<i32>,
}
