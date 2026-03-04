use actix_web::web::{Data, Json, Path};
use apistos::api_operation;
use crate::AppState;
use crate::errors::APIError;
use crate::database::tables::LessonReview;
use crate::services::curriculum_management::reviews;
use crate::database::enums::ReviewerType;

#[derive(serde::Deserialize, schemars::JsonSchema, apistos::ApiComponent)]
pub struct SubmitReviewRequest {
    pub lesson_progress_id: String,
    pub rating: i32,
    pub feedback: Option<String>,
}

#[api_operation(
    summary = "Submit Lesson Review",
    description = "Allows a student or guardian to submit feedback on a lesson's clarity.",
    tag = "curriculum",
    operation_id = "submit_lesson_review"
)]
pub async fn submit_lesson_review(
    data: Data<AppState>,
    body: Json<SubmitReviewRequest>,
) -> Result<Json<LessonReview>, APIError> {
    // TODO: Get reviewer info from auth
    let reviewer_id = "reviewer_placeholder".to_string();
    let reviewer_type = ReviewerType::Student;
    
    let res = reviews::submit_review(data, body.lesson_progress_id.clone(), reviewer_id, reviewer_type, body.rating, body.feedback.clone()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Trigger Weekly Summary",
    description = "Manually triggers the sending of weekly lesson summaries and review requests.",
    tag = "curriculum",
    operation_id = "trigger_weekly_summary"
)]
pub async fn trigger_weekly_summary(
    data: Data<AppState>,
    path: Path<String>, // lesson_progress_id
) -> Result<Json<String>, APIError> {
    reviews::send_lesson_summary_and_review_request(data, path.into_inner()).await?;
    Ok(Json("Notifications sent successfully".to_string()))
}
