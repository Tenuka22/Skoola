use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use diesel::prelude::*;
use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::database::enums::{ExamStatus, SchoolTestType};

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
