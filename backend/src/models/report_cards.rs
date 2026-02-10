use crate::schema::{report_card_marks, report_cards};
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
#[diesel(table_name = report_cards)]
pub struct ReportCard {
    pub id: String,
    pub student_id: String,
    pub academic_year_id: String,
    pub term_id: String,
    pub class_id: String,
    pub generated_at: NaiveDateTime,
    pub generated_by: String,
    pub final_grade: Option<String>,
    pub rank: Option<i32>,
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ApiComponent, JsonSchema)]
#[diesel(table_name = report_cards)]
pub struct CreateReportCard {
    pub id: String,
    pub student_id: String,
    pub academic_year_id: String,
    pub term_id: String,
    pub class_id: String,
    pub generated_by: String,
    pub final_grade: Option<String>,
    pub rank: Option<i32>,
    pub remarks: Option<String>,
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
#[diesel(table_name = report_card_marks)]
pub struct ReportCardMark {
    pub id: String,
    pub report_card_id: String,
    pub subject_id: String,
    pub marks_obtained: Option<i32>,
    pub grade: Option<String>,
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ApiComponent, JsonSchema)]
#[diesel(table_name = report_card_marks)]
pub struct CreateReportCardMark {
    pub id: String,
    pub report_card_id: String,
    pub subject_id: String,
    pub marks_obtained: Option<i32>,
    pub grade: Option<String>,
    pub remarks: Option<String>,
}
