use actix_web::web;
use apistos::api_operation; // Added
use actix_web::web::Json;
use crate::{
    AppState, // Changed from database::connection::DbPool
    errors::APIError,
    models::terms::{CreateTermRequest, TermResponse},
    services::terms,
};

#[api_operation( // Added
    summary = "Create Term",
    description = "Creates a new term.",
    tag = "terms"
)]
// Handler for creating a new term
pub async fn create_term_handler(
    app_state: web::Data<AppState>, // Changed parameter name and type
    new_term_req: web::Json<CreateTermRequest>,
) -> Result<Json<TermResponse>, APIError> {
    let term = terms::create_term(app_state, new_term_req.into_inner()).await?; // Passed app_state directly
    Ok(Json(term))
}
