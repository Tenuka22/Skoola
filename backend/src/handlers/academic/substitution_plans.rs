use actix_web::web::{Data, Json, Query};
use apistos::api_operation;
use crate::AppState;
use crate::errors::APIError;
use crate::database::tables::SubstitutionPlan as SubstitutionPlanTable;
use crate::services::academic::substitution_plans;
use crate::services::academic::SubstitutionPlanService;
use crate::services::admin_db::AdminQuery;
use crate::database::enums::Medium;
use crate::create_admin_handlers;
use crate::models::academic::substitution_plans::{SubstitutionPlan as SubstitutionPlanModel, CreateSubstitutionPlanRequest};

create_admin_handlers!(
    tag => "substitution_plans",
    entity => SubstitutionPlan,
    response => SubstitutionPlanModel,
    query => AdminQuery,
    create => CreateSubstitutionPlanRequest,
    update => SubstitutionPlanTable,
    service => SubstitutionPlanService,
    methods => {
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
    }
);

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
    operation_id = "create_substitution_plan_manual"
)]
pub async fn create_substitution_plan_handler(
    data: Data<AppState>,
    body: Json<CreateSubPlanRequest>,
) -> Result<Json<SubstitutionPlanTable>, APIError> {
    let res = substitution_plans::create_sub_plan(data, body.subject_id.clone(), body.medium.clone(), body.plan_name.clone(), body.content_link.clone(), body.description.clone()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get Substitution Plans",
    description = "Retrieves available plans for a specific subject and medium.",
    tag = "academic",
    operation_id = "get_substitution_plans_manual"
)]
pub async fn get_substitution_plans_handler(
    data: Data<AppState>,
    query: Query<GetSubPlansQuery>,
) -> Result<Json<Vec<SubstitutionPlanTable>>, APIError> {
    let res = substitution_plans::get_plans_for_subject(data, query.subject_id.clone(), query.medium.clone()).await?;
    Ok(Json(res))
}
