use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::report_card_marks)]
#[diesel(treat_compound_current_type_as_possible_option_type = "true")]
pub struct ReportCardMark {
    pub id: String,
    pub report_card_id: String,
    pub subject_id: String,
    pub marks_obtained: Option<i32>,
    pub grade: Option<String>,
    pub remarks: Option<String>,
}
