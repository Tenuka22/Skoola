use crate::{
    AppState, errors::APIError, models::resources::co_curricular::*,
    services::resources::co_curricular,
};
use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, web};
// use serde_json; // Removed unused import

// --- Sports Handlers ---

#[api_operation(
    summary = "Create a new sport",
    description = "Creates a new sport category in the system.",
    tag = "co-curricular",
    operation_id = "create_sport"
)]
pub async fn create_sport(
    data: Data<AppState>,
    body: Json<CreateSportRequest>,
) -> Result<Json<Sport>, APIError> {
    let sport = co_curricular::create_sport(data, body.into_inner()).await?;
    Ok(Json(sport))
}

#[api_operation(
    summary = "Get all sports",
    description = "Retrieves a list of all sports available in the school.",
    tag = "co-curricular",
    operation_id = "get_all_sports"
)]
pub async fn get_all_sports(data: Data<AppState>) -> Result<Json<Vec<Sport>>, APIError> {
    let sports = co_curricular::get_all_sports(data).await?;
    Ok(Json(sports))
}

#[api_operation(
    summary = "Create a sport team",
    description = "Creates a new sport team for a specific sport and grade level.",
    tag = "co-curricular",
    operation_id = "create_sport_team"
)]
pub async fn create_sport_team(
    data: Data<AppState>,
    body: Json<CreateSportTeamRequest>,
) -> Result<Json<SportTeam>, APIError> {
    let team = co_curricular::create_sport_team(data, body.into_inner()).await?;
    Ok(Json(team))
}

#[api_operation(
    summary = "Add member to sport team",
    description = "Adds a student to a specific sport team.",
    tag = "co-curricular",
    operation_id = "add_sport_team_member"
)]
pub async fn add_sport_team_member(
    data: Data<AppState>,
    path: Path<String>,
    body: Json<AddSportTeamMemberRequest>,
) -> Result<Json<SportTeamMember>, APIError> {
    let team_id = path.into_inner();
    let member = co_curricular::add_sport_team_member(data, team_id, body.into_inner()).await?;
    Ok(Json(member))
}

#[api_operation(
    summary = "Create sport event",
    description = "Registers a new sport event (e.g., match, tournament).",
    tag = "co-curricular",
    operation_id = "create_sport_event"
)]
pub async fn create_sport_event(
    data: Data<AppState>,
    body: Json<CreateSportEventRequest>,
) -> Result<Json<SportEvent>, APIError> {
    let event = co_curricular::create_sport_event(data, body.into_inner()).await?;
    Ok(Json(event))
}

#[api_operation(
    summary = "Record sport event result",
    description = "Records the result or achievement of a participant in a sport event.",
    tag = "co-curricular",
    operation_id = "record_sport_event_result"
)]
pub async fn record_sport_event_result(
    data: Data<AppState>,
    path: Path<String>,
    body: Json<RecordEventResultRequest>,
) -> Result<Json<SportEventParticipant>, APIError> {
    let event_id = path.into_inner();
    let participant =
        co_curricular::record_sport_event_result(data, event_id, body.into_inner()).await?;
    Ok(Json(participant))
}

// --- Clubs Handlers ---

#[api_operation(
    summary = "Create a new club",
    description = "Creates a new school club or society.",
    tag = "co-curricular",
    operation_id = "create_club"
)]
pub async fn create_club(
    data: Data<AppState>,
    body: Json<CreateClubRequest>,
) -> Result<Json<Club>, APIError> {
    let club = co_curricular::create_club(data, body.into_inner()).await?;
    Ok(Json(club))
}

#[api_operation(
    summary = "Add member to club",
    description = "Adds a student as a member of a school club.",
    tag = "co-curricular",
    operation_id = "add_club_member"
)]
pub async fn add_club_member(
    data: Data<AppState>,
    path: Path<String>,
    body: Json<AddClubMemberRequest>,
) -> Result<Json<ClubMember>, APIError> {
    let club_id = path.into_inner();
    let member = co_curricular::add_club_member(data, club_id, body.into_inner()).await?;
    Ok(Json(member))
}

#[api_operation(
    summary = "Create club activity",
    description = "Logs a specific activity or meeting for a club.",
    tag = "co-curricular",
    operation_id = "create_club_activity"
)]
pub async fn create_club_activity(
    data: Data<AppState>,
    body: Json<CreateClubActivityRequest>,
) -> Result<Json<ClubActivity>, APIError> {
    let activity = co_curricular::create_club_activity(data, body.into_inner()).await?;
    Ok(Json(activity))
}

// --- Competitions Handlers ---

#[api_operation(
    summary = "Create a competition",
    description = "Registers a new school or inter-school competition.",
    tag = "co-curricular",
    operation_id = "create_competition"
)]
pub async fn create_competition(
    data: Data<AppState>,
    body: Json<CreateCompetitionRequest>,
) -> Result<Json<Competition>, APIError> {
    let comp = co_curricular::create_competition(data, body.into_inner()).await?;
    Ok(Json(comp))
}

#[api_operation(
    summary = "Add participant to competition",
    description = "Registers a student as a participant in a competition.",
    tag = "co-curricular",
    operation_id = "add_competition_participant"
)]
pub async fn add_competition_participant(
    data: Data<AppState>,
    path: Path<String>,
    body: Json<AddCompetitionParticipantRequest>,
) -> Result<Json<CompetitionParticipant>, APIError> {
    let competition_id = path.into_inner();
    let participant =
        co_curricular::add_competition_participant(data, competition_id, body.into_inner()).await?;
    Ok(Json(participant))
}

#[api_operation(
    summary = "Create student achievement",
    description = "Records an achievement or award earned by a student.",
    tag = "co-curricular",
    operation_id = "create_student_achievement"
)]
pub async fn create_student_achievement(
    data: Data<AppState>,
    body: Json<CreateStudentAchievementRequest>,
) -> Result<Json<StudentAchievement>, APIError> {
    let achievement = co_curricular::create_student_achievement(data, body.into_inner()).await?;
    Ok(Json(achievement))
}

// --- Cultural Handlers ---

#[api_operation(
    summary = "Create cultural event",
    description = "Registers a new cultural event (e.g., concert, exhibition).",
    tag = "co-curricular",
    operation_id = "create_cultural_event"
)]
pub async fn create_cultural_event(
    data: Data<AppState>,
    body: Json<CreateCulturalEventRequest>,
) -> Result<Json<CulturalEvent>, APIError> {
    let event = co_curricular::create_cultural_event(data, body.into_inner()).await?;
    Ok(Json(event))
}

#[api_operation(
    summary = "Add participant to cultural event",
    description = "Registers a student as a participant in a cultural event.",
    tag = "co-curricular",
    operation_id = "add_cultural_event_participant"
)]
pub async fn add_cultural_event_participant(
    data: Data<AppState>,
    path: Path<String>,
    body: Json<AddCulturalEventParticipantRequest>,
) -> Result<Json<CulturalEventParticipant>, APIError> {
    let event_id = path.into_inner();
    let participant =
        co_curricular::add_cultural_event_participant(data, event_id, body.into_inner()).await?;
    Ok(Json(participant))
}

#[api_operation(
    summary = "Get student co-curricular summary",
    description = "Retrieves a summary of all co-curricular activities and achievements for a specific student.",
    tag = "co-curricular",
    operation_id = "get_student_co_curricular_summary"
)]
pub async fn get_student_summary(
    data: Data<AppState>,
    path: Path<String>,
) -> Result<Json<StudentCoCurricularSummary>, APIError> {
    let student_id = path.into_inner();
    let summary = co_curricular::get_student_co_curricular_summary(data, student_id).await?;
    Ok(Json(summary))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/co-curricular")
            // Sports
            .route("/sports", web::post().to(create_sport))
            .route("/sports", web::get().to(get_all_sports))
            .route("/sports/teams", web::post().to(create_sport_team))
            .route(
                "/sports/teams/{team_id}/members",
                web::post().to(add_sport_team_member),
            )
            .route("/sports/events", web::post().to(create_sport_event))
            .route(
                "/sports/events/{event_id}/results",
                web::post().to(record_sport_event_result),
            )
            // Clubs
            .route("/clubs", web::post().to(create_club))
            .route("/clubs/{club_id}/members", web::post().to(add_club_member))
            .route("/clubs/activities", web::post().to(create_club_activity))
            // Competitions
            .route("/competitions", web::post().to(create_competition))
            .route(
                "/competitions/{id}/participants",
                web::post().to(add_competition_participant),
            )
            .route("/achievements", web::post().to(create_student_achievement))
            // Cultural
            .route("/cultural/events", web::post().to(create_cultural_event))
            .route(
                "/cultural/events/{id}/participants",
                web::post().to(add_cultural_event_participant),
            )
            // Summary
            .route(
                "/summary/student/{student_id}",
                web::get().to(get_student_summary),
            ),
    );
}
