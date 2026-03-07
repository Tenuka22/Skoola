use crate::schema::{
    student_mark_entries, student_mark_entries_history, student_marks, student_marks_history,
};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::database::enums::AssessmentType;

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = student_marks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentMark {
    pub id: String,
    pub student_id: String,
    pub subject_id: String,
    pub assessment_type: AssessmentType,
    pub assessment_id: String,
    pub marking_scheme_id: String,
    pub total_marks: Option<f32>,
    pub percentage: Option<f32>,
    pub grade: Option<String>,
    pub grade_point: Option<f32>,
    pub is_absent: bool,
    pub remarks: Option<String>,
    pub entered_by: String,
    pub entered_at: NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = student_mark_entries)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentMarkEntry {
    pub id: String,
    pub student_mark_id: String,
    pub marking_scheme_part_id: String,
    pub marks_awarded: f32,
    pub max_marks: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = student_marks_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentMarkHistory {
    pub id: String,
    pub student_id: String,
    pub subject_id: String,
    pub assessment_type: AssessmentType,
    pub assessment_id: String,
    pub marking_scheme_id: String,
    pub total_marks: Option<f32>,
    pub percentage: Option<f32>,
    pub grade: Option<String>,
    pub grade_point: Option<f32>,
    pub is_absent: bool,
    pub remarks: Option<String>,
    pub entered_by: String,
    pub entered_at: NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = student_mark_entries_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentMarkEntryHistory {
    pub id: String,
    pub student_marks_history_id: String,
    pub marking_scheme_part_id: String,
    pub marks_awarded: f32,
    pub max_marks: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct StudentMarkEntryInput {
    pub marking_scheme_part_id: String,
    pub marks_awarded: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct StudentMarkEntryResponse {
    pub id: String,
    pub marking_scheme_part_id: String,
    pub marks_awarded: f32,
    pub max_marks: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct CreateStudentMarkRequest {
    pub student_id: String,
    pub subject_id: String,
    pub assessment_type: AssessmentType,
    pub assessment_id: String,
    pub marking_scheme_id: String,
    pub is_absent: Option<bool>,
    pub remarks: Option<String>,
    pub entries: Vec<StudentMarkEntryInput>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct UpdateStudentMarkRequest {
    pub is_absent: Option<bool>,
    pub remarks: Option<String>,
    pub entries: Option<Vec<StudentMarkEntryInput>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct BulkCreateStudentMarkRequest {
    pub marks: Vec<CreateStudentMarkRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct StudentMarkResponse {
    pub id: String,
    pub student_id: String,
    pub subject_id: String,
    pub assessment_type: AssessmentType,
    pub assessment_id: String,
    pub marking_scheme_id: String,
    pub total_marks: Option<f32>,
    pub percentage: Option<f32>,
    pub grade: Option<String>,
    pub grade_point: Option<f32>,
    pub is_absent: bool,
    pub remarks: Option<String>,
    pub entered_by: String,
    pub entered_at: NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_at: NaiveDateTime,
    pub entries: Vec<StudentMarkEntryResponse>,
}

impl StudentMarkResponse {
    pub fn from_with_entries(
        student_mark: StudentMark,
        entries: Vec<StudentMarkEntryResponse>,
    ) -> Self {
        StudentMarkResponse {
            id: student_mark.id,
            student_id: student_mark.student_id,
            subject_id: student_mark.subject_id,
            assessment_type: student_mark.assessment_type,
            assessment_id: student_mark.assessment_id,
            marking_scheme_id: student_mark.marking_scheme_id,
            total_marks: student_mark.total_marks,
            percentage: student_mark.percentage,
            grade: student_mark.grade,
            grade_point: student_mark.grade_point,
            is_absent: student_mark.is_absent,
            remarks: student_mark.remarks,
            entered_by: student_mark.entered_by,
            entered_at: student_mark.entered_at,
            updated_by: student_mark.updated_by,
            updated_at: student_mark.updated_at,
            entries,
        }
    }
}
