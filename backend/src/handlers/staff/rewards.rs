use actix_web::web::{Data, Json, Path};
use apistos::api_operation;
use crate::AppState;
use crate::errors::APIError;
use crate::services::staff::rewards;

#[api_operation(
    summary = "Get Teacher Reward Balance",
    description = "Retrieves the current reward points balance for a specific teacher.",
    tag = "staff",
    operation_id = "get_teacher_reward_balance"
)]
pub async fn get_teacher_reward_balance(
    data: Data<AppState>,
    path: Path<String>, // teacher_id
) -> Result<Json<i32>, APIError> {
    let balance = rewards::get_teacher_points(data, path.into_inner()).await?;
    Ok(Json(balance))
}
