use diesel::prelude::*;
use diesel::insert_into;
use anyhow::Result;
use crate::schema::*;
use crate::Config;
use std::collections::HashSet;
use crate::bin::seed_modules::utils::*;
use crate::bin::seed_modules::{SeedModule, SeederContext};
use crate::models::{
    AssetCategory,
    InventoryItem,
    Resource,
    AssetAllocation,
    MaintenanceRequest,
    ResourceBooking,
    AssetAllocationStaff,
    AssetAllocationStudent,
};
use rand::Rng; // For rand::random

pub struct ResourceManagementSeeder;

impl ResourceManagementSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for ResourceManagementSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
    ) -> Result<()> {
        println!("Seeding Resource Management module...");

        // Seed Asset Categories
        let asset_categories_data = (1..=5).map(|i| {
            AssetCategory {
                id: generate_uuid(),
                name: format!("Category {}", i),
                description: Some(format!("Description for Category {}", i)),
                created_at: Some(random_datetime_in_past(2)),
                updated_at: Some(random_datetime_in_past(1)),
            }
        }).collect::<Vec<AssetCategory>>();

        insert_into(asset_categories::table)
            .values(&asset_categories_data)
            .execute(conn)?;

        context.asset_category_ids = asset_categories_data.into_iter().map(|c| c.id).collect();
        println!("Seeded {} asset categories.", context.asset_category_ids.len());

        // Seed Inventory Items
        let inventory_items_data = (1..=20).map(|i| {
            InventoryItem {
                id: generate_uuid(),
                category_id: get_random_id(&context.asset_category_ids),
                item_name: format!("Item {}", i),
                description: Some(format!("Description for Item {}", i)),
                unit: "unit".to_string(), // Default unit
                quantity: rand::thread_rng().gen_range(1..=100),
                reorder_level: rand::thread_rng().gen_range(5..=20),
                unit_price: rand::thread_rng().gen_range(10.0..=1000.0),
                created_at: Some(random_datetime_in_past(2)),
                updated_at: Some(random_datetime_in_past(1)),
            }
        }).collect::<Vec<InventoryItem>>();

        insert_into(inventory_items::table)
            .values(&inventory_items_data)
            .execute(conn)?;

        context.inventory_item_ids = inventory_items_data.into_iter().map(|i| i.id).collect();
        println!("Seeded {} inventory items.", context.inventory_item_ids.len());

        // Seed Resources
        let resources_data = (1..=10).map(|i| {
            Resource {
                id: generate_uuid(),
                resource_name: format!("Resource {}", i),
                resource_type: match i % 3 {
                    0 => "Room".to_string(),
                    1 => "Vehicle".to_string(),
                    _ => "Equipment".to_string(),
                },
                description: Some(format!("Description for Resource {}", i)),
                created_at: Some(random_datetime_in_past(2)),
                updated_at: Some(random_datetime_in_past(1)),
            }
        }).collect::<Vec<Resource>>();

        insert_into(resources::table)
            .values(&resources_data)
            .execute(conn)?;

        context.resource_ids = resources_data.into_iter().map(|r| r.id).collect();
        println!("Seeded {} resources.", context.resource_ids.len());

        // Seed Asset Allocations
        // Ensure user_ids, staff_ids, student_ids are populated before this step.
        // For demonstration, we'll ensure they are not empty before proceeding.
        if context.user_ids.is_empty() || context.staff_ids.is_empty() || context.student_ids.is_empty() {
            println!("Skipping AssetAllocation seeding: user_ids, staff_ids, or student_ids are empty. Ensure relevant seeders run first.");
        } else {
            let asset_allocations_data = (1..=15).map(|i| {
                let allocated_to_type = if i % 2 == 0 { "Staff" } else { "Student" }.to_string();
                let allocated_to_id = if allocated_to_type == "Staff" {
                    get_random_id(&context.staff_ids)
                } else {
                    get_random_id(&context.student_ids)
                };

                AssetAllocation {
                    id: generate_uuid(),
                    item_id: get_random_id(&context.inventory_item_ids),
                    allocated_to_type,
                    allocated_to_id,
                    quantity: rand::thread_rng().gen_range(1..=5),
                    allocation_date: random_datetime_in_past(1),
                    return_date: Some(random_datetime_in_past(0)),
                    allocated_by: get_random_id(&context.user_ids),
                    created_at: Some(random_datetime_in_past(1)),
                    updated_at: Some(random_datetime_in_past(0)),
                }
            }).collect::<Vec<AssetAllocation>>();

            insert_into(asset_allocations::table)
                .values(&asset_allocations_data)
                .execute(conn)?;

            context.asset_allocation_ids = asset_allocations_data.into_iter().map(|a| a.id).collect();
            println!("Seeded {} asset allocations.", context.asset_allocation_ids.len());
        }

        // Seed Maintenance Requests
        if context.inventory_item_ids.is_empty() || context.user_ids.is_empty() || context.staff_ids.is_empty() {
             println!("Skipping MaintenanceRequest seeding: inventory_item_ids, user_ids, or staff_ids are empty. Ensure relevant seeders run first.");
        } else {
            let maintenance_requests_data = (1..=10).map(|i| {
                MaintenanceRequest {
                    id: generate_uuid(),
                    item_id: get_random_id(&context.inventory_item_ids),
                    reported_by: get_random_id(&context.user_ids),
                    reported_date: random_datetime_in_past(1),
                    status: match i % 3 {
                        0 => "Pending".to_string(),
                        1 => "In Progress".to_string(),
                        _ => "Completed".to_string(),
                    },
                    assigned_to: Some(get_random_id(&context.staff_ids)),
                    resolved_date: Some(random_datetime_in_past(0)),
                    created_at: Some(random_datetime_in_past(1)),
                    updated_at: Some(random_datetime_in_past(0)),
                    issue_description: format!("Issue with item {}: {:?}", i, generate_uuid()),
                }
            }).collect::<Vec<MaintenanceRequest>>();

            insert_into(maintenance_requests::table)
                .values(&maintenance_requests_data)
                .execute(conn)?;

            println!("Seeded {} maintenance requests.", maintenance_requests_data.len());
        }

        // Seed Resource Bookings
        if context.resource_ids.is_empty() || context.user_ids.is_empty() {
            println!("Skipping ResourceBooking seeding: resource_ids or user_ids are empty. Ensure relevant seeders run first.");
        } else {
            let resource_bookings_data = (1..=15).map(|i| {
                let start_time = random_datetime_in_past(1);
                let end_time = start_time + chrono::Duration::hours(1);
                ResourceBooking {
                    id: generate_uuid(),
                    resource_id: get_random_id(&context.resource_ids),
                    booked_by_user_id: get_random_id(&context.user_ids),
                    start_time,
                    end_time,
                    related_event_id: if i % 2 == 0 { Some(generate_uuid()) } else { None }, // Dummy event ID
                    created_at: Some(random_datetime_in_past(1)),
                    updated_at: Some(random_datetime_in_past(0)),
                }
            }).collect::<Vec<ResourceBooking>>();

            insert_into(resource_bookings::table)
                .values(&resource_bookings_data)
                .execute(conn)?;

            println!("Seeded {} resource bookings.", resource_bookings_data.len());
        }

        // Seed Asset Allocation Staff (assuming asset_allocation_ids and staff_ids are populated)
        if !context.asset_allocation_ids.is_empty() && !context.staff_ids.is_empty() {
            let asset_allocation_staff_data = (0..5).filter_map(|_| {
                let asset_allocation_id = get_random_id(&context.asset_allocation_ids);
                let staff_id = get_random_id(&context.staff_ids);

                // Prevent duplicate primary keys for (asset_allocation_id, staff_id)
                // This is a simple approach, more robust deduplication might be needed for larger scales
                let exists = asset_allocations_staff::table
                    .filter(asset_allocations_staff::asset_allocation_id.eq(&asset_allocation_id))
                    .filter(asset_allocations_staff::staff_id.eq(&staff_id))
                    .count()
                    .get_result::<i64>(conn)
                    .unwrap_or(0) > 0;

                if exists {
                    None
                } else {
                    Some(AssetAllocationStaff {
                        asset_allocation_id,
                        staff_id,
                        created_at: Some(random_datetime_in_past(1)),
                    })
                }
            }).collect::<Vec<AssetAllocationStaff>>();

            insert_into(asset_allocations_staff::table)
                .values(&asset_allocation_staff_data)
                .execute(conn)?;

            println!("Seeded {} asset allocation staff entries.", asset_allocation_staff_data.len());
        } else {
            println!("Skipping AssetAllocationStaff seeding: asset_allocation_ids or staff_ids are empty.");
        }


        // Seed Asset Allocation Students (assuming asset_allocation_ids and student_ids are populated)
        if !context.asset_allocation_ids.is_empty() && !context.student_ids.is_empty() {
            let asset_allocation_student_data = (0..5).filter_map(|_| {
                let asset_allocation_id = get_random_id(&context.asset_allocation_ids);
                let student_id = get_random_id(&context.student_ids);

                let exists = asset_allocations_students::table
                    .filter(asset_allocations_students::asset_allocation_id.eq(&asset_allocation_id))
                    .filter(asset_allocations_students::student_id.eq(&student_id))
                    .count()
                    .get_result::<i64>(conn)
                    .unwrap_or(0) > 0;

                if exists {
                    None
                } else {
                    Some(AssetAllocationStudent {
                        asset_allocation_id,
                        student_id,
                        created_at: Some(random_datetime_in_past(1)),
                    })
                }
            }).collect::<Vec<AssetAllocationStudent>>();

            insert_into(asset_allocations_students::table)
                .values(&asset_allocation_student_data)
                .execute(conn)?;

            println!("Seeded {} asset allocation student entries.", asset_allocation_student_data.len());
        } else {
            println!("Skipping AssetAllocationStudents seeding: asset_allocation_ids or student_ids are empty.");
        }


        Ok(())
    }
}
