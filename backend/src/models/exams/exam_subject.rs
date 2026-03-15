use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::services::admin_db::{AdminQuery, AsAdminQuery};

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateExamSubjectRequest {
    pub exam_id: String,
    pub subject_id: String,
    pub date: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
    pub duration: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent, Default, AsChangeset)]
#[diesel(table_name = crate::schema::exam_subjects)]
pub struct UpdateExamSubjectRequest {
    pub date: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
    pub duration: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::exam_subjects)]
pub struct ExamSubject {
    pub id: String,
    pub exam_id: String,
    pub subject_id: String,
    pub date: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
    pub duration: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ExamSubjectResponse {
    pub id: String,
    pub exam_id: String,
    pub subject_id: String,
    pub date: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
    pub duration: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
}

impl From<ExamSubject> for ExamSubjectResponse {
    fn from(v: ExamSubject) -> Self {
        Self {
            id: v.id,
            exam_id: v.exam_id,
            subject_id: v.subject_id,
            date: v.date,
            time: v.time,
            duration: v.duration,
            max_marks: v.max_marks,
            pass_marks: v.pass_marks,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ExamSubjectQuery {
    pub exam_id: Option<String>,
    pub subject_id: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for ExamSubjectQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}
