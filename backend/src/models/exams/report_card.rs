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
    AsChangeset,
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

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CreateReportCardRequest {
    pub student_id: String,
    pub academic_year_id: String,
    pub term_id: String,
    pub class_id: String,
    pub grading_scheme_id: Option<String>,
    pub overall_percentage: Option<f32>,
    pub overall_grade: Option<String>,
    pub overall_gpa: Option<f32>,
    pub rank: Option<i32>,
    pub remarks: Option<String>,
    pub marks: Option<Vec<CreateReportCardMarkRequest>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, ApiComponent, JsonSchema)]
#[diesel(table_name = report_cards)]
pub struct UpdateReportCardRequest {
    pub grading_scheme_id: Option<String>,
    pub overall_percentage: Option<f32>,
    pub overall_grade: Option<String>,
    pub overall_gpa: Option<f32>,
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
    AsChangeset,
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

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CreateReportCardMarkRequest {
    pub subject_id: String,
    pub assessment_type: AssessmentType,
    pub assessment_id: String,
    pub marking_scheme_id: Option<String>,
    pub total_marks: Option<f32>,
    pub percentage: Option<f32>,
    pub grade: Option<String>,
    pub grade_point: Option<f32>,
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, ApiComponent, JsonSchema)]
#[diesel(table_name = report_card_marks)]
pub struct UpdateReportCardMarkRequest {
    pub marking_scheme_id: Option<String>,
    pub total_marks: Option<f32>,
    pub percentage: Option<f32>,
    pub grade: Option<String>,
    pub grade_point: Option<f32>,
    pub remarks: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct ReportCardMarkQuery {
    pub report_card_id: Option<String>,
    pub subject_id: Option<String>,
    pub assessment_type: Option<AssessmentType>,
    pub assessment_id: Option<String>,
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for ReportCardMarkQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct ReportCardMarkResponse {
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

impl From<ReportCardMark> for ReportCardMarkResponse {
    fn from(mark: ReportCardMark) -> Self {
        ReportCardMarkResponse {
            id: mark.id,
            report_card_id: mark.report_card_id,
            subject_id: mark.subject_id,
            assessment_type: mark.assessment_type,
            assessment_id: mark.assessment_id,
            marking_scheme_id: mark.marking_scheme_id,
            total_marks: mark.total_marks,
            percentage: mark.percentage,
            grade: mark.grade,
            grade_point: mark.grade_point,
            remarks: mark.remarks,
            created_at: mark.created_at,
            updated_at: mark.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct ReportCardQuery {
    pub student_id: Option<String>,
    pub class_id: Option<String>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub last_id: Option<String>,
    pub limit: Option<i64>,
    pub page: Option<i64>,
}
