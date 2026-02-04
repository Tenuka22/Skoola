use actix_web::{web::{Data, Json, Path}, HttpResponse};
use apistos::{api_operation, web};
use crate::{
    AppState,
    errors::APIError,
    models::co_curricular::*,
    services::co_curricular,
};

// --- Sports Handlers ---

#[api_operation(summary = "Create a new sport", tag = "co-curricular")]
pub async fn create_sport(
    data: Data<AppState>,
    body: Json<CreateSportRequest>,
) -> Result<HttpResponse, APIError> {
    let sport = co_curricular::create_sport(data, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(sport))
}

#[api_operation(summary = "Get all sports", tag = "co-curricular")]
pub async fn get_all_sports(data: Data<AppState>) -> Result<HttpResponse, APIError> {
    let sports = co_curricular::get_all_sports(data).await?;
    Ok(HttpResponse::Ok().json(sports))
}

#[api_operation(summary = "Create a sport team", tag = "co-curricular")]
pub async fn create_sport_team(
    data: Data<AppState>,
    body: Json<CreateSportTeamRequest>,
) -> Result<HttpResponse, APIError> {
    let team = co_curricular::create_sport_team(data, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(team))
}

#[api_operation(summary = "Add member to sport team", tag = "co-curricular")]
pub async fn add_sport_team_member(
    data: Data<AppState>,
    path: Path<String>,
    body: Json<AddSportTeamMemberRequest>,
) -> Result<HttpResponse, APIError> {
    let team_id = path.into_inner();
    let member = co_curricular::add_sport_team_member(data, team_id, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(member))
}

#[api_operation(summary = "Create sport event", tag = "co-curricular")]
pub async fn create_sport_event(
    data: Data<AppState>,
    body: Json<CreateSportEventRequest>,
) -> Result<HttpResponse, APIError> {
    let event = co_curricular::create_sport_event(data, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(event))
}

#[api_operation(summary = "Record sport event result", tag = "co-curricular")]
pub async fn record_sport_event_result(
    data: Data<AppState>,
    path: Path<String>,
    body: Json<RecordEventResultRequest>,
) -> Result<HttpResponse, APIError> {
    let event_id = path.into_inner();
    let participant = co_curricular::record_sport_event_result(data, event_id, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(participant))
}

// --- Clubs Handlers ---

#[api_operation(summary = "Create a new club", tag = "co-curricular")]
pub async fn create_club(
    data: Data<AppState>,
    body: Json<CreateClubRequest>,
) -> Result<HttpResponse, APIError> {
    let club = co_curricular::create_club(data, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(club))
}

#[api_operation(summary = "Add member to club", tag = "co-curricular")]
pub async fn add_club_member(
    data: Data<AppState>,
    path: Path<String>,
    body: Json<AddClubMemberRequest>,
) -> Result<HttpResponse, APIError> {
    let club_id = path.into_inner();
    let member = co_curricular::add_club_member(data, club_id, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(member))
}

#[api_operation(summary = "Create club activity", tag = "co-curricular")]
pub async fn create_club_activity(
    data: Data<AppState>,
    body: Json<CreateClubActivityRequest>,
) -> Result<HttpResponse, APIError> {
    let activity = co_curricular::create_club_activity(data, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(activity))
}

// --- Competitions Handlers ---

#[api_operation(summary = "Create a competition", tag = "co-curricular")]
pub async fn create_competition(
    data: Data<AppState>,
    body: Json<CreateCompetitionRequest>,
) -> Result<HttpResponse, APIError> {
    let comp = co_curricular::create_competition(data, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(comp))
}

#[api_operation(summary = "Add participant to competition", tag = "co-curricular")]
pub async fn add_competition_participant(
    data: Data<AppState>,
    path: Path<String>,
    body: Json<AddCompetitionParticipantRequest>,
) -> Result<HttpResponse, APIError> {
    let competition_id = path.into_inner();
    let participant = co_curricular::add_competition_participant(data, competition_id, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(participant))
}

#[api_operation(summary = "Create student achievement", tag = "co-curricular")]
pub async fn create_student_achievement(
    data: Data<AppState>,
    body: Json<CreateStudentAchievementRequest>,
) -> Result<HttpResponse, APIError> {
    let achievement = co_curricular::create_student_achievement(data, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(achievement))
}

// --- Cultural Handlers ---

#[api_operation(summary = "Create cultural event", tag = "co-curricular")]
pub async fn create_cultural_event(
    data: Data<AppState>,
    body: Json<CreateCulturalEventRequest>,
) -> Result<HttpResponse, APIError> {
    let event = co_curricular::create_cultural_event(data, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(event))
}

#[api_operation(summary = "Add participant to cultural event", tag = "co-curricular")]
pub async fn add_cultural_event_participant(
    data: Data<AppState>,
    path: Path<String>,
    body: Json<AddCulturalEventParticipantRequest>,
) -> Result<HttpResponse, APIError> {
    let event_id = path.into_inner();
    let participant = co_curricular::add_cultural_event_participant(data, event_id, body.into_inner()).await?;
    Ok(HttpResponse::Created().json(participant))
}

#[api_operation(summary = "Get student co-curricular summary", tag = "co-curricular")]
pub async fn get_student_summary(
    data: Data<AppState>,
    path: Path<String>,
) -> Result<HttpResponse, APIError> {
    let student_id = path.into_inner();
    let summary = co_curricular::get_student_co_curricular_summary(data, student_id).await?;
    Ok(HttpResponse::Ok().json(summary))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/co-curricular")
            // Sports
            .route("/sports", web::post().to(create_sport))
            .route("/sports", web::get().to(get_all_sports))
            .route("/sports/teams", web::post().to(create_sport_team))
            .route("/sports/teams/{team_id}/members", web::post().to(add_sport_team_member))
            .route("/sports/events", web::post().to(create_sport_event))
            .route("/sports/events/{event_id}/results", web::post().to(record_sport_event_result))
            // Clubs
            .route("/clubs", web::post().to(create_club))
            .route("/clubs/{club_id}/members", web::post().to(add_club_member))
            .route("/clubs/activities", web::post().to(create_club_activity))
            // Competitions
            .route("/competitions", web::post().to(create_competition))
            .route("/competitions/{id}/participants", web::post().to(add_competition_participant))
            .route("/achievements", web::post().to(create_student_achievement))
            // Cultural
            .route("/cultural/events", web::post().to(create_cultural_event))
            .route("/cultural/events/{id}/participants", web::post().to(add_cultural_event_participant))
            // Summary
            .route("/summary/student/{student_id}", web::get().to(get_student_summary)),
    );
}
