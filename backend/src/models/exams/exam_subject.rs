use crate::schema::exam_subjects;
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
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
#[diesel(table_name = exam_subjects)]
#[diesel(primary_key(exam_id, subject_id))] // Specify composite primary key
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ExamSubject {
    pub exam_id: String,
    pub subject_id: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub duration: i32,
    pub max_marks: i32,
    pub pass_marks: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = exam_subjects)]
pub struct CreateExamSubjectRequest {
    pub exam_id: String,
    pub subject_id: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub duration: i32,
    pub max_marks: i32,
    pub pass_marks: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = exam_subjects)]
#[diesel(primary_key(exam_id, subject_id))] // Also specify for AsChangeset
pub struct UpdateExamSubjectRequest {
    pub date: Option<NaiveDate>,
    pub time: Option<NaiveTime>,
    pub duration: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct ExamSubjectResponse {
    pub exam_id: String,
    pub subject_id: String,
    pub date: NaiveDate,
    pub time: NaiveTime,
    pub duration: i32,
    pub max_marks: i32,
    pub pass_marks: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ExamSubject> for ExamSubjectResponse {
    fn from(exam_subject: ExamSubject) -> Self {
        ExamSubjectResponse {
            exam_id: exam_subject.exam_id,
            subject_id: exam_subject.subject_id,
            date: exam_subject.date,
            time: exam_subject.time,
            duration: exam_subject.duration,
            max_marks: exam_subject.max_marks,
            pass_marks: exam_subject.pass_marks,
            created_at: exam_subject.created_at,
            updated_at: exam_subject.updated_at,
        }
    }
}
