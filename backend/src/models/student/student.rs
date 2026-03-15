use crate::database::enums::{Ethnicity, Gender, Religion, StudentStatus};
use crate::schema::students;
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
    pub profile_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
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
    pub profile_id: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct CreateStudentRequest {
    pub id: String,
    pub admission_number: String,
    pub name_english: String,
    pub name_sinhala: Option<String>,
    pub name_tamil: Option<String>,
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

#[derive(Debug, Default, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct UpdateStudentRequest {
    // students
    pub admission_number: Option<String>,
    pub name_english: Option<String>,
    pub name_sinhala: Option<String>,
    pub name_tamil: Option<String>,
    pub dob: Option<NaiveDate>,
    pub gender: Option<Gender>,
    pub profile_id: Option<String>,

    // profiles (linked via profile_id)
    pub profile_name: Option<String>,

    // student_contacts
    pub address: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,

    // student_status
    pub status: Option<StudentStatus>,

    // student_media
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct StudentQuery {
    pub search: Option<String>,
    pub status: Option<String>,
    pub created_after: Option<String>,
    pub created_before: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StudentQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
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
    pub status: Option<StudentStatus>,
    pub photo_url: Option<String>,
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
            status: None,
            photo_url: None,
        }
    }
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    Clone,
    ApiComponent,
)]
#[diesel(table_name = crate::schema::student_status)]
#[diesel(primary_key(student_id))]
pub struct StudentStatusRecord {
    pub student_id: String,
    pub status: StudentStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct StudentStatusQuery {
    pub search: Option<String>,
    pub status: Option<StudentStatus>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StudentStatusQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateStudentStatusRequest {
    pub student_id: String,
    pub status: StudentStatus,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema, ApiComponent, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::student_status)]
pub struct UpdateStudentStatusRequest {
    pub status: Option<StudentStatus>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct StudentStatusResponse {
    pub student_id: String,
    pub status: StudentStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StudentStatusRecord> for StudentStatusResponse {
    fn from(record: StudentStatusRecord) -> Self {
        Self {
            student_id: record.student_id,
            status: record.status,
            created_at: record.created_at,
            updated_at: record.updated_at,
        }
    }
}
