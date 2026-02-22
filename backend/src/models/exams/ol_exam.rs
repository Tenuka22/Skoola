use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::ol_exams)]
#[diesel(treat_compound_current_type_as_possible_option_type = "true")]
pub struct OlExam {
    pub id: String,
    pub student_id: String,
    pub exam_year: i32,
    pub index_number: Option<String>,
    pub medium: Option<String>,
    pub results_summary: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}
