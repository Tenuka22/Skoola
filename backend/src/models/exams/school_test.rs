use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::database::enums::{ExamStatus, SchoolTestType};

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct SchoolTestQuery {
    pub search: Option<String>,
    pub status: Option<ExamStatus>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub exam_structure_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for SchoolTestQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct SchoolTestSubjectQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for SchoolTestSubjectQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        ..Default::default()}
    }
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateSchoolTestRequest {
    pub exam_structure_id: String,
    pub name: String,
    pub test_type: SchoolTestType,
    pub academic_year_id: String,
    pub term_id: Option<String>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub status: ExamStatus,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default, AsChangeset)]
#[diesel(table_name = crate::schema::school_tests)]
pub struct UpdateSchoolTestRequest {
    pub exam_structure_id: Option<String>,
    pub name: Option<String>,
    pub test_type: Option<SchoolTestType>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub start_date: Option<chrono::NaiveDate>,
    pub end_date: Option<chrono::NaiveDate>,
    pub status: Option<ExamStatus>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateSchoolTestSubjectRequest {
    pub school_test_id: String,
    pub subject_id: String,
    pub test_date: Option<chrono::NaiveDate>,
    pub test_time: Option<chrono::NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default, AsChangeset)]
#[diesel(table_name = crate::schema::school_test_subjects)]
pub struct UpdateSchoolTestSubjectRequest {
    pub school_test_id: Option<String>,
    pub subject_id: Option<String>,
    pub test_date: Option<chrono::NaiveDate>,
    pub test_time: Option<chrono::NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::school_tests)]
pub struct SchoolTest {
    pub id: String,
    pub exam_structure_id: String,
    pub name: String,
    pub test_type: SchoolTestType,
    pub academic_year_id: String,
    pub term_id: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub created_by: String,
    pub status: ExamStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::school_tests)]
pub struct NewSchoolTest {
    pub id: String,
    pub exam_structure_id: String,
    pub name: String,
    pub test_type: SchoolTestType,
    pub academic_year_id: String,
    pub term_id: Option<String>,
    pub start_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub created_by: String,
    pub status: ExamStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::school_test_subjects)]
pub struct SchoolTestSubject {
    pub id: String,
    pub school_test_id: String,
    pub subject_id: String,
    pub test_date: Option<NaiveDate>,
    pub test_time: Option<NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::school_test_subjects)]
pub struct NewSchoolTestSubject {
    pub id: String,
    pub school_test_id: String,
    pub subject_id: String,
    pub test_date: Option<NaiveDate>,
    pub test_time: Option<NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub max_marks: Option<i32>,
    pub pass_marks: Option<i32>,
}

impl From<CreateSchoolTestRequest> for SchoolTest {
    fn from(req: CreateSchoolTestRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            exam_structure_id: req.exam_structure_id,
            name: req.name,
            test_type: req.test_type,
            academic_year_id: req.academic_year_id,
            term_id: req.term_id,
            start_date: req.start_date,
            end_date: req.end_date,
            created_by: String::new(), // Should be set by the caller
            status: req.status,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

impl From<CreateSchoolTestSubjectRequest> for SchoolTestSubject {
    fn from(req: CreateSchoolTestSubjectRequest) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            school_test_id: req.school_test_id,
            subject_id: req.subject_id,
            test_date: req.test_date,
            test_time: req.test_time,
            duration_minutes: req.duration_minutes,
            max_marks: req.max_marks,
            pass_marks: req.pass_marks,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}
