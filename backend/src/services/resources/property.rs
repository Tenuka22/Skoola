use crate::database::tables::{AssetCategory, InventoryItem, UniformItem, UniformIssue, AssetAllocation, MaintenanceRequest};
use crate::errors::APIError;
use crate::models::property::*;
use crate::schema::{asset_categories, inventory_items, uniform_items, uniform_issues, asset_allocations, maintenance_requests};
use crate::database::enums::MaintenanceStatus;
use diesel::prelude::*;
use diesel::SqliteConnection;
use uuid::Uuid;
use chrono::Utc;

pub fn create_category(conn: &mut SqliteConnection, req: CreateAssetCategoryRequest) -> Result<AssetCategory, APIError> {
    let new_cat = AssetCategory {
        id: Uuid::new_v4().to_string(),
        name: req.name,
        description: req.description,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    diesel::insert_into(asset_categories::table).values(&new_cat).execute(conn)?;
    Ok(new_cat)
}

pub fn get_categories(conn: &mut SqliteConnection) -> Result<Vec<AssetCategory>, APIError> {
    Ok(asset_categories::table.load::<AssetCategory>(conn)?)
}

pub fn create_inventory_item(conn: &mut SqliteConnection, req: CreateInventoryItemRequest) -> Result<InventoryItem, APIError> {
    let new_item = InventoryItem {
        id: Uuid::new_v4().to_string(),
        category_id: req.category_id,
        item_name: req.item_name,
        description: req.description,
        unit: req.unit,
        quantity: req.quantity,
        reorder_level: req.reorder_level,
        unit_price: req.unit_price,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    diesel::insert_into(inventory_items::table).values(&new_item).execute(conn)?;
    Ok(new_item)
}

pub fn get_inventory_by_category(conn: &mut SqliteConnection, cat_id: &str) -> Result<Vec<InventoryItem>, APIError> {
    Ok(inventory_items::table.filter(inventory_items::category_id.eq(cat_id)).load::<InventoryItem>(conn)?)
}

pub fn create_uniform_item(conn: &mut SqliteConnection, req: CreateUniformItemRequest) -> Result<UniformItem, APIError> {
    let new_item = UniformItem {
        id: Uuid::new_v4().to_string(),
        item_name: req.item_name,
        size: req.size,
        gender: req.gender,
        grade_level: req.grade_level,
        price: req.price,
        quantity: req.quantity,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    diesel::insert_into(uniform_items::table).values(&new_item).execute(conn)?;
    Ok(new_item)
}

pub fn issue_uniform(conn: &mut SqliteConnection, req: IssueUniformRequest) -> Result<UniformIssue, APIError> {
    let new_issue = UniformIssue {
        id: Uuid::new_v4().to_string(),
        student_id: req.student_id,
        uniform_item_id: req.uniform_item_id,
        quantity: req.quantity,
        issue_date: Utc::now().naive_utc(),
        issued_by: req.issued_by,
        amount_collected: req.amount_collected,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    diesel::insert_into(uniform_issues::table).values(&new_issue).execute(conn)?;
    
    // Update stock
    diesel::update(uniform_items::table.filter(uniform_items::id.eq(&new_issue.uniform_item_id)))
        .set(uniform_items::quantity.eq(uniform_items::quantity - new_issue.quantity))
        .execute(conn)
        ?;

    Ok(new_issue)
}

pub fn allocate_asset(conn: &mut SqliteConnection, req: AllocateAssetRequest) -> Result<AssetAllocation, APIError> {
    let new_alloc = AssetAllocation {
        id: Uuid::new_v4().to_string(),
        item_id: req.item_id,
        allocated_to_type: req.allocated_to_type,
        allocated_to_id: req.allocated_to_id,
        quantity: req.quantity,
        allocation_date: Utc::now().naive_utc(),
        return_date: None,
        allocated_by: req.allocated_by,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    diesel::insert_into(asset_allocations::table).values(&new_alloc).execute(conn)?;
    
    // Update stock
    diesel::update(inventory_items::table.filter(inventory_items::id.eq(&new_alloc.item_id)))
        .set(inventory_items::quantity.eq(inventory_items::quantity - new_alloc.quantity))
        .execute(conn)
        ?;

    Ok(new_alloc)
}

pub fn create_maintenance_request(conn: &mut SqliteConnection, req: CreateMaintenanceRequest) -> Result<MaintenanceRequest, APIError> {
    let new_req = MaintenanceRequest {
        id: Uuid::new_v4().to_string(),
        item_id: req.item_id,
        issue_description: req.issue_description,
        reported_by: req.reported_by,
        reported_date: Utc::now().naive_utc(),
        status: MaintenanceStatus::Pending,
        assigned_to: None,
        resolved_date: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    diesel::insert_into(maintenance_requests::table).values(&new_req).execute(conn)?;
    Ok(new_req)
}

pub fn update_inventory_item(conn: &mut SqliteConnection, id: &str, req: UpdateInventoryItemRequest) -> Result<InventoryItem, APIError> {
    diesel::update(inventory_items::table.filter(inventory_items::id.eq(id)))
        .set((
            req.item_name.map(|v| inventory_items::item_name.eq(v)),
            req.description.map(|v| inventory_items::description.eq(v)),
            req.unit.map(|v| inventory_items::unit.eq(v)),
            req.reorder_level.map(|v| inventory_items::reorder_level.eq(v)),
            req.unit_price.map(|v| inventory_items::unit_price.eq(v)),
            Some(inventory_items::updated_at.eq(Utc::now().naive_utc())),
        ))
        .execute(conn)
        ?;
    
    Ok(inventory_items::table.find(id).first(conn)?)
}

pub fn update_stock_quantity(conn: &mut SqliteConnection, id: &str, req: UpdateStockRequest) -> Result<InventoryItem, APIError> {
    diesel::update(inventory_items::table.filter(inventory_items::id.eq(id)))
        .set((
            inventory_items::quantity.eq(req.quantity),
            inventory_items::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(conn)
        ?;
    
    Ok(inventory_items::table.find(id).first(conn)?)
}

pub fn get_low_stock_items(conn: &mut SqliteConnection) -> Result<Vec<InventoryItem>, APIError> {
    Ok(inventory_items::table
        .filter(inventory_items::quantity.le(inventory_items::reorder_level))
        .load::<InventoryItem>(conn)?)
}

pub fn search_inventory(conn: &mut SqliteConnection, query: &str) -> Result<Vec<InventoryItem>, APIError> {
    Ok(inventory_items::table
        .filter(inventory_items::item_name.like(format!("%{}%", query)))
        .load::<InventoryItem>(conn)?)
}

pub fn get_uniform_issue_history(conn: &mut SqliteConnection, student_id: &str) -> Result<Vec<UniformIssue>, APIError> {
    Ok(uniform_issues::table
        .filter(uniform_issues::student_id.eq(student_id))
        .load::<UniformIssue>(conn)?)
}

pub fn get_uniform_inventory(conn: &mut SqliteConnection) -> Result<Vec<UniformItem>, APIError> {
    Ok(uniform_items::table
        .load::<UniformItem>(conn)?)
}

pub fn return_asset(conn: &mut SqliteConnection, id: &str, req: ReturnAssetRequest) -> Result<AssetAllocation, APIError> {
    let now = Utc::now().naive_utc();
    let return_date = req.return_date.unwrap_or(now);
    
    let alloc: AssetAllocation = asset_allocations::table.find(id).first(conn)?;
    
    diesel::update(asset_allocations::table.find(id))
        .set((
            asset_allocations::return_date.eq(return_date),
            asset_allocations::updated_at.eq(now),
        ))
        .execute(conn)
        ?;
        
    // Return to stock
    diesel::update(inventory_items::table.find(&alloc.item_id))
        .set(inventory_items::quantity.eq(inventory_items::quantity + alloc.quantity))
        .execute(conn)
        ?;
        
    Ok(asset_allocations::table.find(id).first(conn)?)
}

pub fn get_allocations_by_item(conn: &mut SqliteConnection, item_id: &str) -> Result<Vec<AssetAllocation>, APIError> {
    Ok(asset_allocations::table
        .filter(asset_allocations::item_id.eq(item_id))
        .load::<AssetAllocation>(conn)?)
}

pub fn get_allocations_by_assignee(conn: &mut SqliteConnection, assignee_id: &str) -> Result<Vec<AssetAllocation>, APIError> {
    Ok(asset_allocations::table
        .filter(asset_allocations::allocated_to_id.eq(assignee_id))
        .load::<AssetAllocation>(conn)?)
}

pub fn update_maintenance_status(conn: &mut SqliteConnection, id: &str, req: UpdateMaintenanceStatusRequest) -> Result<MaintenanceRequest, APIError> {
    diesel::update(maintenance_requests::table.find(id))
        .set((
            maintenance_requests::status.eq(req.status),
            maintenance_requests::assigned_to.eq(req.assigned_to),
            maintenance_requests::resolved_date.eq(req.resolved_date),
            maintenance_requests::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(conn)
        ?;
        
    Ok(maintenance_requests::table.find(id).first(conn)?)
}

pub fn get_pending_maintenance(conn: &mut SqliteConnection) -> Result<Vec<MaintenanceRequest>, APIError> {
    Ok(maintenance_requests::table
        .filter(maintenance_requests::status.eq(MaintenanceStatus::Pending))
        .load::<MaintenanceRequest>(conn)?)
}
