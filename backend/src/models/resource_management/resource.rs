use crate::schema::{resource_assets, resource_details, resources};
use apistos::ApiComponent;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = resources)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Resource {
    pub id: String,
    pub resource_name: String,
    pub resource_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = resources)]
pub struct NewResource {
    pub id: String,
    pub resource_name: String,
    pub resource_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = resources)]
pub struct ResourceChangeset {
    pub resource_name: Option<String>,
    pub resource_type: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct ResourceQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for ResourceQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ResourceResponse {
    pub id: String,
    pub resource_name: String,
    pub resource_type: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<Resource> for ResourceResponse {
    fn from(resource: Resource) -> Self {
        Self {
            id: resource.id,
            resource_name: resource.resource_name,
            resource_type: resource.resource_type,
            created_at: resource.created_at,
            updated_at: resource.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateResourceRequest {
    pub resource_name: String,
    pub resource_type: String,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = resources)]
pub struct UpdateResourceRequest {
    pub resource_name: Option<String>,
    pub resource_type: Option<String>,
}

// Resource Asset
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = resource_assets)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ResourceAsset {
    pub id: String,
    pub resource_id: String,
    pub inventory_item_id: String,
    pub quantity: f32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = resource_assets)]
pub struct NewResourceAsset {
    pub id: String,
    pub resource_id: String,
    pub inventory_item_id: String,
    pub quantity: f32,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = resource_assets)]
pub struct ResourceAssetChangeset {
    pub resource_id: Option<String>,
    pub inventory_item_id: Option<String>,
    pub quantity: Option<f32>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct ResourceAssetQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for ResourceAssetQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ResourceAssetResponse {
    pub id: String,
    pub resource_id: String,
    pub inventory_item_id: String,
    pub quantity: f32,
    pub created_at: NaiveDateTime,
}

impl From<ResourceAsset> for ResourceAssetResponse {
    fn from(asset: ResourceAsset) -> Self {
        Self {
            id: asset.id,
            resource_id: asset.resource_id,
            inventory_item_id: asset.inventory_item_id,
            quantity: asset.quantity,
            created_at: asset.created_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateResourceAssetRequest {
    pub resource_id: String,
    pub inventory_item_id: String,
    pub quantity: f32,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = resource_assets)]
pub struct UpdateResourceAssetRequest {
    pub resource_id: Option<String>,
    pub inventory_item_id: Option<String>,
    pub quantity: Option<f32>,
}

// Resource Detail
#[derive(
    Debug,
    Serialize,
    Deserialize,
    Clone,
    Queryable,
    Selectable,
    Insertable,
    AsChangeset,
    JsonSchema,
    ApiComponent,
)]
#[diesel(table_name = resource_details)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct ResourceDetail {
    pub resource_id: String,
    pub description: Option<String>,
    pub status: String,
    pub location: Option<String>,
    pub capacity: Option<i32>,
    pub booking_policy: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, JsonSchema, ApiComponent)]
#[diesel(table_name = resource_details)]
pub struct NewResourceDetail {
    pub resource_id: String,
    pub description: Option<String>,
    pub status: String,
    pub location: Option<String>,
    pub capacity: Option<i32>,
    pub booking_policy: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = resource_details)]
pub struct ResourceDetailChangeset {
    pub description: Option<String>,
    pub status: Option<String>,
    pub location: Option<String>,
    pub capacity: Option<i32>,
    pub booking_policy: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct ResourceDetailQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

impl crate::services::admin_db::AsAdminQuery for ResourceDetailQuery {
    fn as_admin_query(&self) -> crate::services::admin_db::AdminQuery {
        crate::services::admin_db::AdminQuery {
            search: self.search.clone(),
            sort_by: self.sort_by.clone(),
            sort_order: self.sort_order.clone(),
            page: self.page,
            limit: self.limit,
            last_id: self.last_id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ResourceDetailResponse {
    pub resource_id: String,
    pub description: Option<String>,
    pub status: String,
    pub location: Option<String>,
    pub capacity: Option<i32>,
    pub booking_policy: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ResourceDetail> for ResourceDetailResponse {
    fn from(detail: ResourceDetail) -> Self {
        Self {
            resource_id: detail.resource_id,
            description: detail.description,
            status: detail.status,
            location: detail.location,
            capacity: detail.capacity,
            booking_policy: detail.booking_policy,
            created_at: detail.created_at,
            updated_at: detail.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateResourceDetailRequest {
    pub resource_id: String,
    pub description: Option<String>,
    pub status: String,
    pub location: Option<String>,
    pub capacity: Option<i32>,
    pub booking_policy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, AsChangeset, JsonSchema, ApiComponent)]
#[diesel(table_name = resource_details)]
pub struct UpdateResourceDetailRequest {
    pub description: Option<String>,
    pub status: Option<String>,
    pub location: Option<String>,
    pub capacity: Option<i32>,
    pub booking_policy: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct DetailedAssetAllocationResponse {
    pub allocation: crate::models::resource_management::AssetAllocationResponse,
    pub allocated_to_staff: Option<crate::models::staff::staff::StaffResponse>,
    pub allocated_to_student: Option<crate::models::student::student::StudentResponse>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct CreateUniformItemRequest {
    pub item_name: String,
    pub size: String,
    pub gender: String,
    pub grade_level: Option<String>,
    pub price: f32,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone, AsChangeset)]
#[diesel(table_name = crate::schema::uniform_items)]
pub struct UpdateUniformItemRequest {
    pub item_name: Option<String>,
    pub size: Option<String>,
    pub gender: Option<String>,
    pub grade_level: Option<String>,
    pub price: Option<f32>,
    pub quantity: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct IssueUniformRequest {
    pub student_id: String,
    pub uniform_item_id: String,
    pub quantity: i32,
    pub issued_by: String,
    pub amount_collected: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct UpdateStockRequest {
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct InventoryItemResponse {
    pub id: String,
    pub category_id: String,
    pub item_name: String,
    pub description: Option<String>,
    pub unit: String,
    pub quantity: i32,
    pub reorder_level: i32,
    pub unit_price: f32,
}
