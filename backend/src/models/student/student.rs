use crate::database::enums::{Ethnicity, Gender, Religion, StudentStatus};
use crate::schema::{student_contacts, student_demographics, student_media, student_status, students};
use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema,
)]
#[diesel(table_name = students)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Student {
    pub id: String,
    pub admission_number: String,
    pub name_english: String,
    pub name_sinhala: Option<String>,
    pub name_tamil: Option<String>,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub profile_id: Option<String>,
}

#[derive(Debug, Insertable, Clone)]
#[diesel(table_name = students)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct NewStudent {
    pub id: String,
    pub admission_number: String,
    pub name_english: String,
    pub name_sinhala: Option<String>,
    pub name_tamil: Option<String>,
    pub dob: NaiveDate,
    pub gender: Gender,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub profile_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct CreateStudentRequest {
    pub id: String,
    pub admission_number: String,
    pub name_english: String,         // Still needed for profile creation
    pub name_sinhala: Option<String>, // Still needed for profile creation
    pub name_tamil: Option<String>,   // Still needed for profile creation
    pub dob: NaiveDate,
    pub gender: Gender,
    pub address: String,       // Still needed for profile creation
    pub phone: String,         // Still needed for profile creation
    pub email: Option<String>, // Still needed for user creation
    pub religion: Option<Religion>,
    pub ethnicity: Option<Ethnicity>,
    pub status: Option<StudentStatus>,
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct UpdateStudentRequest {
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
    pub dob: NaiveDate,
    pub gender: Gender,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub profile_id: Option<String>,
    pub profile_name: Option<String>,
    pub profile_address: Option<String>,
    pub profile_phone: Option<String>,
    pub profile_photo_url: Option<String>,
    pub user_email: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub religion: Option<Religion>,
    pub ethnicity: Option<Ethnicity>,
    pub status: Option<StudentStatus>,
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct PaginatedStudentResponse {
    pub data: Vec<StudentResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub next_last_id: Option<String>,
}

impl From<Student> for StudentResponse {
    fn from(student: Student) -> Self {
        Self {
            id: student.id,
            admission_number: student.admission_number,
            name_english: student.name_english,
            dob: student.dob,
            gender: student.gender,
            created_at: student.created_at,
            updated_at: student.updated_at,
            profile_id: student.profile_id,
            profile_name: None,
            profile_address: None,
            profile_phone: None,
            profile_photo_url: None,
            user_email: None,
            address: None,
            phone: None,
            email: None,
            religion: None,
            ethnicity: None,
            status: None,
            photo_url: None,
        }
    }
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema,
)]
#[diesel(table_name = student_contacts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentContact {
    pub student_id: String,
    pub address: String,
    pub address_latitude: Option<f32>,
    pub address_longitude: Option<f32>,
    pub phone: String,
    pub email: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema,
)]
#[diesel(table_name = student_demographics)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentDemographics {
    pub student_id: String,
    pub religion: Option<Religion>,
    pub ethnicity: Option<Ethnicity>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema,
)]
#[diesel(table_name = student_status)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentStatusRow {
    pub student_id: String,
    pub status: StudentStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema,
)]
#[diesel(table_name = student_media)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentMedia {
    pub student_id: String,
    pub photo_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
