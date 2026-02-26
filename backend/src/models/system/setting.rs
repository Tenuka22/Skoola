use crate::schema::school_settings;
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
    Clone,
    ApiComponent,
)]
#[diesel(table_name = school_settings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct SchoolSetting {
    pub setting_key: String,
    pub setting_value: String,
    pub description: Option<String>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct SchoolSettingResponse {
    pub setting_key: String,
    pub setting_value: String,
    pub description: Option<String>,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct UpdateSchoolSettingRequest {
    pub setting_value: String,
    pub description: Option<String>,
}
