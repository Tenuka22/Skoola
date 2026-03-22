use crate::schema::student_class_assignments;
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent
)]
#[diesel(table_name = student_class_assignments)]
pub struct StudentClassAssignment {
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

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct CreateStudentClassAssignmentRequest {
    pub student_id: String,
    pub academic_year_id: String,
    pub grade_id: String,
    pub class_id: String,
    pub from_date: NaiveDate,
    pub to_date: Option<NaiveDate>,
}

impl From<CreateStudentClassAssignmentRequest> for StudentClassAssignment {
    fn from(req: CreateStudentClassAssignmentRequest) -> Self {
        StudentClassAssignment {
            id: uuid::Uuid::new_v4().to_string(),
            student_id: req.student_id,
            academic_year_id: req.academic_year_id,
            grade_id: req.grade_id,
            class_id: req.class_id,
            from_date: req.from_date,
            to_date: req.to_date,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = student_class_assignments)]
pub struct UpdateStudentClassAssignmentRequest {
    pub academic_year_id: Option<String>,
    pub grade_id: Option<String>,
    pub class_id: Option<String>,
    pub from_date: Option<NaiveDate>,
    pub to_date: Option<NaiveDate>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct StudentClassAssignmentResponse {
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

impl From<StudentClassAssignment> for StudentClassAssignmentResponse {
    fn from(assignment: StudentClassAssignment) -> Self {
        StudentClassAssignmentResponse {
            id: assignment.id,
            student_id: assignment.student_id,
            academic_year_id: assignment.academic_year_id,
            grade_id: assignment.grade_id,
            class_id: assignment.class_id,
            from_date: assignment.from_date,
            to_date: assignment.to_date,
            created_at: assignment.created_at,
            updated_at: assignment.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct PromoteStudentRequest {
    pub student_id: String,
    pub current_academic_year_id: String,
    pub new_academic_year_id: String,
    pub new_grade_id: String,
    pub new_class_id: String,
    pub new_assignment_from_date: NaiveDate,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct BulkAssignStudentClassRequest {
    pub assignments: Vec<CreateStudentClassAssignmentRequest>,
}
