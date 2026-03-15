use crate::database::enums::MaintenanceStatus;
use crate::errors::APIError;
use crate::models::resource_management::*;
use crate::models::resources::*;
use crate::models::staff::staff::{Staff, StaffResponse};
use crate::models::student::student::{Student, StudentResponse};
use crate::schema::{
    asset_allocations, asset_allocations_staff, asset_allocations_students, asset_categories,
    inventory_item_details, inventory_items, maintenance_requests, staff, students, uniform_issues,
    uniform_items, inventory_transactions, asset_maintenance_logs,
};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::AppState;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use crate::impl_admin_entity_service;
use crate::services::admin_db::AdminQuery;
use diesel::sqlite::SqliteConnection;

impl_admin_entity_service!(
    AssetCategoryService,
    asset_categories::table,
    AssetCategory,
    AssetCategory,
    asset_categories::id,
    AdminQuery,
    |q: asset_categories::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(asset_categories::name.like(search))
    },
    |q: asset_categories::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(asset_categories::created_at.desc())
    }
);

impl_admin_entity_service!(
    InventoryItemService,
    inventory_items::table,
    InventoryItem,
    InventoryItem,
    inventory_items::id,
    AdminQuery,
    |q: inventory_items::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(inventory_items::item_name.like(search))
    },
    |q: inventory_items::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(inventory_items::created_at.desc())
    }
);

impl_admin_entity_service!(
    InventoryTransactionService,
    inventory_transactions::table,
    InventoryTransaction,
    InventoryTransaction,
    inventory_transactions::id,
    AdminQuery,
    |q: inventory_transactions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: inventory_transactions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(inventory_transactions::created_at.desc())
    }
);

impl_admin_entity_service!(
    AssetMaintenanceLogService,
    asset_maintenance_logs::table,
    AssetMaintenanceLog,
    AssetMaintenanceLog,
    asset_maintenance_logs::id,
    AdminQuery,
    |q: asset_maintenance_logs::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: asset_maintenance_logs::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(asset_maintenance_logs::created_at.desc())
    }
);

impl_admin_entity_service!(
    MaintenanceRequestService,
    maintenance_requests::table,
    MaintenanceRequest,
    MaintenanceRequest,
    maintenance_requests::id,
    AdminQuery,
    |q: maintenance_requests::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(maintenance_requests::issue_description.like(search))
    },
    |q: maintenance_requests::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(maintenance_requests::created_at.desc())
    }
);

impl_admin_entity_service!(
    UniformItemService,
    uniform_items::table,
    UniformItem,
    UniformItem,
    uniform_items::id,
    AdminQuery,
    |q: uniform_items::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(uniform_items::item_name.like(search))
    },
    |q: uniform_items::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(uniform_items::item_name.asc())
    }
);

impl_admin_entity_service!(
    UniformIssueService,
    uniform_issues::table,
    UniformIssue,
    UniformIssue,
    uniform_issues::id,
    AdminQuery,
    |q: uniform_issues::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: uniform_issues::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(uniform_issues::issue_date.desc())
    }
);

impl_admin_entity_service!(
    AssetAllocationService,
    asset_allocations::table,
    AssetAllocation,
    AssetAllocation,
    asset_allocations::id,
    AdminQuery,
    |q: asset_allocations::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: asset_allocations::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(asset_allocations::allocation_date.desc())
    }
);

impl_admin_entity_service!(
    InventoryItemDetailService,
    inventory_item_details::table,
    InventoryItemDetail,
    InventoryItemDetail,
    inventory_item_details::item_id,
    item_id,
    AdminQuery,
    |q: inventory_item_details::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(inventory_item_details::description.like(search))
    },
    |q: inventory_item_details::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(inventory_item_details::created_at.desc())
    }
);

impl InventoryItemDetailService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: InventoryItemDetail,
    ) -> Result<InventoryItemDetail, APIError> {
        let now = Utc::now().naive_utc();
        let mut new_item = req;
        new_item.created_at = now;
        new_item.updated_at = now;
        Self::generic_create(data, new_item).await
    }
}

impl AssetCategoryService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateAssetCategoryRequest,
    ) -> Result<AssetCategory, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::PROPERTY)?;
        let new_item = AssetCategory {
            id,
            name: req.name,
            description: req.description,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl InventoryItemService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateInventoryItemRequest,
    ) -> Result<InventoryItem, APIError> {
        let mut conn = data.db_pool.get()?;
        let now = Utc::now().naive_utc();
        let item_id = generate_prefixed_id(&mut conn, IdPrefix::PROPERTY)?;

        let new_item = InventoryItem {
            id: item_id.clone(),
            category_id: req.category_id,
            item_name: req.item_name,
            unit: req.unit,
            created_at: now,
            updated_at: now,
        };

        conn.transaction::<_, APIError, _>(|conn| {
            diesel::insert_into(inventory_items::table)
                .values(&new_item)
                .execute(conn)?;

            let details = InventoryItemDetail {
                item_id,
                description: req.description,
                quantity: req.quantity,
                reorder_level: req.reorder_level,
                unit_price: req.unit_price,
                created_at: now,
                updated_at: now,
            };
            diesel::insert_into(inventory_item_details::table)
                .values(&details)
                .execute(conn)?;
            Ok(())
        })?;

        Ok(new_item)
    }
}

impl InventoryTransactionService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateInventoryTransactionRequest,
    ) -> Result<InventoryTransaction, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::PROPERTY)?;
        let new_item = InventoryTransaction {
            id,
            item_id: req.item_id,
            transaction_type: req.transaction_type,
            quantity: req.quantity,
            unit_cost: req.unit_cost,
            transaction_date: Utc::now().naive_utc(),
            reference_type: req.reference_type,
            reference_id: req.reference_id,
            created_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

impl AssetMaintenanceLogService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateAssetMaintenanceLogRequest,
    ) -> Result<AssetMaintenanceLog, APIError> {
        let mut conn = data.db_pool.get()?;
        let id = generate_prefixed_id(&mut conn, IdPrefix::PROPERTY)?;
        let new_item = AssetMaintenanceLog {
            id,
            item_id: req.item_id,
            maintenance_date: req.maintenance_date,
            maintenance_type: req.maintenance_type,
            notes: req.notes,
            cost: req.cost,
            performed_by: req.performed_by,
            created_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_item).await
    }
}

// --- specialized services ---

pub async fn create_uniform_item(
    data: web::Data<AppState>,
    req: CreateUniformItemRequest,
) -> Result<UniformItem, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_item = UniformItem {
        id: generate_prefixed_id(&mut conn, IdPrefix::PROPERTY)?,
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
        .execute(&mut conn)?;
    Ok(new_item)
}

impl UniformItemService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateUniformItemRequest,
    ) -> Result<UniformItem, APIError> {
        create_uniform_item(data, req).await
    }
}

pub async fn issue_uniform(
    data: web::Data<AppState>,
    req: IssueUniformRequest,
) -> Result<UniformIssue, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_issue = UniformIssue {
        id: generate_prefixed_id(&mut conn, IdPrefix::PROPERTY)?,
        student_id: req.student_id,
        uniform_item_id: req.uniform_item_id,
        quantity: req.quantity,
        issue_date: Utc::now().naive_utc(),
        issued_by: req.issued_by,
        amount_collected: req.amount_collected,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    conn.transaction::<_, APIError, _>(|conn| {
        diesel::insert_into(uniform_issues::table)
            .values(&new_issue)
            .execute(conn)?;

        // Update stock
        let item: UniformItem = uniform_items::table.find(&new_issue.uniform_item_id).first(conn)?;
        diesel::update(uniform_items::table.filter(uniform_items::id.eq(&new_issue.uniform_item_id)))
            .set(uniform_items::quantity.eq(item.quantity - new_issue.quantity))
            .execute(conn)?;
        Ok(())
    })?;

    Ok(new_issue)
}

impl UniformIssueService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: IssueUniformRequest,
    ) -> Result<UniformIssue, APIError> {
        issue_uniform(data, req).await
    }
}

pub async fn allocate_asset(
    data: web::Data<AppState>,
    req: AllocateAssetRequest,
) -> Result<DetailedAssetAllocationResponse, APIError> {
    let mut conn = data.db_pool.get()?;
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
    let new_alloc_id = generate_prefixed_id(&mut conn, IdPrefix::PROPERTY_ALLOCATION)?;

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

    conn.transaction::<_, APIError, _>(|conn| {
        diesel::insert_into(asset_allocations::table)
            .values(&new_alloc)
            .execute(conn)?;

        if let Some(staff_id) = &req.staff_id {
            let new_junction = AssetAllocationStaff {
                asset_allocation_id: new_alloc_id.clone(),
                staff_id: staff_id.clone(),
                created_at: now,
            };
            diesel::insert_into(asset_allocations_staff::table)
                .values(&new_junction)
                .execute(conn)?;
        } else if let Some(student_id) = &req.student_id {
            let new_junction = AssetAllocationStudent {
                asset_allocation_id: new_alloc_id.clone(),
                student_id: student_id.clone(),
                created_at: now,
            };
            diesel::insert_into(asset_allocations_students::table)
                .values(&new_junction)
                .execute(conn)?;
        }

        // Update stock
        let detail: InventoryItemDetail = inventory_item_details::table.find(&new_alloc.item_id).first(conn)?;
        diesel::update(inventory_item_details::table.filter(inventory_item_details::item_id.eq(&new_alloc.item_id)))
            .set(inventory_item_details::quantity.eq(detail.quantity - new_alloc.quantity))
            .execute(conn)?;
        Ok(())
    })?;

    let mut allocated_to_staff: Option<StaffResponse> = None;
    let mut allocated_to_student: Option<StudentResponse> = None;

    if let Some(staff_id) = req.staff_id {
        let staff_obj: Staff = staff::table
            .find(staff_id)
            .select(Staff::as_select())
            .first(&mut conn)?;
        allocated_to_staff = Some(staff_obj.into());
    } else if let Some(student_id) = req.student_id {
        let student_obj: Student = students::table
            .find(student_id)
            .select(Student::as_select())
            .first(&mut conn)?;
        allocated_to_student = Some(student_obj.into());
    }

    let allocation_response = AssetAllocationResponse::from(new_alloc);

    Ok(DetailedAssetAllocationResponse {
        allocation: allocation_response,
        allocated_to_staff,
        allocated_to_student,
    })
}

impl AssetAllocationService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: AllocateAssetRequest,
    ) -> Result<AssetAllocation, APIError> {
        let detailed = allocate_asset(data.clone(), req).await?;
        let mut conn = data.db_pool.get()?;
        Ok(asset_allocations::table
            .find(&detailed.allocation.id)
            .first::<AssetAllocation>(&mut conn)?)
    }

    pub async fn return_asset(
        data: web::Data<AppState>,
        id: String,
        req: ReturnAssetRequest,
    ) -> Result<AssetAllocation, APIError> {
        return_asset(data, id, req).await
    }
}

pub async fn create_maintenance_request(
    data: web::Data<AppState>,
    req: CreateMaintenanceRequest,
) -> Result<MaintenanceRequest, APIError> {
    let mut conn = data.db_pool.get()?;
    let new_req = MaintenanceRequest {
        id: generate_prefixed_id(&mut conn, IdPrefix::RESOURCE)?,
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
        .execute(&mut conn)?;
    Ok(new_req)
}

impl MaintenanceRequestService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateMaintenanceRequest,
    ) -> Result<MaintenanceRequest, APIError> {
        create_maintenance_request(data, req).await
    }
}

pub async fn update_inventory_item(
    data: web::Data<AppState>,
    id: String,
    req: UpdateInventoryItemWithDetailsRequest,
) -> Result<InventoryItemResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let now = Utc::now().naive_utc();

    conn.transaction::<_, APIError, _>(|conn| {
        diesel::update(inventory_items::table.filter(inventory_items::id.eq(&id)))
            .set((
                req.item_name.as_ref().map(|v| inventory_items::item_name.eq(v)),
                req.unit.as_ref().map(|v| inventory_items::unit.eq(v)),
                Some(inventory_items::updated_at.eq(now)),
            ))
            .execute(conn)?;

        diesel::update(inventory_item_details::table.filter(inventory_item_details::item_id.eq(&id)))
            .set((
                req.description.as_ref().map(|v| inventory_item_details::description.eq(v)),
                req.reorder_level.map(|v| inventory_item_details::reorder_level.eq(v)),
                req.unit_price.map(|v| inventory_item_details::unit_price.eq(v)),
                Some(inventory_item_details::updated_at.eq(now)),
            ))
            .execute(conn)?;
        Ok(())
    })?;

    get_inventory_item_by_id(&mut conn, &id)
}

pub async fn update_stock_quantity(
    data: web::Data<AppState>,
    id: String,
    req: UpdateStockRequest,
) -> Result<InventoryItemResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    diesel::update(inventory_item_details::table.filter(inventory_item_details::item_id.eq(&id)))
        .set((
            inventory_item_details::quantity.eq(req.quantity),
            inventory_item_details::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    get_inventory_item_by_id(&mut conn, &id)
}

fn get_inventory_item_by_id(conn: &mut SqliteConnection, id: &str) -> Result<InventoryItemResponse, APIError> {
    let (item, details) = inventory_items::table
        .inner_join(inventory_item_details::table)
        .filter(inventory_items::id.eq(id))
        .select((InventoryItem::as_select(), InventoryItemDetail::as_select()))
        .first::<(InventoryItem, InventoryItemDetail)>(conn)?;

    Ok(InventoryItemResponse {
        id: item.id,
        category_id: item.category_id,
        item_name: item.item_name,
        description: details.description,
        unit: item.unit,
        quantity: details.quantity,
        reorder_level: details.reorder_level,
        unit_price: details.unit_price,
    })
}

pub async fn get_low_stock_items(data: web::Data<AppState>) -> Result<Vec<InventoryItemResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let results = inventory_items::table
        .inner_join(inventory_item_details::table)
        .filter(inventory_item_details::quantity.le(inventory_item_details::reorder_level))
        .select((InventoryItem::as_select(), InventoryItemDetail::as_select()))
        .load::<(InventoryItem, InventoryItemDetail)>(&mut conn)?;

    Ok(results.into_iter().map(|(item, details)| InventoryItemResponse {
        id: item.id,
        category_id: item.category_id,
        item_name: item.item_name,
        description: details.description,
        unit: item.unit,
        quantity: details.quantity,
        reorder_level: details.reorder_level,
        unit_price: details.unit_price,
    }).collect())
}

pub async fn search_inventory(
    data: web::Data<AppState>,
    query: String,
) -> Result<Vec<InventoryItemResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let pattern = format!("%{}%", query);
    let results = inventory_items::table
        .inner_join(inventory_item_details::table)
        .filter(inventory_items::item_name.like(&pattern).or(inventory_item_details::description.like(&pattern)))
        .select((InventoryItem::as_select(), InventoryItemDetail::as_select()))
        .load::<(InventoryItem, InventoryItemDetail)>(&mut conn)?;

    Ok(results.into_iter().map(|(item, details)| InventoryItemResponse {
        id: item.id,
        category_id: item.category_id,
        item_name: item.item_name,
        description: details.description,
        unit: item.unit,
        quantity: details.quantity,
        reorder_level: details.reorder_level,
        unit_price: details.unit_price,
    }).collect())
}

pub async fn get_uniform_history(
    data: web::Data<AppState>,
    student_id: String,
) -> Result<Vec<UniformIssue>, APIError> {
    let mut conn = data.db_pool.get()?;
    Ok(uniform_issues::table
        .filter(uniform_issues::student_id.eq(student_id))
        .load::<UniformIssue>(&mut conn)?)
}

pub async fn get_uniform_inventory(data: web::Data<AppState>) -> Result<Vec<UniformItem>, APIError> {
    let mut conn = data.db_pool.get()?;
    Ok(uniform_items::table.load::<UniformItem>(&mut conn)?)
}

pub async fn return_asset(
    data: web::Data<AppState>,
    id: String,
    req: ReturnAssetRequest,
) -> Result<AssetAllocation, APIError> {
    let mut conn = data.db_pool.get()?;
    let now = Utc::now().naive_utc();
    let return_date = req.return_date.unwrap_or(now);

    let alloc: AssetAllocation = asset_allocations::table.find(&id).first(&mut conn)?;

    conn.transaction::<_, APIError, _>(|conn| {
        diesel::update(asset_allocations::table.find(&id))
            .set((
                asset_allocations::return_date.eq(return_date),
                asset_allocations::updated_at.eq(now),
            ))
            .execute(conn)?;

        // Return to stock
        let detail: InventoryItemDetail = inventory_item_details::table.find(&alloc.item_id).first(conn)?;
        diesel::update(inventory_item_details::table.filter(inventory_item_details::item_id.eq(&alloc.item_id)))
            .set(inventory_item_details::quantity.eq(detail.quantity + alloc.quantity))
            .execute(conn)?;
        Ok(())
    })?;

    Ok(asset_allocations::table.find(id).first(&mut conn)?)
}

pub async fn get_allocations_by_item(
    data: web::Data<AppState>,
    item_id: String,
) -> Result<Vec<DetailedAssetAllocationResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let allocs: Vec<AssetAllocation> = asset_allocations::table
        .filter(asset_allocations::item_id.eq(item_id))
        .load::<AssetAllocation>(&mut conn)?;

    let mut res = Vec::new();
    for a in allocs {
        let mut staff_resp = None;
        let mut student_resp = None;
        if a.allocated_to_type == "STAFF" {
            if let Ok(s) = staff::table.find(&a.allocated_to_id).first::<Staff>(&mut conn) {
                staff_resp = Some(s.into());
            }
        } else {
            if let Ok(s) = students::table.find(&a.allocated_to_id).first::<Student>(&mut conn) {
                student_resp = Some(s.into());
            }
        }
        res.push(DetailedAssetAllocationResponse {
            allocation: a.into(),
            allocated_to_staff: staff_resp,
            allocated_to_student: student_resp,
        });
    }
    Ok(res)
}

pub async fn get_allocations_by_assignee(
    data: web::Data<AppState>,
    assignee_id: String,
) -> Result<Vec<DetailedAssetAllocationResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let allocs: Vec<AssetAllocation> = asset_allocations::table
        .filter(asset_allocations::allocated_to_id.eq(assignee_id))
        .load::<AssetAllocation>(&mut conn)?;

    let mut res = Vec::new();
    for a in allocs {
        let mut staff_resp = None;
        let mut student_resp = None;
        if a.allocated_to_type == "STAFF" {
            if let Ok(s) = staff::table.find(&a.allocated_to_id).first::<Staff>(&mut conn) {
                staff_resp = Some(s.into());
            }
        } else {
            if let Ok(s) = students::table.find(&a.allocated_to_id).first::<Student>(&mut conn) {
                student_resp = Some(s.into());
            }
        }
        res.push(DetailedAssetAllocationResponse {
            allocation: a.into(),
            allocated_to_staff: staff_resp,
            allocated_to_student: student_resp,
        });
    }
    Ok(res)
}

pub async fn update_maintenance_status(
    data: web::Data<AppState>,
    id: String,
    req: UpdateMaintenanceStatusRequest,
) -> Result<MaintenanceRequest, APIError> {
    let mut conn = data.db_pool.get()?;
    diesel::update(maintenance_requests::table.find(&id))
        .set((
            maintenance_requests::status.eq(req.status),
            maintenance_requests::assigned_to.eq(req.assigned_to),
            maintenance_requests::resolved_date.eq(req.resolved_date),
            maintenance_requests::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    Ok(maintenance_requests::table.find(id).first(&mut conn)?)
}

pub fn get_pending_maintenance(
    conn: &mut SqliteConnection,
) -> Result<Vec<MaintenanceRequest>, APIError> {
    Ok(maintenance_requests::table
        .filter(maintenance_requests::status.eq(MaintenanceStatus::Pending))
        .load::<MaintenanceRequest>(conn)?)
}
