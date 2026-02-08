use crate::config::Config;
use crate::errors::APIError;
use crate::faker::CustomFaker;
use crate::models::co_curricular::{
    Club, ClubMember, Competition, Sport, SportTeam, StudentAchievement,
};
use crate::models::staff::Staff;
use crate::models::student::Student;
use crate::schema::{
    club_members, clubs, competitions, sport_teams, sports, staff, student_achievements, students,
};
use chrono::{Duration, Utc};
use diesel::SqliteConnection;
use diesel::prelude::*;
use rand::seq::SliceRandom;
use uuid::Uuid;

pub fn seed_all(
    conn: &mut SqliteConnection,
    _app_config: &Config,
) -> Result<
    (
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
        Vec<String>,
    ),
    APIError,
> {
    let mut seeded_sport_ids = Vec::new();
    let mut seeded_sport_team_ids = Vec::new();
    let seeded_sport_team_member_ids = Vec::new();
    let seeded_sport_event_ids = Vec::new();
    let seeded_sport_event_participant_ids = Vec::new();
    let mut seeded_club_ids = Vec::new();
    let mut seeded_club_member_ids = Vec::new();
    let seeded_club_activity_ids = Vec::new();
    let mut seeded_competition_ids = Vec::new();
    let seeded_competition_participant_ids = Vec::new();
    let seeded_cultural_event_ids = Vec::new();
    let seeded_cultural_event_participant_ids = Vec::new();
    let mut seeded_student_achievement_ids = Vec::new();

    let now = Utc::now().naive_utc();
    let two_years_ago = now - Duration::days(730);

    // Get all students and staff for assignments
    let students_data = students::table.load::<Student>(conn)?;
    let student_ids: Vec<String> = students_data.iter().map(|s| s.id.clone()).collect();

    let staff_data = staff::table.load::<Staff>(conn)?;
    let staff_ids: Vec<String> = staff_data.iter().map(|s| s.id.clone()).collect();

    if student_ids.is_empty() || staff_ids.is_empty() {
        return Ok((
            seeded_sport_ids,
            seeded_sport_team_ids,
            seeded_sport_team_member_ids,
            seeded_sport_event_ids,
            seeded_sport_event_participant_ids,
            seeded_club_ids,
            seeded_club_member_ids,
            seeded_club_activity_ids,
            seeded_competition_ids,
            seeded_competition_participant_ids,
            seeded_cultural_event_ids,
            seeded_cultural_event_participant_ids,
            seeded_student_achievement_ids,
        ));
    }

    // 1. Seed Sports
    let sport_names = vec![
        "Cricket",
        "Football",
        "Volleyball",
        "Athletics",
        "Swimming",
        "Chess",
        "Badminton",
        "Table Tennis",
    ];
    let mut sports_to_insert = Vec::new();
    for name in sport_names {
        let sport_id = Uuid::new_v4().to_string();
        let new_sport = Sport {
            id: sport_id.clone(),
            sport_name: name.to_string(),
            description: None,
            category: "General".to_string(),
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        sports_to_insert.push(new_sport);
        seeded_sport_ids.push(sport_id);
    }
    diesel::insert_into(sports::table)
        .values(&sports_to_insert)
        .execute(conn)?;

    // 2. Seed Clubs
    let club_names = vec![
        "Science Club",
        "Interact Club",
        "Leo Club",
        "Drama Society",
        "Art Club",
        "Music Society",
        "Environment Club",
        "Coding Club",
    ];
    let mut clubs_to_insert = Vec::new();
    for name in club_names {
        let club_id = Uuid::new_v4().to_string();
        let new_club = Club {
            id: club_id.clone(),
            club_name: name.to_string(),
            description: None,
            teacher_in_charge_id: staff_ids.choose(&mut rand::thread_rng()).unwrap().clone(),
            meeting_schedule: Some("Every Wednesday".to_string()),
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        clubs_to_insert.push(new_club);
        seeded_club_ids.push(club_id);
    }
    diesel::insert_into(clubs::table)
        .values(&clubs_to_insert)
        .execute(conn)?;

    // 3. Seed Sport Teams
    let mut sport_teams_to_insert = Vec::new();
    for sport_id in &seeded_sport_ids {
        for grade_range in vec!["Junior", "Senior"] {
            let team_id = Uuid::new_v4().to_string();
            let new_team = SportTeam {
                id: team_id.clone(),
                sport_id: sport_id.clone(),
                team_name: format!(
                    "{} {} Team",
                    grade_range,
                    sports_to_insert
                        .iter()
                        .find(|s| &s.id == sport_id)
                        .unwrap()
                        .sport_name
                ),
                grade_level: grade_range.to_string(),
                coach_id: staff_ids.choose(&mut rand::thread_rng()).unwrap().clone(),
                created_at: CustomFaker::date_time_between(two_years_ago, now),
                updated_at: CustomFaker::date_time_between(two_years_ago, now),
            };
            sport_teams_to_insert.push(new_team);
            seeded_sport_team_ids.push(team_id);
        }
    }
    diesel::insert_into(sport_teams::table)
        .values(&sport_teams_to_insert)
        .execute(conn)?;

    // 4. Seed Club Members
    let mut club_members_to_insert = Vec::new();
    for club_id in &seeded_club_ids {
        let members_count =
            rand::seq::index::sample(&mut rand::thread_rng(), student_ids.len(), 15);
        for student_idx in members_count {
            let student_id = &student_ids[student_idx];
            let new_member = ClubMember {
                club_id: club_id.clone(),
                student_id: student_id.clone(),
                role: "Member".to_string(),
                joined_date: CustomFaker::date_time_between(two_years_ago, now).date(),
                created_at: CustomFaker::date_time_between(two_years_ago, now),
                updated_at: CustomFaker::date_time_between(two_years_ago, now),
            };
            club_members_to_insert.push(new_member);
            seeded_club_member_ids.push(format!("{}-{}", club_id, student_id));
        }
    }
    diesel::insert_into(club_members::table)
        .values(&club_members_to_insert)
        .execute(conn)?;

    // 5. Seed Competitions
    let mut competitions_to_insert = Vec::new();
    for i in 1..=5 {
        let comp_id = Uuid::new_v4().to_string();
        let new_comp = Competition {
            id: comp_id.clone(),
            competition_name: format!("Inter-School Competition {}", i),
            competition_type: "Academic".to_string(),
            date: CustomFaker::date_time_between(two_years_ago, now),
            organizer: "Ministry of Education".to_string(),
            level: "Provincial".to_string(),
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        competitions_to_insert.push(new_comp);
        seeded_competition_ids.push(comp_id);
    }
    diesel::insert_into(competitions::table)
        .values(&competitions_to_insert)
        .execute(conn)?;

    // 6. Seed Student Achievements
    let mut achievements_to_insert = Vec::new();
    for student_id in student_ids.choose_multiple(&mut rand::thread_rng(), 20) {
        let achievement_id = Uuid::new_v4().to_string();
        let new_achievement = StudentAchievement {
            id: achievement_id.clone(),
            student_id: student_id.clone(),
            achievement_type: "Award".to_string(),
            description: "Winner of the Annual Science Fair".to_string(),
            date: CustomFaker::date_time_between(two_years_ago, now).date(),
            certificate_url: None,
            created_at: CustomFaker::date_time_between(two_years_ago, now),
            updated_at: CustomFaker::date_time_between(two_years_ago, now),
        };
        achievements_to_insert.push(new_achievement);
        seeded_student_achievement_ids.push(achievement_id);
    }
    diesel::insert_into(student_achievements::table)
        .values(&achievements_to_insert)
        .execute(conn)?;

    Ok((
        seeded_sport_ids,
        seeded_sport_team_ids,
        seeded_sport_team_member_ids,
        seeded_sport_event_ids,
        seeded_sport_event_participant_ids,
        seeded_club_ids,
        seeded_club_member_ids,
        seeded_club_activity_ids,
        seeded_competition_ids,
        seeded_competition_participant_ids,
        seeded_cultural_event_ids,
        seeded_cultural_event_participant_ids,
        seeded_student_achievement_ids,
    ))
}
