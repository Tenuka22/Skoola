use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::tables::Resource;
use backend::models::{AssetCategory, InventoryItem};
use backend::models::ids::IdPrefix;
use backend::schema::*;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use rand::Rng;
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
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Resource Management module...");

        let mut rng = rand::thread_rng();

        // 1. asset_categories
        println!("Seeding asset_categories...");
        let cats = vec!["Electronics", "Furniture", "Stationery", "Sports Gear", "Laboratory", "Musical Instruments", "Vehicles", "Library Equipment", "Kitchen", "Maintenance"];
        for name in cats {
            let id = next_id(conn, IdPrefix::PROPERTY);
            insert_into(asset_categories::table)
                .values(&AssetCategory {
                    id: id.clone(),
                    name: name.to_string(),
                    description: Some(format!("{} assets", name)),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;
            context.asset_category_ids.push(id);
        }

        // 2. inventory_items
        println!("Seeding inventory_items...");
        for i in 0..100 {
            let id = next_id(conn, IdPrefix::PROPERTY);
            let cat_id = get_random_id(&context.asset_category_ids);
            insert_into(inventory_items::table)
                .values(&InventoryItem {
                    id: id.clone(),
                    category_id: cat_id.clone(),
                    item_name: format!("Item {}", i),
                    unit: "Unit".to_string(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;
            
            insert_into(inventory_item_details::table)
                .values(&(
                    inventory_item_details::item_id.eq(id.clone()),
                    inventory_item_details::description.eq(Some("Description".to_string())),
                    inventory_item_details::quantity.eq(rng.gen_range(10..500)),
                    inventory_item_details::reorder_level.eq(5),
                    inventory_item_details::unit_price.eq(100.0),
                    inventory_item_details::created_at.eq(Utc::now().naive_utc()),
                    inventory_item_details::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            context.inventory_item_ids.push(id);
        }

        // 3. resources
        println!("Seeding resources...");
        let resources_list = vec!["Main Hall", "Lab A", "Lab B", "Bus 1", "Bus 2", "Projector 1", "Camera 1", "Auditorium", "Swimming Pool", "Field"];
        for name in resources_list {
            let id = next_id(conn, IdPrefix::RESOURCE);
            insert_into(resources::table)
                .values(&Resource {
                    id: id.clone(),
                    resource_name: name.to_string(),
                    resource_type: "Venue".to_string(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;

            insert_into(resource_details::table)
                .values(&(
                    resource_details::resource_id.eq(id.clone()),
                    resource_details::status.eq("Available"),
                    resource_details::created_at.eq(Utc::now().naive_utc()),
                    resource_details::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            context.resource_ids.push(id);
        }

        // 4. resource_assets, asset_maintenance_logs, asset_allocations, maintenance_requests
        println!("Seeding detailed resource info...");
        for res_id in &context.resource_ids {
            for _ in 0..5 {
                insert_into(resource_assets::table)
                    .values((
                        resource_assets::id.eq(next_id(conn, IdPrefix::PROPERTY)),
                        resource_assets::resource_id.eq(res_id.clone()),
                        resource_assets::inventory_item_id.eq(get_random_id(&context.inventory_item_ids)),
                        resource_assets::quantity.eq(1.0),
                        resource_assets::created_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(conn)?;
            }
        }

        for item_id in &context.inventory_item_ids {
            insert_into(asset_maintenance_logs::table)
                .values((
                    asset_maintenance_logs::id.eq(next_id(conn, IdPrefix::PROPERTY)),
                    asset_maintenance_logs::item_id.eq(item_id.clone()),
                    asset_maintenance_logs::maintenance_date.eq(Utc::now().date_naive()),
                    asset_maintenance_logs::maintenance_type.eq("Corrective"),
                    asset_maintenance_logs::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(asset_allocations::table)
                .values((
                    asset_allocations::id.eq(next_id(conn, IdPrefix::PROPERTY_ALLOCATION)),
                    asset_allocations::item_id.eq(item_id.clone()),
                    asset_allocations::allocated_to_type.eq("Teacher"),
                    asset_allocations::allocated_to_id.eq(get_random_id(&context.staff_ids)),
                    asset_allocations::quantity.eq(1),
                    asset_allocations::allocation_date.eq(Utc::now().naive_utc()),
                    asset_allocations::allocated_by.eq(get_random_id(&context.user_ids)),
                    asset_allocations::created_at.eq(Utc::now().naive_utc()),
                    asset_allocations::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(maintenance_requests::table)
                .values((
                    maintenance_requests::id.eq(next_id(conn, IdPrefix::PROPERTY)),
                    maintenance_requests::item_id.eq(item_id.clone()),
                    maintenance_requests::issue_description.eq("Something broke"),
                    maintenance_requests::reported_by.eq(get_random_id(&context.user_ids)),
                    maintenance_requests::reported_date.eq(Utc::now().naive_utc()),
                    maintenance_requests::status.eq("Pending"),
                    maintenance_requests::created_at.eq(Utc::now().naive_utc()),
                    maintenance_requests::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        Ok(())
    }
}
