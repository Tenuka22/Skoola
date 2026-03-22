use crate::models::resource_management::inventory_item::InventoryItemDetail;
use crate::services::resources::property::InventoryItemService;
use crate::models::resources::inventory::{UniformItem, UniformIssue};
use crate::models::resource_management::*;
use crate::services::resources::property::{
    AssetCategoryService, InventoryTransactionService,
    AssetMaintenanceLogService, MaintenanceRequestService, UniformItemService,
    UniformIssueService, AssetAllocationService, InventoryItemDetailService,
};
use crate::services::admin_db::AdminQuery;
use crate::create_admin_handlers;
use crate::services::resources::property;

create_admin_handlers!(
    tag => "inventory_item_details",
    entity => InventoryItemDetail,
    response => InventoryItemDetail,
    query => AdminQuery,
    create => InventoryItemDetail,
    update => InventoryItemDetail,
    service => InventoryItemDetailService
);

use actix_web::web::{Data, Json, Path};
use apistos::api_operation;
use apistos::web;
use crate::AppState;
use crate::errors::APIError;
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use crate::database::enums::PermissionEnum;

create_admin_handlers!(
    tag => "asset_categories",
    entity => AssetCategory,
    response => AssetCategory,
    query => AdminQuery,
    create => CreateAssetCategoryRequest,
    update => UpdateAssetCategoryRequest,
    service => AssetCategoryService
);

create_admin_handlers!(
    tag => "inventory_items",
    entity => InventoryItem,
    response => InventoryItem,
    query => AdminQuery,
    create => CreateInventoryItemRequest,
    update => UpdateInventoryItemRequest,
    service => InventoryItemService
);

create_admin_handlers!(
    tag => "inventory_transactions",
    entity => InventoryTransaction,
    response => InventoryTransaction,
    query => AdminQuery,
    create => CreateInventoryTransactionRequest,
    update => InventoryTransaction,
    service => InventoryTransactionService
);

create_admin_handlers!(
    tag => "asset_maintenance_logs",
    entity => AssetMaintenanceLog,
    response => AssetMaintenanceLog,
    query => AdminQuery,
    create => CreateAssetMaintenanceLogRequest,
    update => UpdateAssetMaintenanceLogRequest,
    service => AssetMaintenanceLogService
);

create_admin_handlers!(
    tag => "maintenance_requests",
    entity => MaintenanceRequest,
    response => MaintenanceRequest,
    query => AdminQuery,
    create => CreateMaintenanceRequest,
    update => UpdateMaintenanceStatusRequest,
    service => MaintenanceRequestService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
    }
);

create_admin_handlers!(
    tag => "uniform_items",
    entity => UniformItem,
    response => UniformItem,
    query => AdminQuery,
    create => CreateUniformItemRequest,
    update => UpdateUniformItemRequest,
    service => UniformItemService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
    }
);

create_admin_handlers!(
    tag => "uniform_issues",
    entity => UniformIssue,
    response => UniformIssue,
    query => AdminQuery,
    create => IssueUniformRequest,
    update => UniformIssue,
    service => UniformIssueService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update,
    }
);

create_admin_handlers!(
    tag => "asset_allocations",
    entity => AssetAllocation,
    response => AssetAllocation,
    query => AdminQuery,
    create => AllocateAssetRequest,
    update => ReturnAssetRequest,
    service => AssetAllocationService
);

#[api_operation(
    summary = "Update stock quantity",
    description = "Updates the available stock quantity for an inventory item.",
    tag = "property",
    operation_id = "update_stock_quantity_manual"
)]
pub async fn update_stock_quantity(
    data: Data<AppState>,
    path: Path<String>,
    body: Json<UpdateStockRequest>,
) -> Result<Json<InventoryItemResponse>, APIError> {
    let res = property::update_stock_quantity(data, path.into_inner(), body.into_inner()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get low stock items",
    description = "Retrieves all inventory items that are below their reorder level.",
    tag = "property",
    operation_id = "get_low_stock_items_manual"
)]
pub async fn get_low_stock_items(
    data: Data<AppState>,
) -> Result<Json<Vec<InventoryItemResponse>>, APIError> {
    let items = property::get_low_stock_items(data).await?;
    Ok(Json(items))
}

#[api_operation(
    summary = "Search inventory",
    description = "Searches for inventory items by name or description.",
    tag = "property",
    operation_id = "search_inventory_manual"
)]
pub async fn search_inventory(
    data: Data<AppState>,
    path: Path<String>,
) -> Result<Json<Vec<InventoryItemResponse>>, APIError> {
    let items = property::search_inventory(data, path.into_inner()).await?;
    Ok(Json(items))
}

#[api_operation(
    summary = "Get uniform issue history",
    description = "Retrieves the history of uniform issuance for a specific student.",
    tag = "property",
    operation_id = "get_uniform_history_manual"
)]
pub async fn get_uniform_history(
    data: Data<AppState>,
    path: Path<String>,
) -> Result<Json<Vec<UniformIssue>>, APIError> {
    let history = property::get_uniform_history(data, path.into_inner()).await?;
    Ok(Json(history))
}

#[api_operation(
    summary = "Get allocations by item",
    description = "Retrieves all current and past allocations for a specific inventory item.",
    tag = "property",
    operation_id = "get_allocations_by_item_manual"
)]
pub async fn get_allocations_by_item(
    data: Data<AppState>,
    path: Path<String>,
) -> Result<Json<Vec<DetailedAssetAllocationResponse>>, APIError> {
    let res = property::get_allocations_by_item(data, path.into_inner()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get allocations by assignee",
    description = "Retrieves all current and past asset allocations for a specific student or staff member.",
    tag = "property",
    operation_id = "get_allocations_by_assignee_manual"
)]
pub async fn get_allocations_by_assignee(
    data: Data<AppState>,
    path: Path<String>,
) -> Result<Json<Vec<DetailedAssetAllocationResponse>>, APIError> {
    let res = property::get_allocations_by_assignee(data, path.into_inner()).await?;
    Ok(Json(res))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/inventory-items")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::ResourceCreate })
            .route("", web::post().to(create_inventory_item))
            .route("/{id}", web::get().to(get_inventory_item_by_id))
            .route("", web::get().to(get_all_inventory_item))
            .route("/{id}", web::put().to(update_inventory_item))
            .route("/{id}", web::delete().to(delete_inventory_item))
            .route("/bulk", web::delete().to(bulk_delete_inventory_item)),
    )
    .service(
        web::scope("/inventory-movements")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::ResourceCreate })
            .route("", web::post().to(create_inventory_transaction))
            .route("/{id}", web::get().to(get_inventory_transaction_by_id))
            .route("", web::get().to(get_all_inventory_transaction))
            .route("/{id}", web::delete().to(delete_inventory_transaction))
            .route("/bulk", web::delete().to(bulk_delete_inventory_transaction)),
    )
    .service(
        web::scope("/asset-allocations")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::ResourceCreate })
            .route("", web::post().to(create_asset_allocation))
            .route("/{id}", web::get().to(get_asset_allocation_by_id))
            .route("", web::get().to(get_all_asset_allocation))
            .route("/{id}", web::put().to(update_asset_allocation))
            .route("/{id}", web::delete().to(delete_asset_allocation))
            .route("/bulk", web::delete().to(bulk_delete_asset_allocation)),
    )
    .service(
        web::scope("/property-ops")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::ResourceCreate })
            .route("/stock/update/{id}", web::post().to(update_stock_quantity))
            .route("/stock/low", web::get().to(get_low_stock_items))
            .route("/inventory/search/{query}", web::get().to(search_inventory))
            .route("/uniform/history/{student_id}", web::get().to(get_uniform_history))
            .route("/allocations/item/{item_id}", web::get().to(get_allocations_by_item))
            .route("/allocations/assignee/{assignee_id}", web::get().to(get_allocations_by_assignee)),
    );
}

