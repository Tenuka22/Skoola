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
    pub name_english: String,
    pub name_sinhala: Option<String>,
    pub name_tamil: Option<String>,
    pub nic_or_birth_certificate: String,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub address: String,
    pub phone: String,
    pub email: Option<String>,
    pub religion: Option<Religion>,
    pub ethnicity: Option<Ethnicity>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: StudentStatus,
    pub photo_url: Option<String>,
    pub profile_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent, Insertable)]
#[diesel(table_name = students)]
pub struct CreateStudentRequest {
    pub id: String,
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
    pub name_english: String,
    pub nic_or_birth_certificate: String,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub email: Option<String>,
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

impl From<Student> for StudentResponse {
    fn from(student: Student) -> Self {
        Self {
            id: student.id,
            admission_number: student.admission_number,
            name_english: student.name_english,
            nic_or_birth_certificate: student.nic_or_birth_certificate,
            dob: student.dob,
            gender: student.gender,
            email: student.email,
            religion: student.religion,
            ethnicity: student.ethnicity,
            created_at: student.created_at,
            updated_at: student.updated_at,
            status: student.status,
            profile_id: student.profile_id,
            profile_name: None,
            profile_address: None,
            profile_phone: None,
            profile_photo_url: None,
            user_email: None,
        }
    }
}