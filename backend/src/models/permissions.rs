use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CreatePermissionRequest {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct UpdatePermissionRequest {
    pub name: String,
}
