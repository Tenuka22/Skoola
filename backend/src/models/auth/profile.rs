use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct UpdateProfileRequest {
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct ChangePasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct ChangeEmailRequest {
    pub new_email: String,
    pub password: String,
}
