use diesel::prelude::*;
use diesel::insert_into;
use anyhow::Result;
use backend::schema::*;
use backend::config::Config;
use std::collections::HashSet;
use super::utils::*;
use super::{SeedModule, SeederContext};
use backend::models::behavior_management::BehaviorIncidentType;
use backend::models::behavior_management::BehaviorIncident;
use backend::models::behavior_management::DetentionBalance;
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
        seed_count_config: &crate::SeedCountConfig, // Add SeedCountConfig here
    ) -> Result<()> {
        println!("Seeding Behavior Management module...");

        let mut rng = rand::thread_rng();

        // Seed Behavior Incident Types
        let behavior_incident_types_data = (0..seed_count_config.behavior_incident_types).map(|i| {
            BehaviorIncidentType {
                id: generate_uuid(),
                type_name: format!("Incident Type {}", i + 1),
                default_points: rng.gen_range(1..=10),
                description: Some(format!("Description for Incident Type {}", i + 1)),
                created_at: random_datetime_in_past(2),
                updated_at: random_datetime_in_past(1),
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
            let behavior_incidents_data = (0..seed_count_config.behavior_incidents).map(|i| {
                BehaviorIncident {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    reported_by_user_id: get_random_id(&context.user_ids),
                    incident_type_id: get_random_id(&context.behavior_incident_type_ids),
                    description: format!("Student incident {}.", i + 1),
                    incident_date: random_datetime_in_past(1),
                    points_awarded: rng.gen_range(1..=10),
                    created_at: random_datetime_in_past(1),
                    updated_at: random_datetime_in_past(0),
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
            let num_detention_balances_to_seed = seed_count_config.students.min(context.student_ids.len());
            let mut detention_balances_data = Vec::new();
            let mut students_with_detention = HashSet::new();

            for _ in 0..num_detention_balances_to_seed {
                let student_id = get_random_id(&context.student_ids);
                // Ensure unique student_id for detention balances, as student_id is PK
                if students_with_detention.insert(student_id.clone()) {
                    let total_assigned = rng.gen_range(0.0..=20.0);
                    let total_served = rng.gen_range(0.0..=total_assigned);

                    detention_balances_data.push(DetentionBalance {
                        student_id,
                        total_hours_assigned: total_assigned,
                        total_hours_served: total_served,
                        remaining_hours: total_assigned - total_served,
                        updated_at: random_datetime_in_past(0),
                    });
                }
            }

            insert_into(detention_balances::table)
                .values(&detention_balances_data)
                .execute(conn)?;

            println!("Seeded {} detention balances.", detention_balances_data.len());
        }

        Ok(())
    }
}
