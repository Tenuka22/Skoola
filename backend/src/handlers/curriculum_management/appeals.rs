use actix_web::web::{Data, Json, Path};
use apistos::api_operation;
use crate::AppState;
use crate::errors::APIError;
use crate::database::tables::PracticalLessonAppeal;
use crate::services::curriculum_management::appeals;
use crate::database::enums::AppealStatus;

#[derive(serde::Deserialize, schemars::JsonSchema, apistos::ApiComponent)]
pub struct SubmitAppealRequest {
    pub lesson_progress_id: String,
    pub appeal_reason: String,
    pub evidence_image_url: Option<String>,
}

#[derive(serde::Deserialize, schemars::JsonSchema, apistos::ApiComponent)]
pub struct ReviewAppealRequest {
    pub status: AppealStatus,
}

#[api_operation(
    summary = "Submit Practical Lesson Appeal",
    description = "Allows a teacher to appeal a missed practical lesson check by providing evidence (e.g., photo of experiment).",
    tag = "curriculum",
    operation_id = "submit_practical_appeal"
)]
pub async fn submit_practical_appeal(
    data: Data<AppState>,
    body: Json<SubmitAppealRequest>,
) -> Result<Json<PracticalLessonAppeal>, APIError> {
    let res = appeals::submit_appeal(data, body.lesson_progress_id.clone(), body.appeal_reason.clone(), body.evidence_image_url.clone()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Review Practical Appeal",
    description = "Allows an admin or manager to approve/reject a practical lesson appeal.",
    tag = "curriculum",
    operation_id = "review_practical_appeal"
)]
pub async fn review_practical_appeal(
    data: Data<AppState>,
    path: Path<String>, // appeal_id
    body: Json<ReviewAppealRequest>,
) -> Result<Json<PracticalLessonAppeal>, APIError> {
    // TODO: Get admin_id from auth
    let admin_id = "admin_placeholder".to_string();
    let res = appeals::review_appeal(data, path.into_inner(), admin_id, body.status.clone()).await?;
    Ok(Json(res))
}
