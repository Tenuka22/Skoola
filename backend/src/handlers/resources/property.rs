use crate::AppState;
use crate::errors::APIError;
use crate::models::resources::inventory::{
    AssetCategoryResponse, CreateAssetCategoryRequest, CreateInventoryItemRequest,
    InventoryItemResponse, UpdateInventoryItemRequest, UpdateStockRequest,
    CreateUniformItemRequest, UniformItemResponse, IssueUniformRequest,
    UniformIssueResponse, AllocateAssetRequest, ReturnAssetRequest,
    AssetAllocationResponse, CreateMaintenanceRequest, UpdateMaintenanceStatusRequest,
    MaintenanceRequestResponse,
};
use crate::services::resources::property;
use actix_web::web::{Data, Json, Path};
use apistos::api_operation;
use apistos::web;

#[api_operation(
    summary = "Create asset category",
    description = "Creates a new category for assets and inventory items.",
    tag = "property",
    operation_id = "create_asset_category"
)]
pub async fn create_category(data: Data<AppState>, req: Json<CreateAssetCategoryRequest>) -> Result<Json<AssetCategoryResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let cat = property::create_category(&mut conn, req.into_inner())?;
    Ok(Json(AssetCategoryResponse::from(cat)))
}

#[api_operation(
    summary = "Get all asset categories",
    description = "Retrieves a list of all defined asset categories.",
    tag = "property",
    operation_id = "get_asset_categories"
)]
pub async fn get_categories(data: Data<AppState>) -> Result<Json<Vec<AssetCategoryResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let cats = property::get_categories(&mut conn)?;
    Ok(Json(cats.into_iter().map(AssetCategoryResponse::from).collect()))
}

#[api_operation(
    summary = "Add inventory item",
    description = "Creates a new inventory item in a specific category.",
    tag = "property",
    operation_id = "add_inventory_item"
)]
pub async fn add_inventory_item(data: Data<AppState>, req: Json<CreateInventoryItemRequest>) -> Result<Json<InventoryItemResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let item = property::create_inventory_item(&mut conn, req.into_inner())?;
    Ok(Json(InventoryItemResponse::from(item)))
}

#[api_operation(
    summary = "Get inventory by category",
    description = "Retrieves all inventory items belonging to a specific category.",
    tag = "property",
    operation_id = "get_inventory_by_category"
)]
pub async fn get_inventory_by_category(data: Data<AppState>, path: Path<String>) -> Result<Json<Vec<InventoryItemResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let items = property::get_inventory_by_category(&mut conn, &path.into_inner())?;
    Ok(Json(items.into_iter().map(InventoryItemResponse::from).collect()))
}

#[api_operation(
    summary = "Create uniform item",
    description = "Adds a new school uniform item to the inventory.",
    tag = "property",
    operation_id = "create_uniform_item"
)]
pub async fn create_uniform_item(data: Data<AppState>, req: Json<CreateUniformItemRequest>) -> Result<Json<UniformItemResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let item = property::create_uniform_item(&mut conn, req.into_inner())?;
    Ok(Json(UniformItemResponse::from(item)))
}

#[api_operation(
    summary = "Issue uniform to student",
    description = "Records the issuance of a uniform item to a student.",
    tag = "property",
    operation_id = "issue_uniform"
)]
pub async fn issue_uniform(data: Data<AppState>, req: Json<IssueUniformRequest>) -> Result<Json<UniformIssueResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let issue = property::issue_uniform(&mut conn, req.into_inner())?;
    Ok(Json(UniformIssueResponse::from(issue)))
}

#[api_operation(
    summary = "Allocate asset",
    description = "Allocates a specific asset to a student or staff member.",
    tag = "property",
    operation_id = "allocate_asset"
)]
pub async fn allocate_asset(data: Data<AppState>, req: Json<AllocateAssetRequest>) -> Result<Json<AssetAllocationResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let alloc = property::allocate_asset(&mut conn, req.into_inner())?;
    Ok(Json(AssetAllocationResponse::from(alloc.allocation)))
}

#[api_operation(
    summary = "Create maintenance request",
    description = "Submits a new request for asset maintenance or repair.",
    tag = "property",
    operation_id = "create_maintenance_request"
)]
pub async fn create_maintenance_request(data: Data<AppState>, req: Json<CreateMaintenanceRequest>) -> Result<Json<MaintenanceRequestResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let m_req = property::create_maintenance_request(&mut conn, req.into_inner())?;
    Ok(Json(MaintenanceRequestResponse::from(m_req)))
}

#[api_operation(
    summary = "Update inventory item",
    description = "Updates the details of an existing inventory item.",
    tag = "property",
    operation_id = "update_inventory_item"
)]
pub async fn update_inventory_item(data: Data<AppState>, path: Path<String>, req: Json<UpdateInventoryItemRequest>) -> Result<Json<InventoryItemResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let item = property::update_inventory_item(&mut conn, &path.into_inner(), req.into_inner())?;
    Ok(Json(InventoryItemResponse::from(item)))
}

#[api_operation(
    summary = "Update stock quantity",
    description = "Updates the available stock quantity for an inventory item.",
    tag = "property",
    operation_id = "update_stock_quantity"
)]
pub async fn update_stock_quantity(data: Data<AppState>, path: Path<String>, req: Json<UpdateStockRequest>) -> Result<Json<InventoryItemResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let item = property::update_stock_quantity(&mut conn, &path.into_inner(), req.into_inner())?;
    Ok(Json(InventoryItemResponse::from(item)))
}

#[api_operation(
    summary = "Get low stock items",
    description = "Retrieves all inventory items that are below their reorder level.",
    tag = "property",
    operation_id = "get_low_stock_items"
)]
pub async fn get_low_stock_items(data: Data<AppState>) -> Result<Json<Vec<InventoryItemResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let items = property::get_low_stock_items(&mut conn)?;
    Ok(Json(items.into_iter().map(InventoryItemResponse::from).collect()))
}

#[api_operation(
    summary = "Search inventory",
    description = "Searches for inventory items by name or description.",
    tag = "property",
    operation_id = "search_inventory"
)]
pub async fn search_inventory(data: Data<AppState>, path: Path<String>) -> Result<Json<Vec<InventoryItemResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let items = property::search_inventory(&mut conn, &path.into_inner())?;
    Ok(Json(items.into_iter().map(InventoryItemResponse::from).collect()))
}

#[api_operation(
    summary = "Get uniform issue history",
    description = "Retrieves the history of uniform issuance for a specific student.",
    tag = "property",
    operation_id = "get_uniform_issue_history"
)]
pub async fn get_uniform_history(data: Data<AppState>, path: Path<String>) -> Result<Json<Vec<UniformIssueResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let items = property::get_uniform_issue_history(&mut conn, &path.into_inner())?;
    Ok(Json(items.into_iter().map(UniformIssueResponse::from).collect()))
}

#[api_operation(
    summary = "Get uniform inventory",
    description = "Retrieves a list of all school uniform items and their stock status.",
    tag = "property",
    operation_id = "get_uniform_inventory"
)]
pub async fn get_uniform_inventory(data: Data<AppState>) -> Result<Json<Vec<UniformItemResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let items = property::get_uniform_inventory(&mut conn)?;
    Ok(Json(items.into_iter().map(UniformItemResponse::from).collect()))
}

#[api_operation(
    summary = "Return allocated asset",
    description = "Records the return of an allocated asset.",
    tag = "property",
    operation_id = "return_allocated_asset"
)]
pub async fn return_asset(data: Data<AppState>, path: Path<String>, req: Json<ReturnAssetRequest>) -> Result<Json<AssetAllocationResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let alloc = property::return_asset(&mut conn, &path.into_inner(), req.into_inner())?;
    Ok(Json(AssetAllocationResponse::from(alloc.allocation)))
}

#[api_operation(
    summary = "Update maintenance status",
    description = "Updates the status of an existing maintenance request.",
    tag = "property",
    operation_id = "update_maintenance_status"
)]
pub async fn update_maintenance_status(data: Data<AppState>, path: Path<String>, req: Json<UpdateMaintenanceStatusRequest>) -> Result<Json<MaintenanceRequestResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let m_req = property::update_maintenance_status(&mut conn, &path.into_inner(), req.into_inner())?;
    Ok(Json(MaintenanceRequestResponse::from(m_req)))
}

#[api_operation(
    summary = "Get pending maintenance",
    description = "Retrieves all maintenance requests that are currently pending.",
    tag = "property",
    operation_id = "get_pending_maintenance"
)]
pub async fn get_pending_maintenance(data: Data<AppState>) -> Result<Json<Vec<MaintenanceRequestResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let items = property::get_pending_maintenance(&mut conn)?;
    Ok(Json(items.into_iter().map(MaintenanceRequestResponse::from).collect()))
}

#[api_operation(
    summary = "Get allocations by item",
    description = "Retrieves all current and past allocations for a specific inventory item.",
    tag = "property",
    operation_id = "get_allocations_by_item"
)]
pub async fn get_allocations_by_item(data: Data<AppState>, path: Path<String>) -> Result<Json<Vec<AssetAllocationResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let items = property::get_allocations_by_item(&mut conn, &path.into_inner())?;
    Ok(Json(items.into_iter().map(AssetAllocationResponse::from).collect()))
}

#[api_operation(
    summary = "Get allocations by assignee",
    description = "Retrieves all current and past asset allocations for a specific student or staff member.",
    tag = "property",
    operation_id = "get_allocations_by_assignee"
)]
pub async fn get_allocations_by_assignee(data: Data<AppState>, path: Path<String>) -> Result<Json<Vec<crate::models::resources::inventory::DetailedAssetAllocationResponse>>, APIError> {
    let mut conn = data.db_pool.get()?;
    let items = property::get_detailed_allocations_by_assignee(&mut conn, &path.into_inner())?;
    Ok(Json(items))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/property")
            .route("/categories", web::post().to(create_category))
            .route("/categories", web::get().to(get_categories))
            .route("/inventory", web::post().to(add_inventory_item))
            .route("/inventory/low-stock", web::get().to(get_low_stock_items))
            .route("/inventory/search/{query}", web::get().to(search_inventory))
            .route("/inventory/category/{id}", web::get().to(get_inventory_by_category))
            .route("/inventory/{id}", web::patch().to(update_inventory_item))
            .route("/inventory/{id}/stock", web::patch().to(update_stock_quantity))
            .route("/uniforms", web::post().to(create_uniform_item))
            .route("/uniforms", web::get().to(get_uniform_inventory))
            .route("/uniforms/issue", web::post().to(issue_uniform))
            .route("/uniforms/history/{student_id}", web::get().to(get_uniform_history))
            .route("/allocations", web::post().to(allocate_asset))
            .route("/allocations/item/{id}", web::get().to(get_allocations_by_item))
            .route("/allocations/assignee/{id}", web::get().to(get_allocations_by_assignee))
            .route("/allocations/{id}/return", web::post().to(return_asset))
            .route("/maintenance", web::post().to(create_maintenance_request))
            .route("/maintenance/pending", web::get().to(get_pending_maintenance))
            .route("/maintenance/{id}/status", web::patch().to(update_maintenance_status)),
    );
}
