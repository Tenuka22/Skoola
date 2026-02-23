use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::report_card_marks)]

pub struct ReportCardMark {
    pub id: String,
    pub report_card_id: String,
    pub subject_id: String,
    pub marks_obtained: Option<i32>,
    pub grade: Option<String>,
    pub remarks: Option<String>,
}
