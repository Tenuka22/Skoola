use crate::schema::student_emergency_contacts;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;

#[derive(
    Debug, Serialize, Deserialize, Clone, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent
)]
#[diesel(table_name = student_emergency_contacts)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StudentEmergencyContact {
    pub id: String,
    pub student_id: String,
    pub name: String,
    pub relationship: String,
    pub phone: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct StudentEmergencyContactQuery {
    pub search: Option<String>,
    pub student_id: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for StudentEmergencyContactQuery {
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
pub struct CreateStudentEmergencyContactRequest {
    pub student_id: String,
    pub name: String,
    pub relationship: String,
    pub phone: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = student_emergency_contacts)]
pub struct UpdateStudentEmergencyContactRequest {
    pub name: Option<String>,
    pub relationship: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema, ApiComponent)]
pub struct StudentEmergencyContactResponse {
    pub id: String,
    pub student_id: String,
    pub name: String,
    pub relationship: String,
    pub phone: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<StudentEmergencyContact> for StudentEmergencyContactResponse {
    fn from(contact: StudentEmergencyContact) -> Self {
        StudentEmergencyContactResponse {
            id: contact.id,
            student_id: contact.student_id,
            name: contact.name,
            relationship: contact.relationship,
            phone: contact.phone,
            created_at: contact.created_at,
            updated_at: contact.updated_at,
        }
    }
}
