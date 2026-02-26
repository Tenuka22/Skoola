use crate::schema::{student_marks, student_marks_history};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    pub exam_id: String,
    pub subject_id: String,
    pub marks_obtained: i32,
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
#[diesel(table_name = student_marks)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentMark {
    pub id: String,
    pub student_id: String,
    pub exam_id: String,
    pub subject_id: String,
    pub marks_obtained: i32,
    pub is_absent: bool,
    pub remarks: Option<String>,
    pub entered_by: String,
    pub entered_at: NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = student_marks)]
pub struct CreateStudentMarkRequest {
    pub student_id: String,
    pub exam_id: String,
    pub subject_id: String,
    pub marks_obtained: i32,
    pub is_absent: Option<bool>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = student_marks)]
pub struct UpdateStudentMarkRequest {
    pub marks_obtained: Option<i32>,
    pub is_absent: Option<bool>,
    pub remarks: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct BulkCreateStudentMarkRequest {
    pub marks: Vec<CreateStudentMarkRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct StudentMarkResponse {
    pub id: String,
    pub student_id: String,
    pub exam_id: String,
    pub subject_id: String,
    pub marks_obtained: i32,
    pub is_absent: bool,
    pub remarks: Option<String>,
    pub entered_by: String,
    pub entered_at: NaiveDateTime,
    pub updated_by: Option<String>,
    pub updated_at: NaiveDateTime,
}

impl From<StudentMark> for StudentMarkResponse {
    fn from(student_mark: StudentMark) -> Self {
        StudentMarkResponse {
            id: student_mark.id,
            student_id: student_mark.student_id,
            exam_id: student_mark.exam_id,
            subject_id: student_mark.subject_id,
            marks_obtained: student_mark.marks_obtained,
            is_absent: student_mark.is_absent,
            remarks: student_mark.remarks,
            entered_by: student_mark.entered_by,
            entered_at: student_mark.entered_at,
            updated_by: student_mark.updated_by,
            updated_at: student_mark.updated_at,
        }
    }
}
