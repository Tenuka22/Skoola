use diesel::prelude::*;
use diesel::insert_into;
use anyhow::Result;
use backend::schema::*;
use backend::config::Config;
use std::collections::HashSet;
use super::utils::*;
use super::{SeedModule, SeederContext};
use backend::models::system::activity::{ActivityType, Activity, ActivityParticipant, ActivityAttendance};
use backend::models::resources::co_curricular::{Sport, SportTeam, Club, ClubMember, ClubActivity, Competition, CompetitionParticipant, StudentAchievement, CulturalEvent, CulturalEventParticipant};
use backend::database::enums::{AttendanceStatus, ParticipantType};
use chrono::{Utc, NaiveDate};
use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Insertable)]
#[diesel(table_name = activity_participants_staff)]
pub struct ActivityParticipantStaff {
    pub activity_id: String,
    pub staff_id: String,
    pub participant_type: String,
    pub enrollment_reason: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = activity_participants_students)]
pub struct ActivityParticipantStudent {
    pub activity_id: String,
    pub student_id: String,
    pub participant_type: String,
    pub enrollment_reason: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

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
        seed_count_config: &crate::SeedCountConfig, // Add SeedCountConfig here
    ) -> Result<()> {
        println!("Seeding Extracurricular module...");

        let mut rng = rand::thread_rng();

        // 1. Activity Types
        let activity_types_data: Vec<ActivityType> = (0..seed_count_config.activity_types).map(|i| {
            ActivityType { id: generate_uuid(), name: format!("Activity Type {}", i + 1), description: Some(format!("Educational activity type {}", i + 1)), created_at: Utc::now().naive_utc() }
        }).collect();
        insert_into(activity_types::table)
            .values(&activity_types_data)
            .execute(conn)?;
        context.activity_type_ids = activity_types_data.into_iter().map(|at| at.id).collect();
        println!("Seeded {} activity types.", context.activity_type_ids.len());

        // 2. Activities
        if !context.activity_type_ids.is_empty() && !context.academic_year_ids.is_empty() && !context.user_ids.is_empty() {
            let mut activities_data = Vec::new();
            for i in 0..seed_count_config.activities {
                let id = generate_uuid();
                activities_data.push(Activity {
                    id: id.clone(),
                    activity_type_id: get_random_id(&context.activity_type_ids),
                    name: format!("Activity {}", i + 1),
                    description: Some(format!("Description for activity {}", i + 1)),
                    location: Some(format!("Venue {}", i + 1)),
                    start_time: random_datetime_in_past(1),
                    end_time: random_datetime_in_past(1),
                    is_mandatory: i % 2 == 0,
                    academic_year_id: get_random_id(&context.academic_year_ids),
                    created_by: get_random_id(&context.user_ids),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                });
                context.activity_ids.push(id);
            }
            insert_into(activities::table)
                .values(&activities_data)
                .execute(conn)?;
            println!("Seeded {} activities.", activities_data.len());
        }

        // 3. Activity Participants & Attendance
        if !context.activity_ids.is_empty() && !context.user_ids.is_empty() {
             let mut participants = Vec::new();
             let mut staff_participants = Vec::new();
             let mut student_participants = Vec::new();
             let mut attendance = Vec::new();

             for activity_id in &context.activity_ids {
                 // Each activity has configurable number of random participants
                 for _ in 0..seed_count_config.activity_participants_per_activity {
                     let user_id = get_random_id(&context.user_ids);
                     participants.push(ActivityParticipant {
                         activity_id: activity_id.clone(),
                         user_id: user_id.clone(),
                         participant_type: ParticipantType::Participant,
                         enrollment_reason: None,
                         created_at: Utc::now().naive_utc(),
                     });
                     attendance.push(ActivityAttendance {
                         id: generate_uuid(),
                         activity_id: activity_id.clone(),
                         user_id: user_id.clone(),
                         status: AttendanceStatus::Present,
                         check_in_time: None,
                         check_out_time: None,
                         remarks: None,
                         marked_by: get_random_id(&context.user_ids),
                         created_at: Utc::now().naive_utc(),
                         updated_at: Utc::now().naive_utc(),
                     });
                 }

                 // Staff junction
                 let staff_participants_count = (seed_count_config.activity_participants_per_activity / 2).max(1);
                 for _ in 0..staff_participants_count {
                     if !context.staff_ids.is_empty() {
                         staff_participants.push(ActivityParticipantStaff {
                             activity_id: activity_id.clone(),
                             staff_id: get_random_id(&context.staff_ids),
                             participant_type: "Supervisor".to_string(),
                             enrollment_reason: None,
                             created_at: Utc::now().naive_utc(),
                         });
                     }
                 }

                 // Student junction
                 let student_participants_count = seed_count_config.activity_participants_per_activity.saturating_sub(staff_participants_count);
                 for _ in 0..student_participants_count {
                     if !context.student_ids.is_empty() {
                         student_participants.push(ActivityParticipantStudent {
                             activity_id: activity_id.clone(),
                             student_id: get_random_id(&context.student_ids),
                             participant_type: "Participant".to_string(),
                             enrollment_reason: None,
                             created_at: Utc::now().naive_utc(),
                         });
                     }
                 }
             }
             
             // Deduplicate participants (activity_id, user_id)
             let mut unique_participants = Vec::new();
             let mut seen_participants = HashSet::new();
             for p in participants {
                 if seen_participants.insert((p.activity_id.clone(), p.user_id.clone())) {
                     unique_participants.push(p);
                 }
             }

             insert_into(activity_participants::table).values(&unique_participants).execute(conn)?;
             insert_into(activity_attendance::table).values(&attendance).execute(conn)?;

             let mut unique_staff_participants = Vec::new();
             let mut seen_staff_participants = HashSet::new();
             for p in staff_participants {
                 if seen_staff_participants.insert((p.activity_id.clone(), p.staff_id.clone())) {
                     unique_staff_participants.push(p);
                 }
             }
             insert_into(activity_participants_staff::table).values(&unique_staff_participants).execute(conn)?;

             let mut unique_student_participants = Vec::new();
             let mut seen_student_participants = HashSet::new();
             for p in student_participants {
                 if seen_student_participants.insert((p.activity_id.clone(), p.student_id.clone())) {
                     unique_student_participants.push(p);
                 }
             }
             insert_into(activity_participants_students::table).values(&unique_student_participants).execute(conn)?;

             println!("Seeded activity participants and attendance.");
        }

        // 4. Sports
        let sports_data: Vec<Sport> = (0..seed_count_config.sports).map(|i| {
            Sport { id: generate_uuid(), sport_name: format!("Sport {}", i + 1), description: Some(format!("Description for Sport {}", i + 1)), category: "Outdoor".to_string(), created_at: Utc::now().naive_utc(), updated_at: Utc::now().naive_utc() }
        }).collect();
        insert_into(sports::table).values(&sports_data).execute(conn)?;
        context.sport_ids = sports_data.into_iter().map(|s| s.id).collect();
        println!("Seeded {} sports.", context.sport_ids.len());

        // 5. Clubs
        if !context.staff_ids.is_empty() {
            let clubs_data: Vec<Club> = (0..seed_count_config.clubs).map(|i| {
                Club { id: generate_uuid(), club_name: format!("Club {}", i + 1), description: Some(format!("Exploring club {}", i + 1)), teacher_in_charge_id: get_random_id(&context.staff_ids), meeting_schedule: Some(format!("{}s 2pm", vec!["Mon", "Tue", "Wed", "Thu", "Fri"].choose(&mut rng).unwrap())), created_at: Utc::now().naive_utc(), updated_at: Utc::now().naive_utc() }
            }).collect();
            insert_into(clubs::table).values(&clubs_data).execute(conn)?;
            context.club_ids = clubs_data.into_iter().map(|c| c.id).collect();
            println!("Seeded {} clubs.", context.club_ids.len());

            // Club Activities
            let mut club_acts = Vec::new();
            for club_id in &context.club_ids {
                for i in 0..seed_count_config.club_activities_per_club {
                    club_acts.push(ClubActivity {
                        id: generate_uuid(),
                        club_id: club_id.clone(),
                        activity_name: format!("{} Activity {}", club_id, i + 1),
                        activity_date: Utc::now().naive_utc() - chrono::Duration::days(rng.gen_range(0..365)),
                        description: Some(format!("Discussion of goals for club activity {}", i + 1)),
                        participants_count: rng.gen_range(10..=30),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    });
                }
            }
            insert_into(club_activities::table).values(&club_acts).execute(conn)?;
            println!("Seeded {} club activities.", club_acts.len());
        }

        // 6. Sport Teams
        if !context.sport_ids.is_empty() && !context.staff_ids.is_empty() {
            let mut teams = Vec::new();
            for i in 0..seed_count_config.sport_teams {
                let id = generate_uuid();
                teams.push(SportTeam {
                    id: id.clone(),
                    sport_id: get_random_id(&context.sport_ids),
                    team_name: format!("Team {} - Under {}", i + 1, rng.gen_range(13..=18)),
                    grade_level: format!("Grade {}", rng.gen_range(8..=13)).to_string(),
                    coach_id: get_random_id(&context.staff_ids),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                });
                context.sport_team_ids.push(id);
            }
            insert_into(sport_teams::table).values(&teams).execute(conn)?;
            println!("Seeded {} sport teams.", teams.len());
        }

        // 7. Competitions
        let competitions_data: Vec<Competition> = (0..seed_count_config.competitions).map(|i| {
            Competition { id: generate_uuid(), competition_name: format!("Competition {}", i + 1), competition_type: if i % 2 == 0 { "Sports".to_string() } else { "Academic".to_string() }, date: Utc::now().naive_utc() - chrono::Duration::days(rng.gen_range(0..365)), organizer: format!("Organizer {}", i + 1), level: if i % 3 == 0 { "Provincial".to_string() } else { "National".to_string() }, created_at: Utc::now().naive_utc(), updated_at: Utc::now().naive_utc() }
        }).collect();
        insert_into(competitions::table).values(&competitions_data).execute(conn)?;
        context.competition_ids = competitions_data.into_iter().map(|c| c.id).collect();
        println!("Seeded {} competitions.", context.competition_ids.len());

        // 8. Cultural Events
        let cultural_events_data: Vec<CulturalEvent> = (0..seed_count_config.cultural_events).map(|i| {
            CulturalEvent { id: generate_uuid(), event_name: format!("Cultural Event {}", i + 1), event_date: Utc::now().naive_utc() - chrono::Duration::days(rng.gen_range(0..365)), venue: format!("Venue {}", i + 1), description: Some(format!("Description for cultural event {}", i + 1)), created_at: Utc::now().naive_utc(), updated_at: Utc::now().naive_utc() }
        }).collect();
        insert_into(cultural_events::table).values(&cultural_events_data).execute(conn)?;
        context.cultural_event_ids = cultural_events_data.into_iter().map(|e| e.id).collect();
        println!("Seeded {} cultural events.", context.cultural_event_ids.len());

        // 9. Participants for Clubs, Sports, Competitions, Cultural Events
        if !context.student_ids.is_empty() {
            // Club members
            if !context.club_ids.is_empty() {
                let mut members = Vec::new();
                let mut seen_club_members = HashSet::new();
                for club_id in &context.club_ids {
                    for _ in 0..seed_count_config.club_members_per_club {
                        let student_id = get_random_id(&context.student_ids);
                        if seen_club_members.insert((club_id.clone(), student_id.clone())) {
                            members.push(ClubMember {
                                club_id: club_id.clone(),
                                student_id: student_id.clone(),
                                role: "Member".to_string(),
                                joined_date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
                                created_at: Utc::now().naive_utc(),
                                updated_at: Utc::now().naive_utc(),
                            });
                        }
                    }
                }
                insert_into(club_members::table).values(&members).execute(conn)?;
                println!("Seeded {} club members.", members.len());
            }

            // Competition participants
            if !context.competition_ids.is_empty() {
                let mut comp_participants = Vec::new();
                let mut seen_comp_participants = HashSet::new();
                for competition_id in &context.competition_ids {
                    for _ in 0..seed_count_config.competition_participants_per_competition {
                        let student_id = get_random_id(&context.student_ids);
                        if seen_comp_participants.insert((competition_id.clone(), student_id.clone())) {
                            comp_participants.push(CompetitionParticipant {
                                competition_id: competition_id.clone(),
                                student_id: student_id.clone(),
                                position: Some(format!("{}{}", rng.gen_range(1..=3), "st".to_string())),
                                award: Some("Gold Medal".to_string()),
                                created_at: Utc::now().naive_utc(),
                                updated_at: Utc::now().naive_utc(),
                            });
                        }
                    }
                }
                insert_into(competition_participants::table).values(&comp_participants).execute(conn)?;
                println!("Seeded {} competition participants.", comp_participants.len());
            }

            // Cultural Event participants
            if !context.cultural_event_ids.is_empty() {
                let mut cultural_event_participants_data = Vec::new();
                let mut seen_cultural_participants = HashSet::new();
                for event_id in &context.cultural_event_ids {
                    for _ in 0..seed_count_config.cultural_event_participants_per_event {
                        let student_id = get_random_id(&context.student_ids);
                        if seen_cultural_participants.insert((event_id.clone(), student_id.clone())) {
                            cultural_event_participants_data.push(CulturalEventParticipant {
                                event_id: event_id.clone(),
                                student_id: student_id.clone(),
                                performance_type: "Dance".to_string(),
                                role: Some("Performer".to_string()),
                                created_at: Utc::now().naive_utc(),
                                updated_at: Utc::now().naive_utc(),
                            });
                        }
                    }
                }
                insert_into(cultural_event_participants::table).values(&cultural_event_participants_data).execute(conn)?;
                println!("Seeded {} cultural event participants.", cultural_event_participants_data.len());
            }

            // Student Achievements
            let mut achievements = Vec::new();
            for i in 0..seed_count_config.student_achievements {
                achievements.push(StudentAchievement {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    achievement_type: "Academic".to_string(),
                    description: format!("High distinction in science {}", i + 1),
                    date: NaiveDate::from_ymd_opt(2023, 12, 1).unwrap(),
                    certificate_url: None,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                });
            }
            insert_into(student_achievements::table).values(&achievements).execute(conn)?;
            println!("Seeded {} student achievements.", achievements.len());
        }

        Ok(())
    }
}
