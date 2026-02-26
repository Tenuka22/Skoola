use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::{AllocationType, MaintenanceStatus};
use backend::models::resource_management::asset_allocation::AssetAllocation;
use backend::models::resource_management::asset_allocation_staff::AssetAllocationStaff;
use backend::models::resource_management::asset_allocation_student::AssetAllocationStudent;
use backend::models::resource_management::asset_category::AssetCategory;
use backend::models::resource_management::inventory_item::InventoryItem;
use backend::models::resource_management::maintenance_request::MaintenanceRequest;
use backend::models::resource_management::resource::Resource;
use backend::models::resource_management::resource_booking::ResourceBooking;
use backend::schema::*;
use diesel::insert_into;
use diesel::prelude::*;
use rand::Rng;
use rand::seq::SliceRandom;
use std::collections::HashSet;

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
        seed_count_config: &crate::SeedCountConfig, // Add SeedCountConfig here
    ) -> Result<()> {
        println!("Seeding Resource Management module...");

        let mut rng = rand::thread_rng();

        // Seed Asset Categories
        let asset_categories_data = (0..seed_count_config.asset_categories)
            .map(|i| AssetCategory {
                id: generate_uuid(),
                name: format!("Category {}", i + 1),
                description: Some(format!("Description for Category {}", i + 1)),
                created_at: random_datetime_in_past(2),
                updated_at: random_datetime_in_past(1),
            })
            .collect::<Vec<AssetCategory>>();

        insert_into(asset_categories::table)
            .values(&asset_categories_data)
            .execute(conn)?;

        context.asset_category_ids = asset_categories_data.into_iter().map(|c| c.id).collect();
        println!(
            "Seeded {} asset categories.",
            context.asset_category_ids.len()
        );

        // Seed Inventory Items
        let inventory_items_data = (0..seed_count_config.inventory_items)
            .map(|i| {
                InventoryItem {
                    id: generate_uuid(),
                    category_id: get_random_id(&context.asset_category_ids),
                    item_name: format!("Item {}", i + 1),
                    description: Some(format!("Description for Item {}", i + 1)),
                    unit: "unit".to_string(), // Default unit
                    quantity: rng.gen_range(1..=100),
                    reorder_level: rng.gen_range(5..=20),
                    unit_price: rng.gen_range(10.0..=1000.0),
                    created_at: random_datetime_in_past(2),
                    updated_at: random_datetime_in_past(1),
                }
            })
            .collect::<Vec<InventoryItem>>();

        insert_into(inventory_items::table)
            .values(&inventory_items_data)
            .execute(conn)?;

        context.inventory_item_ids = inventory_items_data.into_iter().map(|i| i.id).collect();
        println!(
            "Seeded {} inventory items.",
            context.inventory_item_ids.len()
        );

        // Seed Resources
        let resources_data = (0..seed_count_config.resources)
            .map(|i| Resource {
                id: generate_uuid(),
                resource_name: format!("Resource {}", i + 1),
                resource_type: match i % 3 {
                    0 => "Venue".to_string(),
                    1 => "Vehicle".to_string(),
                    _ => "Equipment".to_string(),
                },
                description: Some(format!("Description for Resource {}", i + 1)),
                created_at: random_datetime_in_past(2),
                updated_at: random_datetime_in_past(1),
            })
            .collect::<Vec<Resource>>();

        insert_into(resources::table)
            .values(&resources_data)
            .execute(conn)?;

        context.resource_ids = resources_data.into_iter().map(|r| r.id).collect();
        println!("Seeded {} resources.", context.resource_ids.len());

        // Seed Asset Allocations
        if context.user_ids.is_empty()
            || context.staff_ids.is_empty()
            || context.student_ids.is_empty()
            || context.inventory_item_ids.is_empty()
        {
            println!(
                "Skipping AssetAllocation seeding: user_ids, staff_ids, student_ids, or inventory_item_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let asset_allocations_data = (0..seed_count_config.asset_allocations)
                .map(|i| {
                    let allocated_to_type = if i % 2 == 0 {
                        AllocationType::Teacher
                    } else {
                        AllocationType::Student
                    };
                    let allocated_to_id = if allocated_to_type == AllocationType::Teacher {
                        get_random_id(&context.staff_ids)
                    } else {
                        get_random_id(&context.student_ids)
                    };

                    AssetAllocation {
                        id: generate_uuid(),
                        item_id: get_random_id(&context.inventory_item_ids),
                        allocated_to_type: allocated_to_type.to_string(),
                        allocated_to_id,
                        quantity: rng.gen_range(1..=5),
                        allocation_date: random_datetime_in_past(1),
                        return_date: Some(random_datetime_in_past(0)),
                        allocated_by: get_random_id(&context.user_ids),
                        created_at: random_datetime_in_past(1),
                        updated_at: random_datetime_in_past(0),
                    }
                })
                .collect::<Vec<AssetAllocation>>();

            insert_into(asset_allocations::table)
                .values(&asset_allocations_data)
                .execute(conn)?;

            context.asset_allocation_ids =
                asset_allocations_data.into_iter().map(|a| a.id).collect();
            println!(
                "Seeded {} asset allocations.",
                context.asset_allocation_ids.len()
            );
        }

        // Seed Maintenance Requests
        if context.inventory_item_ids.is_empty()
            || context.user_ids.is_empty()
            || context.staff_ids.is_empty()
        {
            println!(
                "Skipping MaintenanceRequest seeding: inventory_item_ids, user_ids, or staff_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let maintenance_requests_data = (0..seed_count_config.maintenance_requests)
                .map(|i| MaintenanceRequest {
                    id: generate_uuid(),
                    item_id: get_random_id(&context.inventory_item_ids),
                    reported_by: get_random_id(&context.user_ids),
                    reported_date: random_datetime_in_past(1),
                    status: vec![
                        MaintenanceStatus::Pending,
                        MaintenanceStatus::InProgress,
                        MaintenanceStatus::Completed,
                    ]
                    .choose(&mut rng)
                    .unwrap()
                    .to_string(),
                    assigned_to: Some(get_random_id(&context.staff_ids)),
                    resolved_date: Some(random_datetime_in_past(0)),
                    created_at: random_datetime_in_past(1),
                    updated_at: random_datetime_in_past(0),
                    issue_description: format!("Issue with item {}: {}", i + 1, generate_uuid()),
                })
                .collect::<Vec<MaintenanceRequest>>();

            insert_into(maintenance_requests::table)
                .values(&maintenance_requests_data)
                .execute(conn)?;

            println!(
                "Seeded {} maintenance requests.",
                maintenance_requests_data.len()
            );
        }

        // Seed Resource Bookings
        if context.resource_ids.is_empty() || context.user_ids.is_empty() {
            println!(
                "Skipping ResourceBooking seeding: resource_ids or user_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let resource_bookings_data = (0..seed_count_config.resource_bookings)
                .map(|_| {
                    let start_time = random_datetime_in_past(1);
                    let end_time = start_time + chrono::Duration::hours(rng.gen_range(1..=5));
                    ResourceBooking {
                        id: generate_uuid(),
                        resource_id: get_random_id(&context.resource_ids),
                        booked_by_user_id: get_random_id(&context.user_ids),
                        start_time,
                        end_time,
                        related_event_id: if rng.gen_bool(0.5) {
                            Some(generate_uuid())
                        } else {
                            None
                        }, // Dummy event ID
                        created_at: random_datetime_in_past(1),
                        updated_at: random_datetime_in_past(0),
                    }
                })
                .collect::<Vec<ResourceBooking>>();

            insert_into(resource_bookings::table)
                .values(&resource_bookings_data)
                .execute(conn)?;

            println!("Seeded {} resource bookings.", resource_bookings_data.len());
        }

        // Seed Asset Allocation Staff
        if !context.asset_allocation_ids.is_empty() && !context.staff_ids.is_empty() {
            let mut asset_allocation_staff_data = Vec::new();
            let mut seen_allocations = HashSet::new();

            for _ in 0..seed_count_config.asset_allocations {
                let asset_allocation_id = get_random_id(&context.asset_allocation_ids);
                let staff_id = get_random_id(&context.staff_ids);

                if seen_allocations.insert((asset_allocation_id.clone(), staff_id.clone())) {
                    asset_allocation_staff_data.push(AssetAllocationStaff {
                        asset_allocation_id,
                        staff_id,
                        created_at: random_datetime_in_past(1),
                    });
                }
            }

            insert_into(asset_allocations_staff::table)
                .values(&asset_allocation_staff_data)
                .execute(conn)?;

            println!(
                "Seeded {} asset allocation staff entries.",
                asset_allocation_staff_data.len()
            );
        } else {
            println!(
                "Skipping AssetAllocationStaff seeding: asset_allocation_ids or staff_ids are empty."
            );
        }

        // Seed Asset Allocation Students
        if !context.asset_allocation_ids.is_empty() && !context.student_ids.is_empty() {
            let mut asset_allocation_student_data = Vec::new();
            let mut seen_allocations = HashSet::new();

            for _ in 0..seed_count_config.asset_allocations {
                let asset_allocation_id = get_random_id(&context.asset_allocation_ids);
                let student_id = get_random_id(&context.student_ids);

                if seen_allocations.insert((asset_allocation_id.clone(), student_id.clone())) {
                    asset_allocation_student_data.push(AssetAllocationStudent {
                        asset_allocation_id,
                        student_id,
                        created_at: random_datetime_in_past(1),
                    });
                }
            }

            insert_into(asset_allocations_students::table)
                .values(&asset_allocation_student_data)
                .execute(conn)?;

            println!(
                "Seeded {} asset allocation student entries.",
                asset_allocation_student_data.len()
            );
        } else {
            println!(
                "Skipping AssetAllocationStudents seeding: asset_allocation_ids or student_ids are empty."
            );
        }

        Ok(())
    }
}
