use actix_web::web::{Data, Json, Query};
use apistos::api_operation;
use crate::AppState;
use crate::errors::APIError;
use crate::database::tables::SubstitutionPlan;
use crate::services::academic::substitution_plans;
use crate::database::enums::Medium;

#[derive(serde::Deserialize, schemars::JsonSchema, apistos::ApiComponent)]
pub struct CreateSubPlanRequest {
    pub subject_id: String,
    pub medium: Medium,
    pub plan_name: String,
    pub content_link: Option<String>,
    pub description: Option<String>,
}

#[derive(serde::Deserialize, schemars::JsonSchema, apistos::ApiComponent)]
pub struct GetSubPlansQuery {
    pub subject_id: String,
    pub medium: Medium,
}

#[api_operation(
    summary = "Create Substitution Plan",
    description = "Allows a teacher or manager to upload/link materials for future substitutions.",
    tag = "academic",
    operation_id = "create_substitution_plan"
)]
pub async fn create_substitution_plan(
    data: Data<AppState>,
    body: Json<CreateSubPlanRequest>,
) -> Result<Json<SubstitutionPlan>, APIError> {
    let res = substitution_plans::create_sub_plan(data, body.subject_id.clone(), body.medium.clone(), body.plan_name.clone(), body.content_link.clone(), body.description.clone()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get Substitution Plans",
    description = "Retrieves available plans for a specific subject and medium.",
    tag = "academic",
    operation_id = "get_substitution_plans"
)]
pub async fn get_substitution_plans(
    data: Data<AppState>,
    query: Query<GetSubPlansQuery>,
) -> Result<Json<Vec<SubstitutionPlan>>, APIError> {
    let res = substitution_plans::get_plans_for_subject(data, query.subject_id.clone(), query.medium.clone()).await?;
    Ok(Json(res))
}

