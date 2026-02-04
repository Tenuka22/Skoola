use actix_web::web;
use apistos::api_operation;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use actix_web::web::Json;

use crate::errors::APIError;
use crate::models::library::*;
use crate::models::MessageResponse;
use crate::services::library;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

// ============= Category Handlers =============

#[api_operation(
    summary = "Get all library categories",
    description = "Retrieves all book categories in the library.",
    tag = "library"
)]
pub async fn get_all_categories(pool: web::Data<DbPool>) -> Result<Json<Vec<LibraryCategory>>, APIError> {
    let categories = library::get_all_categories(&pool)?;
    Ok(Json(categories))
}

#[api_operation(
    summary = "Create a library category",
    description = "Creates a new book category.",
    tag = "library"
)]
pub async fn create_category(pool: web::Data<DbPool>, req: web::Json<CreateLibraryCategoryRequest>) -> Result<Json<LibraryCategory>, APIError> {
    let category = library::create_category(&pool, req.into_inner())?;
    Ok(Json(category))
}

// ============= Book Handlers =============

#[api_operation(
    summary = "Get all books",
    description = "Retrieves all books with their categories.",
    tag = "library"
)]
pub async fn get_all_books(pool: web::Data<DbPool>) -> Result<Json<Vec<LibraryBookResponse>>, APIError> {
    let books = library::get_all_books(&pool)?;
    Ok(Json(books))
}

#[api_operation(
    summary = "Get book by ID",
    description = "Retrieves a book by its serial ID.",
    tag = "library"
)]
pub async fn get_book_by_id(pool: web::Data<DbPool>, book_id: web::Path<i32>) -> Result<Json<LibraryBookResponse>, APIError> {
    let book = library::get_book_by_id(&pool, book_id.into_inner())?;
    Ok(Json(book))
}

#[api_operation(
    summary = "Search books",
    description = "Search for books by title, author, or ISBN.",
    tag = "library"
)]
pub async fn search_books(pool: web::Data<DbPool>, query: web::Query<std::collections::HashMap<String, String>>) -> Result<Json<Vec<LibraryBookResponse>>, APIError> {
    let search_query = query.get("q").ok_or_else(|| APIError::bad_request("Missing search query parameter 'q'"))?;
    let books = library::search_books(&pool, search_query)?;
    Ok(Json(books))
}

#[api_operation(
    summary = "Get books by category",
    description = "Retrieves all books in a specific category.",
    tag = "library"
)]
pub async fn get_books_by_category(pool: web::Data<DbPool>, category_id: web::Path<i32>) -> Result<Json<Vec<LibraryBookResponse>>, APIError> {
    let books = library::get_books_by_category(&pool, category_id.into_inner())?;
    Ok(Json(books))
}

#[api_operation(
    summary = "Add a book",
    description = "Adds a new book to the library.",
    tag = "library"
)]
pub async fn create_book(pool: web::Data<DbPool>, req: web::Json<CreateLibraryBookRequest>) -> Result<Json<LibraryBookResponse>, APIError> {
    let book = library::create_book(&pool, req.into_inner())?;
    Ok(Json(book))
}

#[api_operation(
    summary = "Update a book",
    description = "Updates an existing book.",
    tag = "library"
)]
pub async fn update_book(pool: web::Data<DbPool>, book_id: web::Path<i32>, req: web::Json<UpdateLibraryBookRequest>) -> Result<Json<LibraryBookResponse>, APIError> {
    let book = library::update_book(&pool, book_id.into_inner(), req.into_inner())?;
    Ok(Json(book))
}

#[api_operation(
    summary = "Delete a book",
    description = "Removes a book from the library.",
    tag = "library"
)]
pub async fn delete_book(pool: web::Data<DbPool>, book_id: web::Path<i32>) -> Result<Json<MessageResponse>, APIError> {
    library::delete_book(&pool, book_id.into_inner())?;
    Ok(Json(MessageResponse { message: "Book deleted successfully".to_string() }))
}

// ============= Issue/Return Handlers =============

#[api_operation(
    summary = "Issue a book",
    description = "Issues a book to a student or staff member.",
    tag = "library"
)]
pub async fn issue_book(pool: web::Data<DbPool>, req: web::Json<IssueBookRequest>) -> Result<Json<LibraryIssueResponse>, APIError> {
    // TODO: Get actual staff ID from authentication context
    let issued_by_id = "1".to_string(); // Placeholder
    let issue = library::issue_book(&pool, req.into_inner(), issued_by_id)?;
    Ok(Json(issue))
}

#[api_operation(
    summary = "Return a book",
    description = "Records a book return and calculates fine if overdue.",
    tag = "library"
)]
pub async fn return_book(pool: web::Data<DbPool>, issue_id: web::Path<i32>, req: web::Json<ReturnBookRequest>) -> Result<Json<LibraryIssueResponse>, APIError> {
    let issue = library::return_book(&pool, issue_id.into_inner(), req.into_inner())?;
    Ok(Json(issue))
}

#[api_operation(
    summary = "Get issue record",
    description = "Retrieves a specific book issue record.",
    tag = "library"
)]
pub async fn get_issue_by_id(pool: web::Data<DbPool>, issue_id: web::Path<i32>) -> Result<Json<LibraryIssueResponse>, APIError> {
    let issue = library::get_issue_by_id(&pool, issue_id.into_inner())?;
    Ok(Json(issue))
}

#[api_operation(
    summary = "Get student issues",
    description = "Retrieves all book issues for a specific student.",
    tag = "library"
)]
pub async fn get_issued_books_by_student(pool: web::Data<DbPool>, student_id: web::Path<String>) -> Result<Json<Vec<LibraryIssueResponse>>, APIError> {
    let issues = library::get_issued_books_by_student(&pool, student_id.into_inner())?;
    Ok(Json(issues))
}

#[api_operation(
    summary = "Get staff issues",
    description = "Retrieves all book issues for a specific staff member.",
    tag = "library"
)]
pub async fn get_issued_books_by_staff(pool: web::Data<DbPool>, staff_id: web::Path<String>) -> Result<Json<Vec<LibraryIssueResponse>>, APIError> {
    let issues = library::get_issued_books_by_staff(&pool, staff_id.into_inner())?;
    Ok(Json(issues))
}

#[api_operation(
    summary = "Get overdue books",
    description = "Retrieves all books that are currently overdue.",
    tag = "library"
)]
pub async fn get_overdue_books(pool: web::Data<DbPool>) -> Result<Json<Vec<LibraryIssueResponse>>, APIError> {
    let issues = library::get_overdue_books(&pool)?;
    Ok(Json(issues))
}

// ============= Fine Handlers =============

#[api_operation(
    summary = "Pay a fine",
    description = "Records a payment for a library fine.",
    tag = "library"
)]
pub async fn pay_fine(pool: web::Data<DbPool>, issue_id: web::Path<i32>, req: web::Json<PayFineRequest>) -> Result<Json<LibraryIssueResponse>, APIError> {
    let issue = library::pay_fine(&pool, issue_id.into_inner(), req.into_inner())?;
    Ok(Json(issue))
}

#[api_operation(
    summary = "Waive a fine",
    description = "Waives a library fine for a specific issue.",
    tag = "library"
)]
pub async fn waive_fine(pool: web::Data<DbPool>, issue_id: web::Path<i32>) -> Result<Json<LibraryIssueResponse>, APIError> {
    let issue = library::waive_fine(&pool, issue_id.into_inner())?;
    Ok(Json(issue))
}

#[api_operation(
    summary = "Get fine history",
    description = "Retrieves all issues where a fine was recorded.",
    tag = "library"
)]
pub async fn get_fine_history(pool: web::Data<DbPool>) -> Result<Json<Vec<LibraryIssueResponse>>, APIError> {
    let issues = library::get_fine_history(&pool)?;
    Ok(Json(issues))
}

// ============= Settings Handlers =============

#[api_operation(
    summary = "Get library settings",
    description = "Retrieves current library settings.",
    tag = "library"
)]
pub async fn get_library_settings(pool: web::Data<DbPool>) -> Result<Json<LibrarySettings>, APIError> {
    let settings = library::get_library_settings(&pool)?;
    Ok(Json(settings))
}

#[api_operation(
    summary = "Update library settings",
    description = "Updates library settings.",
    tag = "library"
)]
pub async fn update_library_settings(pool: web::Data<DbPool>, req: web::Json<UpdateLibrarySettingsRequest>) -> Result<Json<LibrarySettings>, APIError> {
    let settings = library::update_library_settings(&pool, req.into_inner())?;
    Ok(Json(settings))
}

// ============= Statistics Handlers =============

#[api_operation(
    summary = "Get library stats",
    description = "Retrieves library statistics.",
    tag = "library"
)]
pub async fn get_library_stats(pool: web::Data<DbPool>) -> Result<Json<LibraryStatsResponse>, APIError> {
    let stats = library::get_library_stats(&pool)?;
    Ok(Json(stats))
}
