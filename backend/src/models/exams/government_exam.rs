use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::database::enums::{ExamLevel, ExamStatus};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::government_exams)]
pub struct GovernmentExam {
    pub id: String,
    pub exam_structure_id: String,
    pub name: String,
    pub authority: Option<String>,
    pub level: Option<ExamLevel>,
    pub exam_year: Option<i32>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: ExamStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::government_exams)]
pub struct NewGovernmentExam {
    pub id: String,
    pub exam_structure_id: String,
    pub name: String,
    pub authority: Option<String>,
    pub level: Option<ExamLevel>,
    pub exam_year: Option<i32>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub status: ExamStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::government_exam_subjects)]
pub struct GovernmentExamSubject {
    pub id: String,
    pub government_exam_id: String,
    pub subject_id: String,
    pub exam_date: Option<NaiveDate>,
    pub exam_time: Option<NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::government_exam_subjects)]
pub struct NewGovernmentExamSubject {
    pub id: String,
    pub government_exam_id: String,
    pub subject_id: String,
    pub exam_date: Option<NaiveDate>,
    pub exam_time: Option<NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct GovernmentExamQuery {
    pub search: Option<String>,
    pub status: Option<ExamStatus>,
    pub exam_structure_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for GovernmentExamQuery {
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
pub struct GovernmentExamSubjectQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for GovernmentExamSubjectQuery {
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
pub struct CreateGovernmentExamRequest {
    pub exam_structure_id: String,
    pub name: String,
    pub authority: Option<String>,
    pub level: Option<ExamLevel>,
    pub exam_year: Option<i32>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub status: ExamStatus,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default, AsChangeset)]
#[diesel(table_name = crate::schema::government_exams)]
pub struct UpdateGovernmentExamRequest {
    pub exam_structure_id: Option<String>,
    pub name: Option<String>,
    pub authority: Option<String>,
    pub level: Option<ExamLevel>,
    pub exam_year: Option<i32>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub status: Option<ExamStatus>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateGovernmentExamSubjectRequest {
    pub government_exam_id: String,
    pub subject_id: String,
    pub exam_date: Option<chrono::NaiveDate>,
    pub exam_time: Option<chrono::NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default, AsChangeset)]
#[diesel(table_name = crate::schema::government_exam_subjects)]
pub struct UpdateGovernmentExamSubjectRequest {
    pub government_exam_id: Option<String>,
    pub subject_id: Option<String>,
    pub exam_date: Option<chrono::NaiveDate>,
    pub exam_time: Option<chrono::NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
}
