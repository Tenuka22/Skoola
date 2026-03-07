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

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
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
