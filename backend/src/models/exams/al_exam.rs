use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::al_exams)]
#[diesel(treat_compound_current_type_as_possible_option_type = "true")]
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
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
