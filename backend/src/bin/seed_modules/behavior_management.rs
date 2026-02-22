use diesel::prelude::*;
use diesel::insert_into;
use anyhow::Result;
use crate::schema::*;
use crate::Config;
use std::collections::HashSet;
use crate::bin::seed_modules::utils::*;
use crate::bin::seed_modules::{SeedModule, SeederContext};
use crate::models::{
    BehaviorIncidentType,
    BehaviorIncident,
    DetentionBalance,
};
use rand::Rng;

pub struct BehaviorManagementSeeder;

impl BehaviorManagementSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for BehaviorManagementSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
    ) -> Result<()> {
        println!("Seeding Behavior Management module...");

        // Seed Behavior Incident Types
        let behavior_incident_types_data = (1..=5).map(|i| {
            BehaviorIncidentType {
                id: generate_uuid(),
                type_name: format!("Incident Type {}", i),
                default_points: rand::thread_rng().gen_range(1..=10),
                description: Some(format!("Description for Incident Type {}", i)),
                created_at: Some(random_datetime_in_past(2)),
                updated_at: Some(random_datetime_in_past(1)),
            }
        }).collect::<Vec<BehaviorIncidentType>>();

        insert_into(behavior_incident_types::table)
            .values(&behavior_incident_types_data)
            .execute(conn)?;

        context.behavior_incident_type_ids = behavior_incident_types_data.into_iter().map(|t| t.id).collect();
        println!("Seeded {} behavior incident types.", context.behavior_incident_type_ids.len());

        // Seed Behavior Incidents
        if context.student_ids.is_empty() || context.user_ids.is_empty() || context.behavior_incident_type_ids.is_empty() {
            println!("Skipping BehaviorIncident seeding: student_ids, user_ids, or behavior_incident_type_ids are empty. Ensure relevant seeders run first.");
        } else {
            let behavior_incidents_data = (1..=20).map(|i| {
                BehaviorIncident {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    reported_by_user_id: get_random_id(&context.user_ids),
                    incident_type_id: get_random_id(&context.behavior_incident_type_ids),
                    description: format!("Student incident {}.", i),
                    incident_date: random_datetime_in_past(1),
                    points_awarded: rand::thread_rng().gen_range(1..=10),
                    created_at: Some(random_datetime_in_past(1)),
                    updated_at: Some(random_datetime_in_past(0)),
                }
            }).collect::<Vec<BehaviorIncident>>();

            insert_into(behavior_incidents::table)
                .values(&behavior_incidents_data)
                .execute(conn)?;

            println!("Seeded {} behavior incidents.", behavior_incidents_data.len());
        }

        // Seed Detention Balances
        if context.student_ids.is_empty() {
            println!("Skipping DetentionBalance seeding: student_ids are empty. Ensure relevant seeders run first.");
        } else {
            let detention_balances_data = (0..context.student_ids.len()).filter_map(|i| {
                let student_id = context.student_ids[i].clone();
                let total_assigned = rand::thread_rng().gen_range(0.0..=20.0);
                let total_served = rand::thread_rng().gen_range(0.0..=total_assigned);

                Some(DetentionBalance {
                    student_id,
                    total_hours_assigned: total_assigned,
                    total_hours_served: total_served,
                    remaining_hours: total_assigned - total_served,
                    updated_at: Some(random_datetime_in_past(0)),
                })
            }).collect::<Vec<DetentionBalance>>();

            // Filter out potential duplicates if any (though student_id is PK, context ensures unique student IDs)
            let unique_detention_balances: Vec<DetentionBalance> = detention_balances_data.into_iter()
                .fold(Vec::new(), |mut acc, item| {
                    if !acc.iter().any(|db: &DetentionBalance| db.student_id == item.student_id) {
                        acc.push(item);
                    }
                    acc
                });

            insert_into(detention_balances::table)
                .values(&unique_detention_balances)
                .execute(conn)?;

            println!("Seeded {} detention balances.", unique_detention_balances.len());
        }

        Ok(())
    }
}
