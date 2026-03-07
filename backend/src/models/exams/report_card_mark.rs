use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::database::enums::AssessmentType;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[diesel(table_name = crate::schema::report_card_marks)]

pub struct ReportCardMark {
    pub id: String,
    pub report_card_id: String,
    pub subject_id: String,
    pub assessment_type: AssessmentType,
    pub assessment_id: String,
    pub marking_scheme_id: Option<String>,
    pub total_marks: Option<f32>,
    pub percentage: Option<f32>,
    pub grade: Option<String>,
    pub grade_point: Option<f32>,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
