use actix_web::web;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    AppState, // Changed from database::connection::DbPool
    errors::APIError,
    models::{
        academic_year::AcademicYear,
        terms::{CreateTermRequest, Term, TermResponse},
    },
    schema::{academic_years, terms},
};

pub async fn create_term(
    app_state: web::Data<AppState>, // Changed parameter name and type
    new_term_req: CreateTermRequest,
) -> Result<TermResponse, APIError> {
    let mut conn = app_state.db_pool.get().map_err(|e| APIError::internal(&format!("Failed to get DB connection: {:?}", e)))?; // Corrected connection acquisition
    
    // Check if the academic year exists
    let academic_year_exists: bool = academic_years::table
        .filter(academic_years::id.eq(&new_term_req.academic_year_id))
        .select(diesel::dsl::count(academic_years::id))
        .get_result::<i64>(&mut conn)
        .map_err(|e| APIError::internal(&format!("Database error: {:?}", e)))? > 0;

    if !academic_year_exists {
        return Err(APIError::bad_request("Academic year not found"));
    }

    // Check for duplicate term name within the academic year
    let duplicate_name: Option<Term> = terms::table
        .filter(terms::academic_year_id.eq(&new_term_req.academic_year_id))
        .filter(terms::name.eq(&new_term_req.name))
        .first::<Term>(&mut conn)
        .optional()
        .map_err(|e| APIError::internal(&format!("Database error: {:?}", e)))?;

    if duplicate_name.is_some() {
        return Err(APIError::bad_request("Term with this name already exists for the academic year"));
    }

    // Check for overlapping dates within the academic year
    let overlapping_term_exists: bool = terms::table
        .filter(terms::academic_year_id.eq(&new_term_req.academic_year_id))
        .filter(terms::start_date.le(&new_term_req.end_date))
        .filter(terms::end_date.ge(&new_term_req.start_date))
        .select(diesel::dsl::count(terms::id))
        .get_result::<i64>(&mut conn)
        .map_err(|e| APIError::internal(&format!("Database error: {:?}", e)))? > 0;

    if overlapping_term_exists {
        return Err(APIError::bad_request("Term dates overlap with an existing term in the academic year"));
    }

    let term_id = Uuid::new_v4().to_string();

    let new_term = Term {
        id: term_id,
        academic_year_id: new_term_req.academic_year_id.clone(),
        term_number: new_term_req.term_number,
        name: new_term_req.name.clone(),
        start_date: new_term_req.start_date,
        end_date: new_term_req.end_date,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    diesel::insert_into(terms::table)
        .values(&new_term)
        .execute(&mut conn)
        .map_err(|e| APIError::internal(&format!("Database error: {:?}", e)))?;

    Ok(new_term.into())
}