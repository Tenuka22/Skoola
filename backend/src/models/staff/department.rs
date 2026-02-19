use crate::schema::staff_departments;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use schemars::JsonSchema;
use apistos::ApiComponent;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = staff_departments)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct StaffDepartment {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
