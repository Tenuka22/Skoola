use crate::schema::students;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use chrono::{NaiveDate, NaiveDateTime};

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
    pub gender: String,
    pub address: String,
    pub phone: String,
    pub email: Option<String>,
    pub religion: Option<String>,
    pub ethnicity: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct CreateStudentRequest {
    pub admission_number: String,
    pub name_english: String,
    pub name_sinhala: Option<String>,
    pub name_tamil: Option<String>,
    pub nic_or_birth_certificate: String,
    pub dob: NaiveDate,
    pub gender: String,
    pub address: String,
    pub phone: String,
    pub email: Option<String>,
    pub religion: Option<String>,
    pub ethnicity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema)]
#[diesel(table_name = students)]
pub struct UpdateStudentRequest {
    pub name_english: Option<String>,
    pub name_sinhala: Option<String>,
    pub name_tamil: Option<String>,
    pub nic_or_birth_certificate: Option<String>,
    pub dob: Option<NaiveDate>,
    pub gender: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub religion: Option<String>,
    pub ethnicity: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct StudentResponse {
    pub id: String,
    pub admission_number: String,
    pub name_english: String,
    pub name_sinhala: Option<String>,
    pub name_tamil: Option<String>,
    pub nic_or_birth_certificate: String,
    pub dob: NaiveDate,
    pub gender: String,
    pub address: String,
    pub phone: String,
    pub email: Option<String>,
    pub religion: Option<String>,
    pub ethnicity: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
        }
    }
}