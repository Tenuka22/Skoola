use crate::schema::students;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::{NaiveDate, NaiveDateTime};
use crate::database::enums::{Gender, Religion, Ethnicity, StudentStatus};
use apistos::ApiComponent;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema)]
#[diesel(table_name = students)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Student {
    pub id: String,
    pub admission_number: String,
    pub nic_or_birth_certificate: String,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub religion: Option<Religion>,
    pub ethnicity: Option<Ethnicity>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: StudentStatus,
    pub profile_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct CreateStudentRequest {
    pub admission_number: String,
    pub name_english: String, // Still needed for profile creation
    pub name_sinhala: Option<String>, // Still needed for profile creation
    pub name_tamil: Option<String>, // Still needed for profile creation
    pub nic_or_birth_certificate: String,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub address: String, // Still needed for profile creation
    pub phone: String, // Still needed for profile creation
    pub email: Option<String>, // Still needed for user creation
    pub religion: Option<Religion>,
    pub ethnicity: Option<Ethnicity>,
    pub status: Option<StudentStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = students)]
pub struct UpdateStudentRequest {
    pub nic_or_birth_certificate: Option<String>,
    pub dob: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub religion: Option<Religion>,
    pub ethnicity: Option<Ethnicity>,
    pub status: Option<StudentStatus>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct StudentResponse {
    pub id: String,
    pub admission_number: String,
    pub nic_or_birth_certificate: String,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub religion: Option<Religion>,
    pub ethnicity: Option<Ethnicity>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: StudentStatus,
    pub profile_id: Option<String>,
    pub profile_name: Option<String>,
    pub profile_address: Option<String>,
    pub profile_phone: Option<String>,
    pub profile_photo_url: Option<String>,
    pub user_email: Option<String>,
}



#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct PaginatedStudentResponse {
    pub data: Vec<StudentResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}