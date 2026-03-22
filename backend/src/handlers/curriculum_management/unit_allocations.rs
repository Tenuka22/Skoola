use actix_web::web::{Data, Json, Path};
use apistos::api_operation;

use crate::AppState;
use crate::errors::APIError;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema, apistos::ApiComponent)]
pub struct UnitAllocationResponse {
    pub id: String,
    pub class_id: String,
    pub curriculum_topic_id: String,
    pub planned_periods: i32,
    pub buffer_periods: i32,
    pub target_date: Option<chrono::NaiveDate>,
}

#[derive(serde::Deserialize, schemars::JsonSchema, apistos::ApiComponent)]
pub struct CreateUnitAllocationRequest {
    pub class_id: String,
    pub curriculum_topic_id: String,
    pub planned_periods: i32,
    pub buffer_periods: i32,
    pub target_date: Option<chrono::NaiveDate>,
}

#[api_operation(
    summary = "Create Unit Allocation",
    description = "Compatibility endpoint. Unit allocations were moved during schema refactor.",
    tag = "curriculum",
    operation_id = "create_unit_allocation"
)]
pub async fn create_unit_allocation(
    _data: Data<AppState>,
    _body: Json<CreateUnitAllocationRequest>,
) -> Result<Json<UnitAllocationResponse>, APIError> {
    Err(APIError::bad_request(
        "Unit allocation storage has been refactored and this endpoint is pending remap.",
    ))
}

#[api_operation(
    summary = "Get Unit Allocations by Class",
    description = "Compatibility endpoint.",
    tag = "curriculum",
    operation_id = "get_unit_allocations_by_class"
)]
pub async fn get_unit_allocations_by_class(
    _data: Data<AppState>,
    _path: Path<String>,
) -> Result<Json<Vec<UnitAllocationResponse>>, APIError> {
    Ok(Json(Vec::new()))
}

