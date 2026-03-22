use actix_web::web::{Data, Json, Path};
use apistos::api_operation;

use crate::AppState;
use crate::errors::APIError;
use crate::services::curriculum_management::pacing::{self, UnitPacingReport};
use crate::services::students::catch_up_notifications;

#[api_operation(
    summary = "Get Class Pacing Report",
    description = "Compares actual progress against planned unit allocations to identify if a class is behind schedule.",
    tag = "curriculum",
    operation_id = "get_class_pacing_report"
)]
pub async fn get_class_pacing_report(
    data: Data<AppState>,
    path: Path<String>, // class_id
) -> Result<Json<Vec<UnitPacingReport>>, APIError> {
    let report = pacing::get_class_pacing_report(data, path.into_inner()).await?;
    Ok(Json(report))
}

#[api_operation(
    summary = "Trigger Catch-up Notifications",
    description = "Manually triggers email notifications to guardians for any missed lessons that haven't been notified yet.",
    tag = "students",
    operation_id = "trigger_catch_up_notifications"
)]
pub async fn trigger_catch_up_notifications(
    data: Data<AppState>,
    path: Path<String>, // student_id
) -> Result<Json<i32>, APIError> {
    let count = catch_up_notifications::notify_guardians_of_missed_lessons(data, path.into_inner()).await?;
    Ok(Json(count))
}

