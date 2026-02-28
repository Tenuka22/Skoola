use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct CreateRoleRequest {
    pub name: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct UpdateRoleRequest {
    pub name: String,
    pub parent_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct RoleSetGetRoleResponse {
    pub roles: Vec<String>,
}
