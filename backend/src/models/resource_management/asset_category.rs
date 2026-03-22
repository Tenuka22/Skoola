use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use apistos::ApiComponent;
use schemars::JsonSchema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = crate::schema::asset_categories)]
pub struct AssetCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateAssetCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent, Clone)]
#[diesel(table_name = crate::schema::asset_categories)]
pub struct UpdateAssetCategoryRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl From<CreateAssetCategoryRequest> for AssetCategory {
    fn from(req: CreateAssetCategoryRequest) -> Self {
        let now = chrono::Utc::now().naive_utc();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: req.name,
            description: req.description,
            created_at: now,
            updated_at: now,
        }
    }
}
