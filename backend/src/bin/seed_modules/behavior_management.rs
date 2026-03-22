use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::{BehaviorIncidentStatus};
use backend::models::behavior_management::{BehaviorIncident, BehaviorIncidentType, BehaviorIncidentDetail};
use backend::models::DetentionBalance;
use backend::models::ids::IdPrefix;
use backend::schema::*;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use std::collections::HashSet;

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
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Behavior Management module...");

        // 1. behavior_incident_severity_levels
        println!("Seeding severity levels...");
        let levels = vec![
            ("Low", 5),
            ("Medium", 15),
            ("High", 30),
            ("Critical", 50),
        ];
        let mut severity_ids = Vec::new();
        for (name, pts) in levels {
            let id = next_id(conn, IdPrefix::BEHAVIOR);
            insert_into(behavior_incident_severity_levels::table)
                .values(&(
                    behavior_incident_severity_levels::id.eq(id.clone()),
                    behavior_incident_severity_levels::name.eq(name),
                    behavior_incident_severity_levels::points.eq(pts),
                    behavior_incident_severity_levels::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
            severity_ids.push(id);
        }

        // 2. behavior_incident_types
        println!("Seeding behavior_incident_types...");
        let mut behavior_incident_type_ids = Vec::new();
        for i in 0..50 {
            let id = next_id(conn, IdPrefix::BEHAVIOR);
            insert_into(behavior_incident_types::table)
                .values(&BehaviorIncidentType {
                    id: id.clone(),
                    type_name: format!("Incident Type {}", i),
                    default_points: 10,
                    description: None,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;
            behavior_incident_type_ids.push(id);
        }

        // 3. behavior_incidents & details & participants & actions & followups
        println!("Seeding behavior_incidents...");
        for _ in 0..200 {
            let id = next_id(conn, IdPrefix::BEHAVIOR);
            insert_into(behavior_incidents::table)
                .values(&BehaviorIncident {
                    id: id.clone(),
                    student_id: get_random_id(&context.student_ids),
                    reported_by_user_id: get_random_id(&context.user_ids),
                    incident_type_id: get_random_id(&behavior_incident_type_ids),
                    incident_date: Utc::now().naive_utc(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;

            insert_into(behavior_incident_details::table)
                .values(&BehaviorIncidentDetail {
                    incident_id: id.clone(),
                    description: "Incident happened in class".to_string(),
                    points_awarded: 10,
                    severity_id: Some(get_random_id(&severity_ids)),
                    status: BehaviorIncidentStatus::Open.to_string(),
                    resolved_by: None,
                    resolved_at: None,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;

            insert_into(behavior_incident_participants::table)
                .values(&(
                    behavior_incident_participants::incident_id.eq(id.clone()),
                    behavior_incident_participants::participant_type.eq("Student"),
                    behavior_incident_participants::participant_id.eq(get_random_id(&context.student_ids)),
                    behavior_incident_participants::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(behavior_incident_actions::table)
                .values(&(
                    behavior_incident_actions::id.eq(next_id(conn, IdPrefix::BEHAVIOR)),
                    behavior_incident_actions::incident_id.eq(id.clone()),
                    behavior_incident_actions::action_type.eq("Warning"),
                    behavior_incident_actions::status.eq("Pending"),
                    behavior_incident_actions::created_at.eq(Utc::now().naive_utc()),
                    behavior_incident_actions::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(behavior_incident_followups::table)
                .values(&(
                    behavior_incident_followups::id.eq(next_id(conn, IdPrefix::BEHAVIOR)),
                    behavior_incident_followups::incident_id.eq(id.clone()),
                    behavior_incident_followups::followup_date.eq(Utc::now().date_naive()),
                    behavior_incident_followups::notes.eq(Some("Followup notes".to_string())),
                    behavior_incident_followups::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(behavior_incident_evidence::table)
                .values(&(
                    behavior_incident_evidence::id.eq(next_id(conn, IdPrefix::BEHAVIOR)),
                    behavior_incident_evidence::incident_id.eq(id),
                    behavior_incident_evidence::file_url.eq("http://example.com/evidence.jpg"),
                    behavior_incident_evidence::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        // 4. detention_balances
        println!("Seeding detention_balances...");
        let mut detentions = Vec::new();
        for stu_id in &context.student_ids {
            detentions.push(DetentionBalance {
                student_id: stu_id.clone(),
                total_hours_assigned: 0.0,
                total_hours_served: 0.0,
                remaining_hours: 0.0,
                updated_at: Utc::now().naive_utc(),
            });
        }
        insert_into(detention_balances::table).values(&detentions).execute(conn)?;

        Ok(())
    }
}
