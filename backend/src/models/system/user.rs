use serde::{Deserialize, Serialize};
use apistos::{ApiComponent};
use schemars::JsonSchema; // Import JsonSchema

#[derive(Debug, Clone, Serialize, Deserialize, ApiComponent, JsonSchema)] // Add JsonSchema
#[serde(rename_all = "camelCase")]
pub struct BulkDeleteUsersRequest {
    pub user_ids: Vec<String>,
}
