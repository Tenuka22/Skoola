use crate::{
    AppState,
    errors::APIError,
    models::academic::terms::{CreateTermRequest, TermResponse},
    services::academic::terms,
};
use actix_web::web;
use actix_web::web::Json;
use apistos::api_operation; // Added

#[api_operation(
    summary = "Create Term",
    description = "Creates a new term.",
    tag = "terms",
    operation_id = "create_term"
)]
// Handler for creating a new term
pub async fn create_term_handler(
    app_state: web::Data<AppState>, // Changed parameter name and type
    new_term_req: web::Json<CreateTermRequest>,
) -> Result<Json<TermResponse>, APIError> {
    let term = terms::create_term(app_state, new_term_req.into_inner()).await?; // Passed app_state directly
    Ok(Json(term))
}
