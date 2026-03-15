use crate::schema::{student_contacts, student_media, student_nics};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::services::admin_db::{AdminQuery, AsAdminQuery};

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, AsChangeset, Clone, ApiComponent)]
#[diesel(table_name = student_contacts)]
#[diesel(primary_key(student_id))]
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

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct StudentContactResponse {
    pub student_id: String,
    pub address: String,
    pub address_latitude: Option<f32>,
    pub address_longitude: Option<f32>,
    pub phone: String,
    pub email: Option<String>,
}

impl From<StudentContact> for StudentContactResponse {
    fn from(c: StudentContact) -> Self {
        Self {
            student_id: c.student_id,
            address: c.address,
            address_latitude: c.address_latitude,
            address_longitude: c.address_longitude,
            phone: c.phone,
            email: c.email,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateStudentContactRequest {
    pub student_id: String,
    pub address: String,
    pub address_latitude: Option<f32>,
    pub address_longitude: Option<f32>,
    pub phone: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = student_contacts)]
pub struct UpdateStudentContactRequest {
    pub address: Option<String>,
    pub address_latitude: Option<f32>,
    pub address_longitude: Option<f32>,
    pub phone: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct StudentContactQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl AsAdminQuery for StudentContactQuery {
    fn as_admin_query(&self) -> AdminQuery {
        AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, AsChangeset, Clone, ApiComponent)]
#[diesel(table_name = student_media)]
#[diesel(primary_key(student_id))]
pub struct StudentMedia {
    pub student_id: String,
    pub photo_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct StudentMediaResponse {
    pub student_id: String,
    pub photo_url: Option<String>,
}

impl From<StudentMedia> for StudentMediaResponse {
    fn from(m: StudentMedia) -> Self {
        Self {
            student_id: m.student_id,
            photo_url: m.photo_url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateStudentMediaRequest {
    pub student_id: String,
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = student_media)]
pub struct UpdateStudentMediaRequest {
    pub photo_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, AsChangeset, Clone, ApiComponent)]
#[diesel(table_name = student_nics)]
pub struct StudentNic {
    pub id: String,
    pub student_id: String,
    pub nic_number: String,
    pub issued_date: Option<chrono::NaiveDate>,
    pub document_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct StudentNicResponse {
    pub id: String,
    pub student_id: String,
    pub nic_number: String,
    pub issued_date: Option<chrono::NaiveDate>,
    pub document_url: Option<String>,
}

impl From<StudentNic> for StudentNicResponse {
    fn from(i: StudentNic) -> Self {
        Self {
            id: i.id,
            student_id: i.student_id,
            nic_number: i.nic_number,
            issued_date: i.issued_date,
            document_url: i.document_url,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateStudentNicRequest {
    pub student_id: String,
    pub nic_number: String,
    pub issued_date: Option<chrono::NaiveDate>,
    pub document_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = student_nics)]
pub struct UpdateStudentNicRequest {
    pub nic_number: Option<String>,
    pub issued_date: Option<chrono::NaiveDate>,
    pub document_url: Option<String>,
}
