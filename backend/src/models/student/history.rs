use crate::schema::{student_previous_schools, student_class_assignments_history};
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::{NaiveDate, NaiveDateTime};

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema)]
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

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema)]
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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CreateStudentPreviousSchoolRequest {
    pub student_id: String,
    pub school_name: String,
    pub grade_left: Option<String>,
    pub date_left: Option<NaiveDate>,
    pub reason_for_leaving: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema)]
#[diesel(table_name = student_previous_schools)]
pub struct UpdateStudentPreviousSchoolRequest {
    pub school_name: Option<String>,
    pub grade_left: Option<String>,
    pub date_left: Option<NaiveDate>,
    pub reason_for_leaving: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
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
