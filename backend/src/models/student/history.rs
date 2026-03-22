use crate::schema::{student_class_assignments_history, student_previous_schools};
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;

#[derive(
    Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent
)]
#[diesel(table_name = student_class_assignments_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentClassAssignmentHistory {
    pub id: String,
    pub student_id: String,
    pub academic_year_id: String,
    pub grade_id: String,
    pub class_id: String,
    pub from_date: NaiveDate,
    pub to_date: Option<NaiveDate>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent
)]
#[diesel(table_name = student_previous_schools)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentPreviousSchool {
    pub id: String,
    pub student_id: String,
    pub school_name: String,
    pub grade_left: Option<String>,
    pub date_left: Option<NaiveDate>,
    pub reason_for_leaving: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct CreateStudentPreviousSchoolRequest {
    pub student_id: String,
    pub school_name: String,
    pub grade_left: Option<String>,
    pub date_left: Option<NaiveDate>,
    pub reason_for_leaving: Option<String>,
}

impl From<CreateStudentPreviousSchoolRequest> for StudentPreviousSchool {
    fn from(req: CreateStudentPreviousSchoolRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            student_id: req.student_id,
            school_name: req.school_name,
            grade_left: req.grade_left,
            date_left: req.date_left,
            reason_for_leaving: req.reason_for_leaving,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = student_previous_schools)]
pub struct UpdateStudentPreviousSchoolRequest {
    pub school_name: Option<String>,
    pub grade_left: Option<String>,
    pub date_left: Option<NaiveDate>,
    pub reason_for_leaving: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct StudentPreviousSchoolResponse {
    pub id: String,
    pub student_id: String,
    pub school_name: String,
    pub grade_left: Option<String>,
    pub date_left: Option<NaiveDate>,
    pub reason_for_leaving: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StudentPreviousSchool> for StudentPreviousSchoolResponse {
    fn from(school: StudentPreviousSchool) -> Self {
        StudentPreviousSchoolResponse {
            id: school.id,
            student_id: school.student_id,
            school_name: school.school_name,
            grade_left: school.grade_left,
            date_left: school.date_left,
            reason_for_leaving: school.reason_for_leaving,
            created_at: school.created_at,
            updated_at: school.updated_at,
        }
    }
}
