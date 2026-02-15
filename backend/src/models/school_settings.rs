use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use apistos::ApiComponent;
use chrono::NaiveDateTime;

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
