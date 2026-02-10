use crate::schema::{al_exams, ol_exams, scholarship_exams};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = scholarship_exams)]
pub struct ScholarshipExam {
    pub id: String,
    pub student_id: String,
    pub exam_year: i32,
    pub index_number: Option<String>,
    pub marks: Option<i32>,
    pub district_rank: Option<i32>,
    pub island_rank: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ApiComponent, JsonSchema)]
#[diesel(table_name = scholarship_exams)]
pub struct CreateScholarshipExam {
    pub id: String,
    pub student_id: String,
    pub exam_year: i32,
    pub index_number: Option<String>,
    pub marks: Option<i32>,
    pub district_rank: Option<i32>,
    pub island_rank: Option<i32>,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = ol_exams)]
pub struct OlExam {
    pub id: String,
    pub student_id: String,
    pub exam_year: i32,
    pub index_number: Option<String>,
    pub medium: Option<String>,
    pub results_summary: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ApiComponent, JsonSchema)]
#[diesel(table_name = ol_exams)]
pub struct CreateOlExam {
    pub id: String,
    pub student_id: String,
    pub exam_year: i32,
    pub index_number: Option<String>,
    pub medium: Option<String>,
    pub results_summary: Option<String>,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    Insertable,
    ApiComponent,
    JsonSchema,
)]
#[diesel(table_name = al_exams)]
pub struct AlExam {
    pub id: String,
    pub student_id: String,
    pub exam_year: i32,
    pub index_number: Option<String>,
    pub stream_id: Option<String>,
    pub z_score: Option<f64>,
    pub district_rank: Option<i32>,
    pub island_rank: Option<i32>,
    pub general_test_marks: Option<i32>,
    pub results_summary: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ApiComponent, JsonSchema)]
#[diesel(table_name = al_exams)]
pub struct CreateAlExam {
    pub id: String,
    pub student_id: String,
    pub exam_year: i32,
    pub index_number: Option<String>,
    pub stream_id: Option<String>,
    pub z_score: Option<f64>,
    pub district_rank: Option<i32>,
    pub island_rank: Option<i32>,
    pub general_test_marks: Option<i32>,
    pub results_summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct ExamRegistrationRequest {
    pub student_id: String,
    pub exam_year: i32,
    pub index_number: Option<String>,
}
