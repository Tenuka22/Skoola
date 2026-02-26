use apistos::ApiComponent;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize}; // Import JsonSchema

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)] // Add JsonSchema
#[serde(rename_all = "camelCase")]
pub struct BulkDeleteUsersRequest {
    pub user_ids: Vec<String>,
}
