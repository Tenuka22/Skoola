use actix_web::web::Json;
use actix_web::{web, HttpRequest};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::errors::APIError;
use crate::database::enums::AssessmentType;
use crate::models::exams::report_card::{ReportCard, ReportCardMark};
use crate::models::MessageResponse;
use crate::models::{ReportCardId, ReportCardMarkId, StudentId};
use crate::utils::jwt::UserId;
use crate::{services::exams::report_cards, AppState};

#[derive(Debug, Clone, Deserialize, JsonSchema, ApiComponent)]
pub struct ReportCardQuery {
    pub student_id: Option<String>,
    pub class_id: Option<String>,
    pub academic_year_id: Option<String>,
    pub term_id: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedReportCardResponse {
    pub data: Vec<ReportCard>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub next_last_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct ReportCardDetailResponse {
    pub report_card: ReportCard,
    pub marks: Vec<ReportCardMark>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateReportCardMarkRequest {
    pub subject_id: String,
    pub assessment_type: AssessmentType,
    pub assessment_id: String,
    pub marking_scheme_id: Option<String>,
    pub total_marks: Option<f32>,
    pub percentage: Option<f32>,
    pub grade: Option<String>,
    pub grade_point: Option<f32>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct CreateReportCardRequest {
    pub student_id: String,
    pub academic_year_id: String,
    pub term_id: String,
    pub class_id: String,
    pub grading_scheme_id: Option<String>,
    pub overall_percentage: Option<f32>,
    pub overall_grade: Option<String>,
    pub overall_gpa: Option<f32>,
    pub rank: Option<i32>,
    pub remarks: Option<String>,
    pub marks: Option<Vec<CreateReportCardMarkRequest>>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default)]
pub struct UpdateReportCardRequest {
    pub grading_scheme_id: Option<String>,
    pub overall_percentage: Option<f32>,
    pub overall_grade: Option<String>,
    pub overall_gpa: Option<f32>,
    pub rank: Option<i32>,
    pub remarks: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent, Default)]
pub struct UpdateReportCardMarkRequest {
    pub marking_scheme_id: Option<String>,
    pub total_marks: Option<f32>,
    pub percentage: Option<f32>,
    pub grade: Option<String>,
    pub grade_point: Option<f32>,
    pub remarks: Option<String>,
}

#[api_operation(
    summary = "Create report card",
    description = "Creates a report card with optional marks.",
    tag = "report-cards",
    operation_id = "create_report_card"
)]
pub async fn create_report_card(
    data: web::Data<AppState>,
    req: HttpRequest,
    body: web::Json<CreateReportCardRequest>,
) -> Result<Json<ReportCard>, APIError> {
    let user_id = UserId::from_request(&req)?;
    let created =
        report_cards::create_report_card(data, body.into_inner(), user_id.0).await?;
    Ok(Json(created))
}

#[api_operation(
    summary = "Get report card by ID",
    description = "Retrieves a report card and marks by ID.",
    tag = "report-cards",
    operation_id = "get_report_card_by_id"
)]
pub async fn get_report_card_by_id(
    data: web::Data<AppState>,
    path: web::Path<ReportCardId>,
) -> Result<Json<ReportCardDetailResponse>, APIError> {
    let (report_card, marks) =
        report_cards::get_report_card_by_id(data, path.into_inner().0).await?;
    Ok(Json(ReportCardDetailResponse {
        report_card,
        marks,
    }))
}

#[api_operation(
    summary = "Get all report cards",
    description = "Retrieves a paginated list of report cards.",
    tag = "report-cards",
    operation_id = "get_all_report_cards"
)]
pub async fn get_all_report_cards(
    data: web::Data<AppState>,
    query: web::Query<ReportCardQuery>,
) -> Result<Json<PaginatedReportCardResponse>, APIError> {
    let (items, total, total_pages) =
        report_cards::get_all_report_cards(data, query.clone().into_inner()).await?;
    let next_last_id = items.last().map(|i| i.id.clone());
    let limit = query.limit.unwrap_or(10);
    Ok(Json(PaginatedReportCardResponse {
        data: items,
        total,
        page: query.page.unwrap_or(1),
        limit,
        total_pages,
        next_last_id,
    }))
}

#[api_operation(
    summary = "Update report card",
    description = "Updates report card core fields.",
    tag = "report-cards",
    operation_id = "update_report_card"
)]
pub async fn update_report_card(
    data: web::Data<AppState>,
    path: web::Path<ReportCardId>,
    body: web::Json<UpdateReportCardRequest>,
) -> Result<Json<ReportCard>, APIError> {
    let updated =
        report_cards::update_report_card(data, path.into_inner().0, body.into_inner()).await?;
    Ok(Json(updated))
}

#[api_operation(
    summary = "Delete report card",
    description = "Deletes a report card by ID.",
    tag = "report-cards",
    operation_id = "delete_report_card"
)]
pub async fn delete_report_card(
    data: web::Data<AppState>,
    path: web::Path<ReportCardId>,
) -> Result<Json<MessageResponse>, APIError> {
    report_cards::delete_report_card(data, path.into_inner().0).await?;
    Ok(Json(MessageResponse {
        message: "Report card deleted successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Create report card mark",
    description = "Adds a mark to a report card.",
    tag = "report-cards",
    operation_id = "create_report_card_mark"
)]
pub async fn create_report_card_mark(
    data: web::Data<AppState>,
    path: web::Path<ReportCardId>,
    body: web::Json<CreateReportCardMarkRequest>,
) -> Result<Json<ReportCardMark>, APIError> {
    let created = report_cards::create_report_card_mark(
        data,
        path.into_inner().0,
        body.into_inner(),
    )
    .await?;
    Ok(Json(created))
}

#[api_operation(
    summary = "Update report card mark",
    description = "Updates a report card mark by ID.",
    tag = "report-cards",
    operation_id = "update_report_card_mark"
)]
pub async fn update_report_card_mark(
    data: web::Data<AppState>,
    path: web::Path<ReportCardMarkId>,
    body: web::Json<UpdateReportCardMarkRequest>,
) -> Result<Json<ReportCardMark>, APIError> {
    let updated = report_cards::update_report_card_mark(
        data,
        path.into_inner().0,
        body.into_inner(),
    )
    .await?;
    Ok(Json(updated))
}

#[api_operation(
    summary = "Delete report card mark",
    description = "Deletes a report card mark by ID.",
    tag = "report-cards",
    operation_id = "delete_report_card_mark"
)]
pub async fn delete_report_card_mark(
    data: web::Data<AppState>,
    path: web::Path<ReportCardMarkId>,
) -> Result<Json<MessageResponse>, APIError> {
    report_cards::delete_report_card_mark(data, path.into_inner().0).await?;
    Ok(Json(MessageResponse {
        message: "Report card mark deleted successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Generate report card",
    description = "Placeholder: generate report card for a student.",
    tag = "report-cards",
    operation_id = "generate_report_card"
)]
pub async fn generate_report_card(
    _path: web::Path<StudentId>,
) -> Result<Json<MessageResponse>, APIError> {
    Ok(Json(MessageResponse {
        message: "Report Card generation queued".to_string(),
    }))
}

pub fn config(cfg: &mut apistos::web::ServiceConfig) {
    cfg.service(
        apistos::web::resource("/report-cards")
            .route(apistos::web::post().to(create_report_card))
            .route(apistos::web::get().to(get_all_report_cards)),
    )
    .service(
        apistos::web::resource("/report-cards/{id}")
            .route(apistos::web::get().to(get_report_card_by_id))
            .route(apistos::web::put().to(update_report_card))
            .route(apistos::web::delete().to(delete_report_card)),
    )
    .service(
        apistos::web::resource("/report-cards/{report_card_id}/marks")
            .route(apistos::web::post().to(create_report_card_mark)),
    )
    .service(
        apistos::web::resource("/report-cards/marks/{id}")
            .route(apistos::web::put().to(update_report_card_mark))
            .route(apistos::web::delete().to(delete_report_card_mark)),
    )
    .service(
        apistos::web::resource("/report-cards/generate/{student_id}")
            .route(apistos::web::post().to(generate_report_card)),
    );
}
