use crate::{
    AppState, errors::APIError, models::resources::co_curricular::*,
};
use crate::services::resources::co_curricular::{
    SportService, SportTeamService, ClubService, CompetitionService,
    StudentAchievementService, CulturalEventService, ClubActivityService,
    self as co_curricular_service
};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use crate::database::enums::PermissionEnum;
use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, web};
use crate::create_admin_handlers;
use crate::services::admin_db::AdminQuery;

create_admin_handlers!(
    tag => "sports",
    entity => Sport,
    response => Sport,
    query => AdminQuery,
    create => CreateSportRequest,
    update => UpdateSportRequest,
    service => SportService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "sport_teams",
    entity => SportTeam,
    response => SportTeam,
    query => AdminQuery,
    create => CreateSportTeamRequest,
    update => UpdateSportTeamRequest,
    service => SportTeamService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "clubs",
    entity => Club,
    response => Club,
    query => AdminQuery,
    create => CreateClubRequest,
    update => UpdateClubRequest,
    service => ClubService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "competitions",
    entity => Competition,
    response => Competition,
    query => AdminQuery,
    create => CreateCompetitionRequest,
    update => UpdateCompetitionRequest,
    service => CompetitionService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "student_achievements",
    entity => StudentAchievement,
    response => StudentAchievement,
    query => AdminQuery,
    create => CreateStudentAchievementRequest,
    update => UpdateStudentAchievementRequest,
    service => StudentAchievementService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "cultural_events",
    entity => CulturalEvent,
    response => CulturalEvent,
    query => AdminQuery,
    create => CreateCulturalEventRequest,
    update => UpdateCulturalEventRequest,
    service => CulturalEventService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers!(
    tag => "club_activities",
    entity => ClubActivity,
    response => ClubActivity,
    query => AdminQuery,
    create => CreateClubActivityRequest,
    update => UpdateClubActivityRequest,
    service => ClubActivityService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

// --- Specialized Handlers ---

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
    let member = co_curricular_service::add_sport_team_member(data, team_id, body.into_inner()).await?;
    Ok(Json(member))
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
        co_curricular_service::record_sport_event_result(data, event_id, body.into_inner()).await?;
    Ok(Json(participant))
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
    let member = co_curricular_service::add_club_member(data, club_id, body.into_inner()).await?;
    Ok(Json(member))
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
        co_curricular_service::add_competition_participant(data, competition_id, body.into_inner()).await?;
    Ok(Json(participant))
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
        co_curricular_service::add_cultural_event_participant(data, event_id, body.into_inner()).await?;
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
    let summary = co_curricular_service::get_student_co_curricular_summary(data, student_id).await?;
    Ok(Json(summary))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/sports")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::CoCurricularManage })
            .route("", web::post().to(create_sport))
            .route("/{id}", web::get().to(get_sport_by_id))
            .route("", web::get().to(get_all_sport))
            .route("/{id}", web::put().to(update_sport))
            .route("/{id}", web::delete().to(delete_sport))
            .route("/bulk", web::delete().to(bulk_delete_sport))
            .route("/bulk", web::patch().to(bulk_update_sport)),
    )
    .service(
        web::scope("/sport-teams")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::CoCurricularManage })
            .route("", web::post().to(create_sport_team))
            .route("/{id}", web::get().to(get_sport_team_by_id))
            .route("", web::get().to(get_all_sport_team))
            .route("/{id}", web::put().to(update_sport_team))
            .route("/{id}", web::delete().to(delete_sport_team))
            .route("/bulk", web::delete().to(bulk_delete_sport_team))
            .route("/bulk", web::patch().to(bulk_update_sport_team)),
    )
    .service(
        web::scope("/clubs")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::CoCurricularManage })
            .route("", web::post().to(create_club))
            .route("/{id}", web::get().to(get_club_by_id))
            .route("", web::get().to(get_all_club))
            .route("/{id}", web::put().to(update_club))
            .route("/{id}", web::delete().to(delete_club))
            .route("/bulk", web::delete().to(bulk_delete_club))
            .route("/bulk", web::patch().to(bulk_update_club)),
    )
    .service(
        web::scope("/club-activities")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::CoCurricularManage })
            .route("", web::post().to(create_club_activity))
            .route("/{id}", web::get().to(get_club_activity_by_id))
            .route("", web::get().to(get_all_club_activity))
            .route("/{id}", web::put().to(update_club_activity))
            .route("/{id}", web::delete().to(delete_club_activity))
            .route("/bulk", web::delete().to(bulk_delete_club_activity))
            .route("/bulk", web::patch().to(bulk_update_club_activity)),
    )
    .service(
        web::scope("/co-curricular-ops")
            .wrap(Authenticated)
            .wrap(PermissionVerification { required_permission: PermissionEnum::CoCurricularManage })
            .route("/sports/teams/{team_id}/members", web::post().to(add_sport_team_member))
            .route("/sports/events/{event_id}/results", web::post().to(record_sport_event_result))
            .route("/clubs/{club_id}/members", web::post().to(add_club_member))
            .route("/competitions/{id}/participants", web::post().to(add_competition_participant))
            .route("/cultural/events/{id}/participants", web::post().to(add_cultural_event_participant))
            .route("/summary/student/{student_id}", web::get().to(get_student_summary)),
    );
}
