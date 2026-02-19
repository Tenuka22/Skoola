use crate::database::enums::{AllocationType, MaintenanceStatus};
use crate::schema::{asset_categories, inventory_items, uniform_items, uniform_issues, asset_allocations, maintenance_requests};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;
use apistos::ApiComponent;
use diesel::prelude::*;
use crate::models::student::student::Student;
use crate::models::staff::staff::Staff;

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = asset_categories)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AssetCategory {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    Clone,
    Associations,
    ApiComponent
)]
#[diesel(table_name = inventory_items)]
#[diesel(belongs_to(AssetCategory, foreign_key = category_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct InventoryItem {
    pub id: String,
    pub category_id: String,
    pub item_name: String,
    pub description: Option<String>,
    pub unit: String,
    pub quantity: i32,
    pub reorder_level: i32,
    pub unit_price: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Queryable, Selectable, Insertable, Clone, ApiComponent)]
#[diesel(table_name = uniform_items)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UniformItem {
    pub id: String,
    pub item_name: String,
    pub size: String,
    pub gender: String,
    pub grade_level: Option<String>,
    pub price: f32,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    Clone,
    Associations,
    ApiComponent
)]
#[diesel(table_name = uniform_issues)]
#[diesel(belongs_to(Student))]
#[diesel(belongs_to(UniformItem))]
#[diesel(belongs_to(Staff, foreign_key = issued_by))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UniformIssue {
    pub id: String,
    pub student_id: String,
    pub uniform_item_id: String,
    pub quantity: i32,
    pub issue_date: NaiveDateTime,
    pub issued_by: String,
    pub amount_collected: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    Clone,
    Associations,
    ApiComponent
)]
#[diesel(table_name = asset_allocations)]
#[diesel(belongs_to(InventoryItem, foreign_key = item_id))]
#[diesel(belongs_to(Staff, foreign_key = allocated_by))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct AssetAllocation {
    pub id: String,
    pub item_id: String,
    pub allocated_to_type: AllocationType,
    pub allocated_to_id: String,
    pub quantity: i32,
    pub allocation_date: NaiveDateTime,
    pub return_date: Option<NaiveDateTime>,
    pub allocated_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(
    Debug,
    Serialize,
    Deserialize,
    JsonSchema,
    Queryable,
    Selectable,
    Insertable,
    Clone,
    Associations,
    ApiComponent
)]
#[diesel(table_name = maintenance_requests)]
#[diesel(belongs_to(InventoryItem, foreign_key = item_id))]
#[diesel(belongs_to(Staff, foreign_key = reported_by))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct MaintenanceRequest {
    pub id: String,
    pub item_id: String,
    pub issue_description: String,
    pub reported_by: String,
    pub reported_date: NaiveDateTime,
    pub status: MaintenanceStatus,
    pub assigned_to: Option<String>,
    pub resolved_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateAssetCategoryRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AssetCategoryResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<AssetCategory> for AssetCategoryResponse {
    fn from(cat: AssetCategory) -> Self {
        Self {
            id: cat.id,
            name: cat.name,
            description: cat.description,
            created_at: cat.created_at,
            updated_at: cat.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateInventoryItemRequest {
    pub category_id: String,
    pub item_name: String,
    pub description: Option<String>,
    pub unit: String,
    pub quantity: i32,
    pub reorder_level: i32,
    pub unit_price: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct UpdateInventoryItemRequest {
    pub item_name: Option<String>,
    pub description: Option<String>,
    pub unit: Option<String>,
    pub reorder_level: Option<i32>,
    pub unit_price: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct UpdateStockRequest {
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct InventoryItemResponse {
    pub id: String,
    pub category_id: String,
    pub item_name: String,
    pub description: Option<String>,
    pub unit: String,
    pub quantity: i32,
    pub reorder_level: i32,
    pub unit_price: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<InventoryItem> for InventoryItemResponse {
    fn from(item: InventoryItem) -> Self {
        Self {
            id: item.id,
            category_id: item.category_id,
            item_name: item.item_name,
            description: item.description,
            unit: item.unit,
            quantity: item.quantity,
            reorder_level: item.reorder_level,
            unit_price: item.unit_price,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateUniformItemRequest {
    pub item_name: String,
    pub size: String,
    pub gender: String,
    pub grade_level: Option<String>,
    pub price: f32,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct UniformItemResponse {
    pub id: String,
    pub item_name: String,
    pub size: String,
    pub gender: String,
    pub grade_level: Option<String>,
    pub price: f32,
    pub quantity: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<UniformItem> for UniformItemResponse {
    fn from(item: UniformItem) -> Self {
        Self {
            id: item.id,
            item_name: item.item_name,
            size: item.size,
            gender: item.gender,
            grade_level: item.grade_level,
            price: item.price,
            quantity: item.quantity,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct IssueUniformRequest {
    pub student_id: String,
    pub uniform_item_id: String,
    pub quantity: i32,
    pub issued_by: String,
    pub amount_collected: f32,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct UniformIssueResponse {
    pub id: String,
    pub student_id: String,
    pub uniform_item_id: String,
    pub quantity: i32,
    pub issue_date: NaiveDateTime,
    pub issued_by: String,
    pub amount_collected: f32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<UniformIssue> for UniformIssueResponse {
    fn from(issue: UniformIssue) -> Self {
        Self {
            id: issue.id,
            student_id: issue.student_id,
            uniform_item_id: issue.uniform_item_id,
            quantity: issue.quantity,
            issue_date: issue.issue_date,
            issued_by: issue.issued_by,
            amount_collected: issue.amount_collected,
            created_at: issue.created_at,
            updated_at: issue.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AllocateAssetRequest {
    pub item_id: String,
    pub allocated_to_type: AllocationType,
    pub allocated_to_id: String,
    pub quantity: i32,
    pub allocated_by: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct ReturnAssetRequest {
    pub return_date: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct AssetAllocationResponse {
    pub id: String,
    pub item_id: String,
    pub allocated_to_type: AllocationType,
    pub allocated_to_id: String,
    pub quantity: i32,
    pub allocation_date: NaiveDateTime,
    pub return_date: Option<NaiveDateTime>,
    pub allocated_by: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<AssetAllocation> for AssetAllocationResponse {
    fn from(alloc: AssetAllocation) -> Self {
        Self {
            id: alloc.id,
            item_id: alloc.item_id,
            allocated_to_type: alloc.allocated_to_type,
            allocated_to_id: alloc.allocated_to_id,
            quantity: alloc.quantity,
            allocation_date: alloc.allocation_date,
            return_date: alloc.return_date,
            allocated_by: alloc.allocated_by,
            created_at: alloc.created_at,
            updated_at: alloc.updated_at,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct CreateMaintenanceRequest {
    pub item_id: String,
    pub issue_description: String,
    pub reported_by: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct UpdateMaintenanceStatusRequest {
    pub status: MaintenanceStatus,
    pub assigned_to: Option<String>,
    pub resolved_date: Option<NaiveDateTime>,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct MaintenanceRequestResponse {
    pub id: String,
    pub item_id: String,
    pub issue_description: String,
    pub reported_by: String,
    pub reported_date: NaiveDateTime,
    pub status: MaintenanceStatus,
    pub assigned_to: Option<String>,
    pub resolved_date: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<MaintenanceRequest> for MaintenanceRequestResponse {
    fn from(req: MaintenanceRequest) -> Self {
        Self {
            id: req.id,
            item_id: req.item_id,
            issue_description: req.issue_description,
            reported_by: req.reported_by,
            reported_date: req.reported_date,
            status: req.status,
            assigned_to: req.assigned_to,
            resolved_date: req.resolved_date,
            created_at: req.created_at,
            updated_at: req.updated_at,
        }
    }
}
