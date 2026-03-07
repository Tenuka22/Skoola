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
