use crate::schema::{
    club_activities, club_members, clubs, competition_participants, competitions,
    cultural_event_participants, cultural_events, sport_event_participants, sport_teams,
    sport_team_members, sports, student_achievements,
};
use crate::{AppState, errors::APIError, models::resources::co_curricular::*};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::services::admin_db::AdminQuery;
use diesel::prelude::*;
use chrono::Utc;
use actix_web::web;
use crate::impl_admin_entity_service;

impl_admin_entity_service!(
    SportService,
    sports::table,
    Sport,
    Sport,
    sports::id,
    AdminQuery,
    |q: sports::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(sports::sport_name.like(search))
    },
    |q: sports::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(sports::created_at.desc())
    }
);

impl_admin_entity_service!(
    SportTeamService,
    sport_teams::table,
    SportTeam,
    SportTeam,
    sport_teams::id,
    AdminQuery,
    |q: sport_teams::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(sport_teams::team_name.like(search))
    },
    |q: sport_teams::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(sport_teams::created_at.desc())
    }
);

impl_admin_entity_service!(
    ClubService,
    clubs::table,
    Club,
    Club,
    clubs::id,
    AdminQuery,
    |q: clubs::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(clubs::club_name.like(search))
    },
    |q: clubs::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(clubs::created_at.desc())
    }
);

impl_admin_entity_service!(
    CompetitionService,
    competitions::table,
    Competition,
    Competition,
    competitions::id,
    AdminQuery,
    |q: competitions::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(competitions::competition_name.like(search))
    },
    |q: competitions::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(competitions::created_at.desc())
    }
);

impl_admin_entity_service!(
    StudentAchievementService,
    student_achievements::table,
    StudentAchievement,
    StudentAchievement,
    student_achievements::id,
    AdminQuery,
    |q: student_achievements::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(student_achievements::description.like(search))
    },
    |q: student_achievements::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(student_achievements::created_at.desc())
    }
);

impl_admin_entity_service!(
    CulturalEventService,
    cultural_events::table,
    CulturalEvent,
    CulturalEvent,
    cultural_events::id,
    AdminQuery,
    |q: cultural_events::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(cultural_events::event_name.like(search))
    },
    |q: cultural_events::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(cultural_events::created_at.desc())
    }
);

impl SportService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateSportRequest,
    ) -> Result<Sport, APIError> {
        let mut conn = data.db_pool.get()?;
        let new_sport = Sport {
            id: generate_prefixed_id(&mut conn, IdPrefix::CO_CURRICULAR)?,
            sport_name: req.sport_name,
            description: req.description,
            category: req.category,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_sport).await
    }
}

impl SportTeamService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateSportTeamRequest,
    ) -> Result<SportTeam, APIError> {
        let mut conn = data.db_pool.get()?;
        let new_team = SportTeam {
            id: generate_prefixed_id(&mut conn, IdPrefix::CO_CURRICULAR)?,
            sport_id: req.sport_id,
            team_name: req.team_name,
            grade_level: req.grade_level,
            coach_id: req.coach_id,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_team).await
    }
}

impl ClubService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateClubRequest,
    ) -> Result<Club, APIError> {
        let mut conn = data.db_pool.get()?;
        let new_club = Club {
            id: generate_prefixed_id(&mut conn, IdPrefix::CO_CURRICULAR)?,
            club_name: req.club_name,
            description: req.description,
            teacher_in_charge_id: req.teacher_in_charge_id,
            meeting_schedule: req.meeting_schedule,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_club).await
    }
}

impl CompetitionService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateCompetitionRequest,
    ) -> Result<Competition, APIError> {
        let mut conn = data.db_pool.get()?;
        let new_comp = Competition {
            id: generate_prefixed_id(&mut conn, IdPrefix::CO_CURRICULAR)?,
            competition_name: req.competition_name,
            competition_type: req.competition_type,
            date: req.date,
            organizer: req.organizer,
            level: req.level,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_comp).await
    }
}

impl StudentAchievementService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateStudentAchievementRequest,
    ) -> Result<StudentAchievement, APIError> {
        let mut conn = data.db_pool.get()?;
        let new_achievement = StudentAchievement {
            id: generate_prefixed_id(&mut conn, IdPrefix::CO_CURRICULAR)?,
            student_id: req.student_id,
            achievement_type: req.achievement_type,
            description: req.description,
            date: req.date,
            certificate_url: req.certificate_url,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_achievement).await
    }
}

impl CulturalEventService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateCulturalEventRequest,
    ) -> Result<CulturalEvent, APIError> {
        let mut conn = data.db_pool.get()?;
        let new_event = CulturalEvent {
            id: generate_prefixed_id(&mut conn, IdPrefix::CO_CURRICULAR)?,
            event_name: req.event_name,
            event_date: req.event_date,
            venue: req.venue,
            description: req.description,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_event).await
    }
}

// --- Specialized Services ---

impl_admin_entity_service!(
    ClubActivityService,
    club_activities::table,
    ClubActivity,
    ClubActivity,
    club_activities::id,
    AdminQuery,
    |q: club_activities::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(club_activities::activity_name.like(search))
    },
    |q: club_activities::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(club_activities::created_at.desc())
    }
);

impl ClubActivityService {
    pub async fn create_with_logic(
        data: web::Data<AppState>,
        req: CreateClubActivityRequest,
    ) -> Result<ClubActivity, APIError> {
        let mut conn = data.db_pool.get()?;
        let new_activity = ClubActivity {
            id: generate_prefixed_id(&mut conn, IdPrefix::CO_CURRICULAR)?,
            club_id: req.club_id,
            activity_name: req.activity_name,
            activity_date: req.activity_date,
            description: req.description,
            participants_count: req.participants_count,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        Self::generic_create(data, new_activity).await
    }

    pub async fn create_with_logic_admin(
        data: web::Data<AppState>,
        req: ClubActivity,
    ) -> Result<ClubActivity, APIError> {
        let mut conn = data.db_pool.get()?;
        let mut new_item = req;
        new_item.id = generate_prefixed_id(&mut conn, IdPrefix::CO_CURRICULAR)?;
        new_item.created_at = Utc::now().naive_utc();
        new_item.updated_at = Utc::now().naive_utc();
        Self::generic_create(data, new_item).await
    }
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
        id: generate_prefixed_id(&mut conn, IdPrefix::CO_CURRICULAR)?,
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
        representing_type: None,
        representing_id: None,
    };

    diesel::insert_into(competition_participants::table)
        .values(&new_participant)
        .execute(&mut conn)?;

    Ok(new_participant)
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
        achievements,
    })
}
