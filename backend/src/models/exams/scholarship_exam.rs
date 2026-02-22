use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::scholarship_exams)]
#[diesel(treat_compound_current_type_as_possible_option_type = "true")]
pub struct ScholarshipExam {
    pub id: String,
    pub student_id: String,
    pub exam_year: i32,
    pub index_number: Option<String>,
    pub marks: Option<i32>,
    pub district_rank: Option<i32>,
    pub island_rank: Option<i32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
