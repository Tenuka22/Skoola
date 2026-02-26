use crate::schema::{
    club_activities, club_members, clubs, competition_participants, competitions,
    cultural_event_participants, cultural_events, sport_event_participants, sport_events,
    sport_team_members, sport_teams, sports, student_achievements,
};
use crate::{AppState, errors::APIError, models::co_curricular::*};
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use uuid::Uuid;

// --- Sports Services ---

pub async fn create_sport(
    pool: web::Data<AppState>,
    req: CreateSportRequest,
) -> Result<Sport, APIError> {
    let mut conn = pool.db_pool.get()?;
    let new_sport = Sport {
        id: Uuid::new_v4().to_string(),
        sport_name: req.sport_name,
        description: req.description,
        category: req.category,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(sports::table)
        .values(&new_sport)
        .execute(&mut conn)?;

    Ok(new_sport)
}

pub async fn get_all_sports(pool: web::Data<AppState>) -> Result<Vec<Sport>, APIError> {
    let mut conn = pool.db_pool.get()?;
    let results = sports::table.load::<Sport>(&mut conn)?;
    Ok(results)
}

pub async fn create_sport_team(
    pool: web::Data<AppState>,
    req: CreateSportTeamRequest,
) -> Result<SportTeam, APIError> {
    let mut conn = pool.db_pool.get()?;
    let new_team = SportTeam {
        id: Uuid::new_v4().to_string(),
        sport_id: req.sport_id,
        team_name: req.team_name,
        grade_level: req.grade_level,
        coach_id: req.coach_id,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(sport_teams::table)
        .values(&new_team)
        .execute(&mut conn)?;

    Ok(new_team)
}

pub async fn add_sport_team_member(
    pool: web::Data<AppState>,
    team_id: String,
    req: AddSportTeamMemberRequest,
) -> Result<SportTeamMember, APIError> {
    let mut conn = pool.db_pool.get()?;
    let new_member = SportTeamMember {
        team_id,
        student_id: req.student_id,
        position: req.position,
        joined_date: req.joined_date,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(sport_team_members::table)
        .values(&new_member)
        .execute(&mut conn)?;

    Ok(new_member)
}

pub async fn create_sport_event(
    pool: web::Data<AppState>,
    req: CreateSportEventRequest,
) -> Result<SportEvent, APIError> {
    let mut conn = pool.db_pool.get()?;
    let new_event = SportEvent {
        id: Uuid::new_v4().to_string(),
        sport_id: req.sport_id,
        event_name: req.event_name,
        event_date: req.event_date,
        venue: req.venue,
        organizer: req.organizer,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(sport_events::table)
        .values(&new_event)
        .execute(&mut conn)?;

    Ok(new_event)
}

pub async fn record_sport_event_result(
    pool: web::Data<AppState>,
    event_id: String,
    req: RecordEventResultRequest,
) -> Result<SportEventParticipant, APIError> {
    let mut conn = pool.db_pool.get()?;
    let new_participant = SportEventParticipant {
        event_id,
        student_id: req.student_id,
        team_id: req.team_id,
        position: req.position,
        points: req.points,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(sport_event_participants::table)
        .values(&new_participant)
        .execute(&mut conn)?;

    Ok(new_participant)
}

// --- Clubs Services ---

pub async fn create_club(
    pool: web::Data<AppState>,
    req: CreateClubRequest,
) -> Result<Club, APIError> {
    let mut conn = pool.db_pool.get()?;
    let new_club = Club {
        id: Uuid::new_v4().to_string(),
        club_name: req.club_name,
        description: req.description,
        teacher_in_charge_id: req.teacher_in_charge_id,
        meeting_schedule: req.meeting_schedule,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(clubs::table)
        .values(&new_club)
        .execute(&mut conn)?;

    Ok(new_club)
}

pub async fn add_club_member(
    pool: web::Data<AppState>,
    club_id: String,
    req: AddClubMemberRequest,
) -> Result<ClubMember, APIError> {
    let mut conn = pool.db_pool.get()?;
    let new_member = ClubMember {
        club_id,
        student_id: req.student_id,
        role: req.role,
        joined_date: req.joined_date,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(club_members::table)
        .values(&new_member)
        .execute(&mut conn)?;

    Ok(new_member)
}

pub async fn create_club_activity(
    pool: web::Data<AppState>,
    req: CreateClubActivityRequest,
) -> Result<ClubActivity, APIError> {
    let mut conn = pool.db_pool.get()?;
    let new_activity = ClubActivity {
        id: Uuid::new_v4().to_string(),
        club_id: req.club_id,
        activity_name: req.activity_name,
        activity_date: req.activity_date,
        description: req.description,
        participants_count: req.participants_count,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(club_activities::table)
        .values(&new_activity)
        .execute(&mut conn)?;

    Ok(new_activity)
}

// --- Competitions Services ---

pub async fn create_competition(
    pool: web::Data<AppState>,
    req: CreateCompetitionRequest,
) -> Result<Competition, APIError> {
    let mut conn = pool.db_pool.get()?;
    let new_comp = Competition {
        id: Uuid::new_v4().to_string(),
        competition_name: req.competition_name,
        competition_type: req.competition_type,
        date: req.date,
        organizer: req.organizer,
        level: req.level,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(competitions::table)
        .values(&new_comp)
        .execute(&mut conn)?;

    Ok(new_comp)
}

pub async fn add_competition_participant(
    pool: web::Data<AppState>,
    competition_id: String,
    req: AddCompetitionParticipantRequest,
) -> Result<CompetitionParticipant, APIError> {
    let mut conn = pool.db_pool.get()?;
    let new_participant = CompetitionParticipant {
        competition_id,
        student_id: req.student_id,
        position: req.position,
        award: req.award,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(competition_participants::table)
        .values(&new_participant)
        .execute(&mut conn)?;

    Ok(new_participant)
}

pub async fn create_student_achievement(
    pool: web::Data<AppState>,
    req: CreateStudentAchievementRequest,
) -> Result<StudentAchievement, APIError> {
    let mut conn = pool.db_pool.get()?;
    let new_achievement = StudentAchievement {
        id: Uuid::new_v4().to_string(),
        student_id: req.student_id,
        achievement_type: req.achievement_type,
        description: req.description,
        date: req.date,
        certificate_url: req.certificate_url,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(student_achievements::table)
        .values(&new_achievement)
        .execute(&mut conn)?;

    Ok(new_achievement)
}

// --- Cultural Events Services ---

pub async fn create_cultural_event(
    pool: web::Data<AppState>,
    req: CreateCulturalEventRequest,
) -> Result<CulturalEvent, APIError> {
    let mut conn = pool.db_pool.get()?;
    let new_event = CulturalEvent {
        id: Uuid::new_v4().to_string(),
        event_name: req.event_name,
        event_date: req.event_date,
        venue: req.venue,
        description: req.description,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(cultural_events::table)
        .values(&new_event)
        .execute(&mut conn)?;

    Ok(new_event)
}

pub async fn add_cultural_event_participant(
    pool: web::Data<AppState>,
    event_id: String,
    req: AddCulturalEventParticipantRequest,
) -> Result<CulturalEventParticipant, APIError> {
    let mut conn = pool.db_pool.get()?;
    let new_participant = CulturalEventParticipant {
        event_id,
        student_id: req.student_id,
        performance_type: req.performance_type,
        role: req.role,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(cultural_event_participants::table)
        .values(&new_participant)
        .execute(&mut conn)?;

    Ok(new_participant)
}

pub async fn get_student_co_curricular_summary(
    pool: web::Data<AppState>,
    student_id: String,
) -> Result<StudentCoCurricularSummary, APIError> {
    let mut conn = pool.db_pool.get()?;

    let sports_participated = sport_team_members::table
        .filter(sport_team_members::student_id.eq(&student_id))
        .load::<SportTeamMember>(&mut conn)?;

    let clubs_joined = club_members::table
        .filter(club_members::student_id.eq(&student_id))
        .load::<ClubMember>(&mut conn)?;

    let achievements = student_achievements::table
        .filter(student_achievements::student_id.eq(&student_id))
        .load::<StudentAchievement>(&mut conn)?;

    Ok(StudentCoCurricularSummary {
        sports: sports_participated,
        clubs: clubs_joined,
        achievements: achievements,
    })
}
