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
    AsChangeset,
    Clone,
    ApiComponent,
)]
#[diesel(table_name = school_settings)]
#[diesel(primary_key(setting_key))]
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

impl From<SchoolSetting> for SchoolSettingResponse {
    fn from(s: SchoolSetting) -> Self {
        Self {
            setting_key: s.setting_key,
            setting_value: s.setting_value,
            description: s.description,
            updated_at: s.updated_at,
        }
    }
}

impl From<CreateSchoolSettingRequest> for SchoolSetting {
    fn from(req: CreateSchoolSettingRequest) -> Self {
        Self {
            setting_key: req.setting_key,
            setting_value: req.setting_value,
            description: req.description,
            updated_at: chrono::Utc::now().naive_utc(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = school_settings)]
pub struct CreateSchoolSettingRequest {
    pub setting_key: String,
    pub setting_value: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = school_settings)]
pub struct UpdateSchoolSettingRequest {
    pub setting_value: String,
    pub description: Option<String>,
}
