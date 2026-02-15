use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use actix_web::web;
use apistos::{api_operation, ApiComponent};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;
use actix_web::web::Json;

use crate::errors::APIError;
use crate::models::library::*;
use crate::models::MessageResponse;
use crate::services::library;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

// New Query, Paginated Response, and Bulk Request/Update structs for Library Categories
#[derive(Debug, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct LibraryCategoryQuery {
    pub search: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedLibraryCategoryResponse {
    pub data: Vec<LibraryCategory>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkDeleteLibraryCategoriesRequest {
    pub category_ids: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateLibraryCategoriesRequest {
    pub category_ids: Vec<i32>,
    pub category_name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema, ApiComponent, Clone)]
pub struct LibraryBookQuery {
    pub search: Option<String>,
    pub category_id: Option<i32>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedLibraryBookResponse {
    pub data: Vec<LibraryBookResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkDeleteLibraryBooksRequest {
    pub book_ids: Vec<i32>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateLibraryBooksRequest {
    pub book_ids: Vec<i32>,
    pub isbn: Option<String>,
    pub title: Option<String>,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub category_id: Option<i32>,
    pub quantity: Option<i32>,
    pub available_quantity: Option<i32>,
    pub rack_number: Option<String>,
}

// ============= Category Handlers =============

#[api_operation(
    summary = "Get all library categories",
    description = "Retrieves all book categories in the library with pagination, search, and sorting.",
    tag = "library",
    operation_id = "get_all_library_categories"
)]
pub async fn get_all_categories(
    pool: web::Data<DbPool>,
    query: web::Query<LibraryCategoryQuery>,
) -> Result<Json<PaginatedLibraryCategoryResponse>, APIError> {
    let inner_query = query.into_inner();
    let (categories, total_categories, total_pages) =
        library::get_all_categories_paginated(&pool, inner_query.clone()).await?;
    Ok(Json(PaginatedLibraryCategoryResponse {
        data: categories,
        total: total_categories,
        page: inner_query.page.unwrap_or(1),
        limit: inner_query.limit.unwrap_or(10),
        total_pages,
    }))
}

#[api_operation(
    summary = "Create a library category",
    description = "Creates a new book category.",
    tag = "library",
    operation_id = "create_library_category"
)]
pub async fn create_category(pool: web::Data<DbPool>, req: web::Json<CreateLibraryCategoryRequest>) -> Result<Json<LibraryCategory>, APIError> {
    let category = library::create_category(&pool, req.into_inner())?;
    Ok(Json(category))
}

#[api_operation(
    summary = "Bulk delete library categories",
    description = "Deletes multiple library categories by their IDs.",
    tag = "library",
    operation_id = "bulk_delete_library_categories"
)]
pub async fn bulk_delete_library_categories(
    pool: web::Data<DbPool>,
    body: web::Json<BulkDeleteLibraryCategoriesRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    library::bulk_delete_library_categories(&pool, body.into_inner().category_ids).await?;
    Ok(Json(MessageResponse { message: "Library categories deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Bulk update library categories",
    description = "Updates multiple library categories' information.",
    tag = "library",
    operation_id = "bulk_update_library_categories"
)]
pub async fn bulk_update_library_categories(
    pool: web::Data<DbPool>,
    body: web::Json<BulkUpdateLibraryCategoriesRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    library::bulk_update_library_categories(&pool, body.into_inner()).await?;
    Ok(Json(MessageResponse { message: "Library categories updated successfully".to_string() }))
}

// ============= Book Handlers =============

#[api_operation(
    summary = "Get all books",
    description = "Retrieves all books with their categories with pagination, search, and sorting.",
    tag = "library",
    operation_id = "get_all_library_books"
)]
pub async fn get_all_books(
    pool: web::Data<DbPool>,
    query: web::Query<LibraryBookQuery>,
) -> Result<Json<PaginatedLibraryBookResponse>, APIError> {
    let inner_query = query.into_inner();
    let (books, total_books, total_pages) =
        library::get_all_books_paginated(&pool, inner_query.clone()).await?;
    Ok(Json(PaginatedLibraryBookResponse {
        data: books,
        total: total_books,
        page: inner_query.page.unwrap_or(1),
        limit: inner_query.limit.unwrap_or(10),
        total_pages,
    }))
}

#[api_operation(
    summary = "Bulk delete library books",
    description = "Deletes multiple library books by their IDs.",
    tag = "library",
    operation_id = "bulk_delete_library_books"
)]
pub async fn bulk_delete_library_books(
    pool: web::Data<DbPool>,
    body: web::Json<BulkDeleteLibraryBooksRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    library::bulk_delete_library_books(&pool, body.into_inner().book_ids).await?;
    Ok(Json(MessageResponse { message: "Library books deleted successfully".to_string() }))
}

#[api_operation(
    summary = "Bulk update library books",
    description = "Updates multiple library books' information.",
    tag = "library",
    operation_id = "bulk_update_library_books"
)]
pub async fn bulk_update_library_books(
    pool: web::Data<DbPool>,
    body: web::Json<BulkUpdateLibraryBooksRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    library::bulk_update_library_books(&pool, body.into_inner()).await?;
    Ok(Json(MessageResponse { message: "Library books updated successfully".to_string() }))
}

#[api_operation(
    summary = "Get book by ID",
    description = "Retrieves a book by its serial ID.",
    tag = "library",
    operation_id = "get_library_book_by_id"
)]
pub async fn get_book_by_id(pool: web::Data<DbPool>, book_id: web::Path<i32>) -> Result<Json<LibraryBookResponse>, APIError> {
    let book = library::get_book_by_id(&pool, book_id.into_inner())?;
    Ok(Json(book))
}

#[api_operation(
    summary = "Search books",
    description = "Search for books by title, author, or ISBN.",
    tag = "library",
    operation_id = "search_library_books"
)]
pub async fn search_books(pool: web::Data<DbPool>, query: web::Query<std::collections::HashMap<String, String>>) -> Result<Json<Vec<LibraryBookResponse>>, APIError> {
    let search_query = query.get("q").ok_or_else(|| APIError::bad_request("Missing search query parameter 'q'"))?;
    let books = library::search_books(&pool, search_query)?;
    Ok(Json(books))
}

#[api_operation(
    summary = "Get books by category",
    description = "Retrieves all books in a specific category.",
    tag = "library",
    operation_id = "get_library_books_by_category"
)]
pub async fn get_books_by_category(pool: web::Data<DbPool>, category_id: web::Path<i32>) -> Result<Json<Vec<LibraryBookResponse>>, APIError> {
    let books = library::get_books_by_category(&pool, category_id.into_inner())?;
    Ok(Json(books))
}

#[api_operation(
    summary = "Add a book",
    description = "Adds a new book to the library.",
    tag = "library",
    operation_id = "create_library_book"
)]
pub async fn create_book(pool: web::Data<DbPool>, req: web::Json<CreateLibraryBookRequest>) -> Result<Json<LibraryBookResponse>, APIError> {
    let book = library::create_book(&pool, req.into_inner())?;
    Ok(Json(book))
}

#[api_operation(
    summary = "Update a book",
    description = "Updates an existing book.",
    tag = "library",
    operation_id = "update_library_book"
)]
pub async fn update_book(pool: web::Data<DbPool>, book_id: web::Path<i32>, req: web::Json<UpdateLibraryBookRequest>) -> Result<Json<LibraryBookResponse>, APIError> {
    let book = library::update_book(&pool, book_id.into_inner(), req.into_inner())?;
    Ok(Json(book))
}

#[api_operation(
    summary = "Delete a book",
    description = "Removes a book from the library.",
    tag = "library",
    operation_id = "delete_library_book"
)]
pub async fn delete_book(pool: web::Data<DbPool>, book_id: web::Path<i32>) -> Result<Json<MessageResponse>, APIError> {
    library::delete_book(&pool, book_id.into_inner())?;
    Ok(Json(MessageResponse { message: "Book deleted successfully".to_string() }))
}

// ============= Issue/Return Handlers =============

#[api_operation(
    summary = "Issue a book",
    description = "Issues a book to a student or staff member.",
    tag = "library",
    operation_id = "issue_library_book"
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
    tag = "library",
    operation_id = "return_library_book"
)]
pub async fn return_book(pool: web::Data<DbPool>, issue_id: web::Path<i32>, req: web::Json<ReturnBookRequest>) -> Result<Json<LibraryIssueResponse>, APIError> {
    let issue = library::return_book(&pool, issue_id.into_inner(), req.into_inner())?;
    Ok(Json(issue))
}

#[api_operation(
    summary = "Get issue record",
    description = "Retrieves a specific book issue record.",
    tag = "library",
    operation_id = "get_library_issue_by_id"
)]
pub async fn get_issue_by_id(pool: web::Data<DbPool>, issue_id: web::Path<i32>) -> Result<Json<LibraryIssueResponse>, APIError> {
    let issue = library::get_issue_by_id(&pool, issue_id.into_inner())?;
    Ok(Json(issue))
}

#[api_operation(
    summary = "Get student issues",
    description = "Retrieves all book issues for a specific student.",
    tag = "library",
    operation_id = "get_library_issues_by_student"
)]
pub async fn get_issued_books_by_student(pool: web::Data<DbPool>, student_id: web::Path<String>) -> Result<Json<Vec<LibraryIssueResponse>>, APIError> {
    let issues = library::get_issued_books_by_student(&pool, student_id.into_inner())?;
    Ok(Json(issues))
}

#[api_operation(
    summary = "Get staff issues",
    description = "Retrieves all book issues for a specific staff member.",
    tag = "library",
    operation_id = "get_library_issues_by_staff"
)]
pub async fn get_issued_books_by_staff(pool: web::Data<DbPool>, staff_id: web::Path<String>) -> Result<Json<Vec<LibraryIssueResponse>>, APIError> {
    let issues = library::get_issued_books_by_staff(&pool, staff_id.into_inner())?;
    Ok(Json(issues))
}

#[api_operation(
    summary = "Get overdue books",
    description = "Retrieves all books that are currently overdue.",
    tag = "library",
    operation_id = "get_overdue_library_books"
)]
pub async fn get_overdue_books(pool: web::Data<DbPool>) -> Result<Json<Vec<LibraryIssueResponse>>, APIError> {
    let issues = library::get_overdue_books(&pool)?;
    Ok(Json(issues))
}

// ============= Fine Handlers =============

#[api_operation(
    summary = "Pay a fine",
    description = "Records a payment for a library fine.",
    tag = "library",
    operation_id = "pay_library_fine"
)]
pub async fn pay_fine(pool: web::Data<DbPool>, issue_id: web::Path<i32>, req: web::Json<PayFineRequest>) -> Result<Json<LibraryIssueResponse>, APIError> {
    let issue = library::pay_fine(&pool, issue_id.into_inner(), req.into_inner())?;
    Ok(Json(issue))
}

#[api_operation(
    summary = "Waive a fine",
    description = "Waives a library fine for a specific issue.",
    tag = "library",
    operation_id = "waive_library_fine"
)]
pub async fn waive_fine(pool: web::Data<DbPool>, issue_id: web::Path<i32>) -> Result<Json<LibraryIssueResponse>, APIError> {
    let issue = library::waive_fine(&pool, issue_id.into_inner())?;
    Ok(Json(issue))
}

#[api_operation(
    summary = "Get fine history",
    description = "Retrieves all issues where a fine was recorded.",
    tag = "library",
    operation_id = "get_library_fine_history"
)]
pub async fn get_fine_history(pool: web::Data<DbPool>) -> Result<Json<Vec<LibraryIssueResponse>>, APIError> {
    let issues = library::get_fine_history(&pool)?;
    Ok(Json(issues))
}

// ============= Settings Handlers =============

#[api_operation(
    summary = "Get library settings",
    description = "Retrieves current library settings.",
    tag = "library",
    operation_id = "get_library_settings"
)]
pub async fn get_library_settings(pool: web::Data<DbPool>) -> Result<Json<LibrarySettings>, APIError> {
    let settings = library::get_library_settings(&pool)?;
    Ok(Json(settings))
}

#[api_operation(
    summary = "Update library settings",
    description = "Updates library settings.",
    tag = "library",
    operation_id = "update_library_settings"
)]
pub async fn update_library_settings(pool: web::Data<DbPool>, req: web::Json<UpdateLibrarySettingsRequest>) -> Result<Json<LibrarySettings>, APIError> {
    let settings = library::update_library_settings(&pool, req.into_inner())?;
    Ok(Json(settings))
}

// ============= Statistics Handlers =============

#[api_operation(
    summary = "Get library stats",
    description = "Retrieves library statistics.",
    tag = "library",
    operation_id = "get_library_stats"
)]
pub async fn get_library_stats(pool: web::Data<DbPool>) -> Result<Json<LibraryStatsResponse>, APIError> {
    let stats = library::get_library_stats(&pool)?;
    Ok(Json(stats))
}