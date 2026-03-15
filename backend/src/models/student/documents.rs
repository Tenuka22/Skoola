use apistos::ApiComponent;
use chrono::{NaiveDate, NaiveDateTime};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::student::student::Student;

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
    Associations,
    ApiComponent,
)]
#[diesel(table_name = crate::schema::student_birth_certificates)]
#[diesel(belongs_to(Student))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentBirthCertificate {
    pub id: String,
    pub student_id: String,
    pub certificate_number: String,
    pub issued_date: Option<NaiveDate>,
    pub document_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct StudentBirthCertificateQuery {
    pub search: Option<String>,
    pub student_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StudentBirthCertificateQuery {
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
pub struct CreateStudentBirthCertificateRequest {
    pub student_id: String,
    pub certificate_number: String,
    pub issued_date: Option<NaiveDate>,
    pub document_url: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema, ApiComponent, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::student_birth_certificates)]
pub struct UpdateStudentBirthCertificateRequest {
    pub certificate_number: Option<String>,
    pub issued_date: Option<NaiveDate>,
    pub document_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct StudentBirthCertificateResponse {
    pub id: String,
    pub student_id: String,
    pub certificate_number: String,
    pub issued_date: Option<NaiveDate>,
    pub document_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StudentBirthCertificate> for StudentBirthCertificateResponse {
    fn from(doc: StudentBirthCertificate) -> Self {
        Self {
            id: doc.id,
            student_id: doc.student_id,
            certificate_number: doc.certificate_number,
            issued_date: doc.issued_date,
            document_url: doc.document_url,
            created_at: doc.created_at,
            updated_at: doc.updated_at,
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
    Associations,
    ApiComponent,
)]
#[diesel(table_name = crate::schema::student_nics)]
#[diesel(belongs_to(Student))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentNic {
    pub id: String,
    pub student_id: String,
    pub nic_number: String,
    pub issued_date: Option<NaiveDate>,
    pub document_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct StudentNicQuery {
    pub search: Option<String>,
    pub student_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StudentNicQuery {
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
pub struct CreateStudentNicRequest {
    pub student_id: String,
    pub nic_number: String,
    pub issued_date: Option<NaiveDate>,
    pub document_url: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize, JsonSchema, ApiComponent, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::student_nics)]
pub struct UpdateStudentNicRequest {
    pub nic_number: Option<String>,
    pub issued_date: Option<NaiveDate>,
    pub document_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct StudentNicResponse {
    pub id: String,
    pub student_id: String,
    pub nic_number: String,
    pub issued_date: Option<NaiveDate>,
    pub document_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StudentNic> for StudentNicResponse {
    fn from(doc: StudentNic) -> Self {
        Self {
            id: doc.id,
            student_id: doc.student_id,
            nic_number: doc.nic_number,
            issued_date: doc.issued_date,
            document_url: doc.document_url,
            created_at: doc.created_at,
            updated_at: doc.updated_at,
        }
    }
}
