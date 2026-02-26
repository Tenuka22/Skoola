use crate::config::Config;
use crate::database::tables::{AssetCategory, InventoryItem};
use crate::errors::APIError;
use crate::faker::CustomFaker;
use crate::models::student::Student;
use crate::schema::{asset_categories, inventory_items, students};
use chrono::{Duration, Utc};
use diesel::SqliteConnection;
use diesel::prelude::*;
use uuid::Uuid;

pub fn seed_all(
    conn: &mut SqliteConnection,
    _app_config: &Config,
    staff_ids: &[String],
) -> Result<
    (
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    ),
    APIError,
> {
    let mut seeded_asset_category_ids = Vec::new();
    let mut seeded_inventory_item_ids = Vec::new();
    let seeded_maintenance_request_ids = Vec::new();
    let seeded_asset_allocation_ids = Vec::new();
    let seeded_uniform_item_ids = Vec::new();
    let seeded_uniform_issue_ids = Vec::new();

    let now = Utc::now().naive_utc();
    let two_years_ago = now - Duration::days(730);

    let students_data = students::table
        .select(Student::as_select())
        .load::<Student>(conn)?;
    let student_ids: Vec<String> = students_data.iter().map(|s| s.id.clone()).collect();

    if student_ids.is_empty() || staff_ids.is_empty() {
        return Ok((
            seeded_asset_category_ids,
            seeded_inventory_item_ids,
            seeded_maintenance_request_ids,
            seeded_asset_allocation_ids,
            seeded_uniform_item_ids,
            seeded_uniform_issue_ids,
        ));
    }

    // 1. Seed Asset Categories
    let category_names = vec![
        "Electronics",
        "Furniture",
        "Laboratory Equipment",
        "Sports Equipment",
        "Musical Instruments",
    ];
    let mut categories_to_insert = Vec::new();
    for name in category_names {
        let cat_id = Uuid::new_v4().to_string();
        let new_cat = AssetCategory {
            id: cat_id.clone(),
            name: name.to_string(),
            description: None,
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        categories_to_insert.push(new_cat);
        seeded_asset_category_ids.push(cat_id);
    }
    diesel::insert_into(asset_categories::table)
        .values(&categories_to_insert)
        .execute(conn)?;

    // 2. Seed Inventory Items
    let mut items_to_insert = Vec::new();
    for cat_id in &seeded_asset_category_ids {
        for i in 1..=3 {
            let item_id = Uuid::new_v4().to_string();
            let new_item = InventoryItem {
                id: item_id.clone(),
                category_id: cat_id.clone(),
                item_name: format!("Item {}-{}", cat_id, i),
                description: None,
                unit: "Piece".to_string(),
                quantity: rand::Rng::gen_range(&mut rand::thread_rng(), 10..100),
                reorder_level: 5,
                unit_price: rand::Rng::gen_range(&mut rand::thread_rng(), 50.0..500.0),
                created_at: CustomFaker::date_time_between(two_years_ago, now),
                updated_at: CustomFaker::date_time_between(two_years_ago, now),
            };
            items_to_insert.push(new_item);
            seeded_inventory_item_ids.push(item_id);
        }
    }
    diesel::insert_into(inventory_items::table)
        .values(&items_to_insert)
        .execute(conn)?;

    Ok((
        seeded_asset_category_ids,
        seeded_inventory_item_ids,
        seeded_maintenance_request_ids,
        seeded_asset_allocation_ids,
        seeded_uniform_item_ids,
        seeded_uniform_issue_ids,
    ))
}
