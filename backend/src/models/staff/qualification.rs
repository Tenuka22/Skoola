use crate::models::staff::staff::Staff;
use crate::schema::staff_qualifications;
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
#[diesel(table_name = staff_qualifications)]
#[diesel(belongs_to(Staff))]
pub struct StaffQualification {
    pub id: String,
    pub staff_id: String,
    pub degree: String,
    pub institution: String,
    pub year_of_completion: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub file_name: Option<String>,
    pub file_url: Option<String>,
    pub file_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = staff_qualifications)]
pub struct CreateStaffQualificationRequest {
    pub staff_id: String,
    pub degree: String,
    pub institution: String,
    pub year_of_completion: i32,
    pub file_name: Option<String>,
    pub file_url: Option<String>,
    pub file_type: Option<String>,
}

impl From<CreateStaffQualificationRequest> for StaffQualification {
    fn from(req: CreateStaffQualificationRequest) -> Self {
        StaffQualification {
            id: uuid::Uuid::new_v4().to_string(),
            staff_id: req.staff_id,
            degree: req.degree,
            institution: req.institution,
            year_of_completion: req.year_of_completion,
            created_at: chrono::Utc::now().naive_utc(),
            updated_at: chrono::Utc::now().naive_utc(),
            file_name: req.file_name,
            file_url: req.file_url,
            file_type: req.file_type,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = staff_qualifications)]
pub struct UpdateStaffQualificationRequest {
    pub degree: Option<String>,
    pub institution: Option<String>,
    pub year_of_completion: Option<i32>,
    pub file_name: Option<String>,
    pub file_url: Option<String>,
    pub file_type: Option<String>,
}
