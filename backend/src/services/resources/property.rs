use crate::database::enums::MaintenanceStatus;
use crate::errors::APIError;
use crate::models::resources::inventory::{
    AllocateAssetRequest, AssetAllocation, AssetAllocationResponse, AssetAllocationsStaff,
    AssetAllocationsStudents, AssetCategory, CreateAssetCategoryRequest,
    CreateInventoryItemRequest, CreateMaintenanceRequest, CreateUniformItemRequest,
    DetailedAssetAllocationResponse, InventoryItem, IssueUniformRequest, MaintenanceRequest,
    NewAssetAllocationsStaff, NewAssetAllocationsStudents, ReturnAssetRequest, UniformIssue,
    UniformItem, UpdateInventoryItemRequest, UpdateMaintenanceStatusRequest, UpdateStockRequest,
};
use crate::models::staff::staff::{Staff, StaffResponse};
use crate::models::student::student::{Student, StudentResponse};
use crate::schema::{
    asset_allocations, asset_allocations_staff, asset_allocations_students, asset_categories,
    inventory_items, maintenance_requests, staff, students, uniform_issues, uniform_items,
};
use chrono::Utc;
use diesel::SqliteConnection;
use diesel::prelude::*;
use uuid::Uuid;

pub fn create_category(
    conn: &mut SqliteConnection,
    req: CreateAssetCategoryRequest,
) -> Result<AssetCategory, APIError> {
    let new_cat = AssetCategory {
        id: Uuid::new_v4().to_string(),
        name: req.name,
        description: req.description,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    diesel::insert_into(asset_categories::table)
        .values(&new_cat)
        .execute(conn)?;
    Ok(new_cat)
}

pub fn get_categories(conn: &mut SqliteConnection) -> Result<Vec<AssetCategory>, APIError> {
    Ok(asset_categories::table.load::<AssetCategory>(conn)?)
}

pub fn create_inventory_item(
    conn: &mut SqliteConnection,
    req: CreateInventoryItemRequest,
) -> Result<InventoryItem, APIError> {
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
    diesel::insert_into(inventory_items::table)
        .values(&new_item)
        .execute(conn)?;
    Ok(new_item)
}

pub fn get_inventory_by_category(
    conn: &mut SqliteConnection,
    cat_id: &str,
) -> Result<Vec<InventoryItem>, APIError> {
    Ok(inventory_items::table
        .filter(inventory_items::category_id.eq(cat_id))
        .load::<InventoryItem>(conn)?)
}

pub fn create_uniform_item(
    conn: &mut SqliteConnection,
    req: CreateUniformItemRequest,
) -> Result<UniformItem, APIError> {
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
    diesel::insert_into(uniform_items::table)
        .values(&new_item)
        .execute(conn)?;
    Ok(new_item)
}

pub fn issue_uniform(
    conn: &mut SqliteConnection,
    req: IssueUniformRequest,
) -> Result<UniformIssue, APIError> {
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
    diesel::insert_into(uniform_issues::table)
        .values(&new_issue)
        .execute(conn)?;

    // Update stock
    diesel::update(uniform_items::table.filter(uniform_items::id.eq(&new_issue.uniform_item_id)))
        .set(uniform_items::quantity.eq(uniform_items::quantity - new_issue.quantity))
        .execute(conn)?;

    Ok(new_issue)
}

pub fn allocate_asset(
    conn: &mut SqliteConnection,
    req: AllocateAssetRequest,
) -> Result<DetailedAssetAllocationResponse, APIError> {
    if req.staff_id.is_some() && req.student_id.is_some() {
        return Err(APIError::bad_request(
            "Cannot allocate to both staff and student.",
        ));
    }
    if req.staff_id.is_none() && req.student_id.is_none() {
        return Err(APIError::bad_request(
            "Must allocate to either staff or student.",
        ));
    }

    let now = Utc::now().naive_utc();
    let new_alloc_id = Uuid::new_v4().to_string();

    let allocated_to_type = if req.staff_id.is_some() {
        "STAFF".to_string()
    } else {
        "STUDENT".to_string()
    };
    let allocated_to_id = req.staff_id.clone().or(req.student_id.clone()).unwrap();

    let new_alloc = AssetAllocation {
        id: new_alloc_id.clone(),
        item_id: req.item_id.clone(),
        allocated_to_type,
        allocated_to_id,
        quantity: req.quantity,
        allocation_date: now,
        return_date: None,
        allocated_by: req.allocated_by.clone(),
        created_at: now,
        updated_at: now,
    };

    diesel::insert_into(asset_allocations::table)
        .values(&new_alloc)
        .execute(conn)?;

    let mut allocated_to_staff: Option<StaffResponse> = None;
    let mut allocated_to_student: Option<StudentResponse> = None;

    if let Some(staff_id) = req.staff_id {
        let new_junction = NewAssetAllocationsStaff {
            asset_allocation_id: new_alloc_id.clone(),
            staff_id: staff_id.clone(),
            created_at: now,
        };
        diesel::insert_into(asset_allocations_staff::table)
            .values(&new_junction)
            .execute(conn)?;

        let staff_obj: Staff = staff::table
            .find(staff_id)
            .select(Staff::as_select())
            .first(conn)?;
        allocated_to_staff = Some(staff_obj.into()); // Assuming into() converts Staff to StaffResponse
    } else if let Some(student_id) = req.student_id {
        let new_junction = NewAssetAllocationsStudents {
            asset_allocation_id: new_alloc_id.clone(),
            student_id: student_id.clone(),
            created_at: now,
        };
        diesel::insert_into(asset_allocations_students::table)
            .values(&new_junction)
            .execute(conn)?;

        let student_obj: Student = students::table
            .find(student_id)
            .select(Student::as_select())
            .first(conn)?;
        allocated_to_student = Some(student_obj.into()); // Assuming into() converts Student to StudentResponse
    }

    // Update stock
    diesel::update(inventory_items::table.filter(inventory_items::id.eq(&new_alloc.item_id)))
        .set(inventory_items::quantity.eq(inventory_items::quantity - new_alloc.quantity))
        .execute(conn)?;

    let allocation_response = AssetAllocationResponse::from(new_alloc);

    Ok(DetailedAssetAllocationResponse {
        allocation: allocation_response,
        allocated_to_staff,
        allocated_to_student,
    })
}

pub fn create_maintenance_request(
    conn: &mut SqliteConnection,
    req: CreateMaintenanceRequest,
) -> Result<MaintenanceRequest, APIError> {
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
    diesel::insert_into(maintenance_requests::table)
        .values(&new_req)
        .execute(conn)?;
    Ok(new_req)
}

pub fn update_inventory_item(
    conn: &mut SqliteConnection,
    id: &str,
    req: UpdateInventoryItemRequest,
) -> Result<InventoryItem, APIError> {
    diesel::update(inventory_items::table.filter(inventory_items::id.eq(id)))
        .set((
            req.item_name.map(|v| inventory_items::item_name.eq(v)),
            req.description.map(|v| inventory_items::description.eq(v)),
            req.unit.map(|v| inventory_items::unit.eq(v)),
            req.reorder_level
                .map(|v| inventory_items::reorder_level.eq(v)),
            req.unit_price.map(|v| inventory_items::unit_price.eq(v)),
            Some(inventory_items::updated_at.eq(Utc::now().naive_utc())),
        ))
        .execute(conn)?;

    Ok(inventory_items::table.find(id).first(conn)?)
}

pub fn update_stock_quantity(
    conn: &mut SqliteConnection,
    id: &str,
    req: UpdateStockRequest,
) -> Result<InventoryItem, APIError> {
    diesel::update(inventory_items::table.filter(inventory_items::id.eq(id)))
        .set((
            inventory_items::quantity.eq(req.quantity),
            inventory_items::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(conn)?;

    Ok(inventory_items::table.find(id).first(conn)?)
}

pub fn get_low_stock_items(conn: &mut SqliteConnection) -> Result<Vec<InventoryItem>, APIError> {
    Ok(inventory_items::table
        .filter(inventory_items::quantity.le(inventory_items::reorder_level))
        .load::<InventoryItem>(conn)?)
}

pub fn search_inventory(
    conn: &mut SqliteConnection,
    query: &str,
) -> Result<Vec<InventoryItem>, APIError> {
    Ok(inventory_items::table
        .filter(inventory_items::item_name.like(format!("%{}%", query)))
        .load::<InventoryItem>(conn)?)
}

pub fn get_uniform_issue_history(
    conn: &mut SqliteConnection,
    student_id: &str,
) -> Result<Vec<UniformIssue>, APIError> {
    Ok(uniform_issues::table
        .filter(uniform_issues::student_id.eq(student_id))
        .load::<UniformIssue>(conn)?)
}

pub fn get_uniform_inventory(conn: &mut SqliteConnection) -> Result<Vec<UniformItem>, APIError> {
    Ok(uniform_items::table.load::<UniformItem>(conn)?)
}

pub fn return_asset(
    conn: &mut SqliteConnection,
    id: &str,
    req: ReturnAssetRequest,
) -> Result<DetailedAssetAllocationResponse, APIError> {
    let now = Utc::now().naive_utc();
    let return_date = req.return_date.unwrap_or(now);

    let alloc: AssetAllocation = asset_allocations::table.find(id).first(conn)?;

    diesel::update(asset_allocations::table.find(id))
        .set((
            asset_allocations::return_date.eq(return_date),
            asset_allocations::updated_at.eq(now),
        ))
        .execute(conn)?;

    // Return to stock
    diesel::update(inventory_items::table.find(&alloc.item_id))
        .set(inventory_items::quantity.eq(inventory_items::quantity + alloc.quantity))
        .execute(conn)?;

    let allocation_response = AssetAllocationResponse::from(alloc.clone());

    let mut allocated_to_staff: Option<StaffResponse> = None;
    let mut allocated_to_student: Option<StudentResponse> = None;

    if let Ok(junction) = asset_allocations_staff::table
        .filter(asset_allocations_staff::asset_allocation_id.eq(&alloc.id))
        .first::<AssetAllocationsStaff>(conn)
    {
        let staff_obj: Staff = staff::table
            .find(&junction.staff_id)
            .select(Staff::as_select())
            .first(conn)?;
        allocated_to_staff = Some(staff_obj.into());
    } else if let Ok(junction) = asset_allocations_students::table
        .filter(asset_allocations_students::asset_allocation_id.eq(&alloc.id))
        .first::<AssetAllocationsStudents>(conn)
    {
        let student_obj: Student = students::table
            .find(&junction.student_id)
            .select(Student::as_select())
            .first(conn)?;
        allocated_to_student = Some(student_obj.into());
    }

    Ok(DetailedAssetAllocationResponse {
        allocation: allocation_response,
        allocated_to_staff,
        allocated_to_student,
    })
}

pub fn get_allocations_by_item(
    conn: &mut SqliteConnection,
    item_id: &str,
) -> Result<Vec<AssetAllocation>, APIError> {
    Ok(asset_allocations::table
        .filter(asset_allocations::item_id.eq(item_id))
        .load::<AssetAllocation>(conn)?)
}

pub fn get_detailed_allocations_by_assignee(
    conn: &mut SqliteConnection,
    assignee_id: &str,
) -> Result<Vec<DetailedAssetAllocationResponse>, APIError> {
    let mut detailed_allocations = Vec::new();

    // Try to find allocations in staff junction table
    let staff_junctions: Vec<AssetAllocationsStaff> = asset_allocations_staff::table
        .filter(asset_allocations_staff::staff_id.eq(assignee_id))
        .load(conn)?;

    for junction in staff_junctions {
        let alloc: AssetAllocation = asset_allocations::table
            .find(&junction.asset_allocation_id)
            .first(conn)?;
        let staff_obj: Staff = staff::table
            .find(&junction.staff_id)
            .select(Staff::as_select())
            .first(conn)?;

        detailed_allocations.push(DetailedAssetAllocationResponse {
            allocation: AssetAllocationResponse::from(alloc),
            allocated_to_staff: Some(staff_obj.into()),
            allocated_to_student: None,
        });
    }

    // Try to find allocations in student junction table
    let student_junctions: Vec<AssetAllocationsStudents> = asset_allocations_students::table
        .filter(asset_allocations_students::student_id.eq(assignee_id))
        .load(conn)?;

    for junction in student_junctions {
        let alloc: AssetAllocation = asset_allocations::table
            .find(&junction.asset_allocation_id)
            .first(conn)?;
        let student_obj: Student = students::table
            .find(&junction.student_id)
            .select(Student::as_select())
            .first(conn)?;

        detailed_allocations.push(DetailedAssetAllocationResponse {
            allocation: AssetAllocationResponse::from(alloc),
            allocated_to_staff: None,
            allocated_to_student: Some(student_obj.into()),
        });
    }

    Ok(detailed_allocations)
}

pub fn update_maintenance_status(
    conn: &mut SqliteConnection,
    id: &str,
    req: UpdateMaintenanceStatusRequest,
) -> Result<MaintenanceRequest, APIError> {
    diesel::update(maintenance_requests::table.find(id))
        .set((
            maintenance_requests::status.eq(req.status),
            maintenance_requests::assigned_to.eq(req.assigned_to),
            maintenance_requests::resolved_date.eq(req.resolved_date),
            maintenance_requests::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(conn)?;

    Ok(maintenance_requests::table.find(id).first(conn)?)
}

pub fn get_pending_maintenance(
    conn: &mut SqliteConnection,
) -> Result<Vec<MaintenanceRequest>, APIError> {
    Ok(maintenance_requests::table
        .filter(maintenance_requests::status.eq(MaintenanceStatus::Pending))
        .load::<MaintenanceRequest>(conn)?)
}
