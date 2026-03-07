use crate::schema::{report_card_marks, report_cards};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::database::enums::AssessmentType;

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
    pub grading_scheme_id: Option<String>,
    pub generated_at: NaiveDateTime,
    pub generated_by: String,
    pub overall_percentage: Option<f32>,
    pub overall_grade: Option<String>,
    pub overall_gpa: Option<f32>,
    pub rank: Option<i32>,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ApiComponent, JsonSchema)]
#[diesel(table_name = report_cards)]
pub struct CreateReportCard {
    pub id: String,
    pub student_id: String,
    pub academic_year_id: String,
    pub term_id: String,
    pub class_id: String,
    pub grading_scheme_id: Option<String>,
    pub generated_at: NaiveDateTime,
    pub generated_by: String,
    pub overall_percentage: Option<f32>,
    pub overall_grade: Option<String>,
    pub overall_gpa: Option<f32>,
    pub rank: Option<i32>,
    pub remarks: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, ApiComponent, JsonSchema)]
#[diesel(table_name = report_card_marks)]
pub struct CreateReportCardMark {
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
