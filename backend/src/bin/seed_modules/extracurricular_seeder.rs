use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::tables::ActivityType;
use backend::models::{Sport, Club};
use backend::models::ids::IdPrefix;
use backend::schema::*;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use std::collections::HashSet;

pub struct ExtracurricularSeeder;

impl ExtracurricularSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for ExtracurricularSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Extracurricular module...");

        // 1. activity_types
        println!("Seeding activity_types...");
        let types = vec!["Workshop", "Seminar", "Field Trip", "Sport Day", "Prize Giving", "Concert", "Exhibition", "Debate", "Camp", "Tournament"];
        for name in types {
            let id = next_id(conn, IdPrefix::ACTIVITY);
            insert_into(activity_types::table)
                .values(&ActivityType {
                    id: id.clone(),
                    name: name.to_string(),
                    description: Some(format!("{} activity type", name)),
                    created_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;
            context.activity_type_ids.push(id);
        }

        // 2. sports
        println!("Seeding sports...");
        let sports_list = vec!["Cricket", "Football", "Netball", "Volleyball", "Swimming", "Athletics", "Table Tennis", "Badminton", "Chess", "Rugby"];
        for name in sports_list {
            let id = next_id(conn, IdPrefix::CO_CURRICULAR);
            insert_into(sports::table)
                .values(&Sport {
                    id: id.clone(),
                    sport_name: name.to_string(),
                    description: Some(format!("{} sport", name)),
                    category: "Standard".to_string(),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;
            context.sport_ids.push(id);
        }

        // 3. clubs
        println!("Seeding clubs...");
        let clubs_list = vec!["Science Club", "Interact Club", "Commerce Society", "Chess Club", "Drama Society", "Art Club", "Music Club", "IT Society", "Library Club", "Eco Club"];
        for name in clubs_list {
            let id = next_id(conn, IdPrefix::CO_CURRICULAR);
            insert_into(clubs::table)
                .values(&Club {
                    id: id.clone(),
                    club_name: name.to_string(),
                    description: Some(format!("{} club", name)),
                    teacher_in_charge_id: get_random_id(&context.staff_ids),
                    meeting_schedule: Some("Fridays at 2pm".to_string()),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;
            context.club_ids.push(id);
        }

        // 4. competitions & cultural_events
        println!("Seeding competitions and events...");
        for i in 0..20 {
            let comp_id = next_id(conn, IdPrefix::CO_CURRICULAR);
            insert_into(competitions::table)
                .values(&(
                    competitions::id.eq(comp_id.clone()),
                    competitions::competition_name.eq(format!("Competition {}", i)),
                    competitions::competition_type.eq("Inter-School"),
                    competitions::date.eq(Utc::now().naive_utc()),
                    competitions::organizer.eq("DOE"),
                    competitions::level.eq("Provincial"),
                    competitions::created_at.eq(Utc::now().naive_utc()),
                    competitions::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            let event_id = next_id(conn, IdPrefix::CO_CURRICULAR);
            insert_into(cultural_events::table)
                .values(&(
                    cultural_events::id.eq(event_id.clone()),
                    cultural_events::event_name.eq(format!("Cultural Event {}", i)),
                    cultural_events::event_date.eq(Utc::now().naive_utc()),
                    cultural_events::venue.eq("Main Hall"),
                    cultural_events::created_at.eq(Utc::now().naive_utc()),
                    cultural_events::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        // 5. activity_participants & activity_attendance
        println!("Seeding activity participation...");
        for act_id in context.activity_ids.iter().take(20) {
            for _ in 0..5 {
                insert_into(activity_participants::table)
                    .values(&(
                        activity_participants::activity_id.eq(act_id.clone()),
                        activity_participants::user_id.eq(get_random_id(&context.user_ids)),
                        activity_participants::participant_type.eq("Participant"),
                        activity_participants::created_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(conn).ok();

                insert_into(activity_attendance::table)
                    .values(&(
                        activity_attendance::id.eq(next_id(conn, IdPrefix::ACTIVITY)),
                        activity_attendance::activity_id.eq(act_id.clone()),
                        activity_attendance::user_id.eq(get_random_id(&context.user_ids)),
                        activity_attendance::status.eq("Present"),
                        activity_attendance::marked_by.eq(get_random_id(&context.user_ids)),
                        activity_attendance::created_at.eq(Utc::now().naive_utc()),
                        activity_attendance::updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(conn).ok();
            }
        }

        // 6. reward_types & adjustments
        println!("Seeding reward details...");
        let mut reward_type_ids = Vec::new();
        let rtypes = vec![("Extra Work", "Performance"), ("Innovative Idea", "Innovation")];
        for (name, cat) in rtypes {
            let rid = next_id(conn, IdPrefix::REWARD);
            insert_into(reward_types::table)
                .values(&(
                    reward_types::id.eq(rid.clone()),
                    reward_types::name.eq(name),
                    reward_types::category.eq(cat),
                    reward_types::default_points.eq(10),
                    reward_types::is_active.eq(true),
                    reward_types::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
            reward_type_ids.push(rid);
        }

        for s_id in &context.staff_ids {
            insert_into(teacher_reward_balances::table)
                .values(&(
                    teacher_reward_balances::teacher_id.eq(s_id.clone()),
                    teacher_reward_balances::total_points.eq(100),
                    teacher_reward_balances::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn).ok();

            for _ in 0..2 {
                let hist_id = next_id(conn, IdPrefix::REWARD);
                insert_into(teacher_reward_history::table)
                    .values(&(
                        teacher_reward_history::id.eq(hist_id.clone()),
                        teacher_reward_history::teacher_id.eq(s_id.clone()),
                        teacher_reward_history::points.eq(10),
                        teacher_reward_history::created_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(conn)?;

                insert_into(teacher_reward_details::table)
                    .values(&(
                        teacher_reward_details::reward_id.eq(hist_id),
                        teacher_reward_details::reason_type.eq("LessonCompleted"),
                        teacher_reward_details::reward_type_id.eq(Some(get_random_id(&reward_type_ids))),
                        teacher_reward_details::status.eq("Approved"),
                        teacher_reward_details::created_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(conn)?;
            }

            insert_into(reward_adjustments::table)
                .values(&(
                    reward_adjustments::id.eq(next_id(conn, IdPrefix::REWARD)),
                    reward_adjustments::teacher_id.eq(s_id.clone()),
                    reward_adjustments::adjustment_points.eq(5),
                    reward_adjustments::reason.eq(Some("Correction".to_string())),
                    reward_adjustments::approved_by.eq(Some(get_random_id(&context.user_ids))),
                    reward_adjustments::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        Ok(())
    }
}
