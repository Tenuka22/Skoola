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
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct CreateStudentRequest {
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
    pub status: Option<StudentStatus>,
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = students)]
pub struct UpdateStudentRequest {
    pub name_english: Option<String>,
    pub name_sinhala: Option<String>,
    pub name_tamil: Option<String>,
    pub nic_or_birth_certificate: Option<String>,
    pub dob: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub religion: Option<Religion>,
    pub ethnicity: Option<Ethnicity>,
    pub status: Option<StudentStatus>,
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct StudentResponse {
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
}

impl From<Student> for StudentResponse {
    fn from(student: Student) -> Self {
        StudentResponse {
            id: student.id,
            admission_number: student.admission_number,
            name_english: student.name_english,
            name_sinhala: student.name_sinhala,
            name_tamil: student.name_tamil,
            nic_or_birth_certificate: student.nic_or_birth_certificate,
            dob: student.dob,
            gender: student.gender,
            address: student.address,
            phone: student.phone,
            email: student.email,
            religion: student.religion,
            ethnicity: student.ethnicity,
            created_at: student.created_at,
            updated_at: student.updated_at,
            status: student.status,
            photo_url: student.photo_url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct PaginatedStudentResponse {
    pub students: Vec<StudentResponse>,
    pub total_students: i64,
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct StudentSearchQuery {
    pub name: Option<String>,
    pub admission_number: Option<String>,
    #[serde(flatten)]
    pub pagination: PaginationInfo,
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct StudentFilterQuery {
    pub grade_id: Option<String>,
    pub class_id: Option<String>,
    pub status: Option<StudentStatus>, // Assuming StudentStatus enum exists
    #[serde(flatten)]
    pub pagination: PaginationInfo,
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct PaginationInfo {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}