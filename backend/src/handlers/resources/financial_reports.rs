use actix_web::{web, HttpResponse};
use apistos::{api_operation, ApiComponent};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::{
    AppState,
    errors::APIError,
    services::finance::ledger,
};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, JsonSchema, ApiComponent)]
pub struct TrialBalanceResponse {
    pub account_balances: HashMap<String, f32>,
}

#[api_operation(
    summary = "Generate Trial Balance",
    description = "Generates a basic trial balance showing the balance of each account.",
    tag = "financial_reports",
    operation_id = "generate_trial_balance"
)]
pub async fn get_trial_balance(
    data: web::Data<AppState>,
) -> Result<web::Json<TrialBalanceResponse>, APIError> {
    let account_balances = ledger::generate_trial_balance(data).await?;
    Ok(web::Json(TrialBalanceResponse { account_balances }))
}
