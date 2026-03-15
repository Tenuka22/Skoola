use actix_web::web::{Data, Json, Path, Query};
use apistos::{web as apistos_web, api_operation};
use crate::errors::APIError;
use crate::models::MessageResponse;
use crate::models::resources::library::*;
use crate::services::resources::library::{
    LibraryCategoryService, LibraryBookService, LibraryIssueService, LibrarySettingsService,
    self as library_service
};
use crate::utils::jwt::UserId;
use crate::AppState;
use crate::services::admin_db::AdminQuery;

use crate::create_admin_handlers_i32;

create_admin_handlers_i32!(
    tag => "library_settings",
    entity => LibrarySettings,
    response => LibrarySettingsResponse,
    query => LibrarySettingsQuery,
    create => LibrarySettings,
    update => LibrarySettings, // Placeholder
    service => LibrarySettingsService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers_i32!(
    tag => "library_categories",
    entity => LibraryCategory,
    response => LibraryCategory,
    query => LibraryCategoryQuery,
    create => CreateLibraryCategoryRequest,
    update => UpdateLibraryCategoryRequest,
    service => LibraryCategoryService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers_i32!(
    tag => "library_books",
    entity => LibraryBook,
    response => LibraryBook,
    query => LibraryBookQuery,
    create => CreateLibraryBookRequest,
    update => UpdateLibraryBookRequest,
    service => LibraryBookService,
    methods => {
        create => create_with_logic,
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

create_admin_handlers_i32!(
    tag => "library_issues",
    entity => LibraryIssue,
    response => LibraryIssue,
    query => AdminQuery,
    create => IssueBookRequest,
    update => UpdateLibraryIssueRequest,
    service => LibraryIssueService,
    methods => {
        get_by_id => generic_get_by_id,
        get_all => generic_get_all,
        update => generic_update,
        delete => generic_delete,
        bulk_delete => generic_bulk_delete,
        bulk_update => generic_bulk_update
    }
);

#[api_operation(
    summary = "Issue a book",
    tag = "Library Ops",
    operation_id = "issue_book"
)]
pub async fn issue_book(
    data: Data<AppState>,
    user_id: UserId,
    req: Json<IssueBookRequest>,
) -> Result<Json<LibraryIssue>, APIError> {
    let res = library_service::issue_book(data, req.into_inner(), user_id.0).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Return a book",
    tag = "Library Ops",
    operation_id = "return_book"
)]
pub async fn return_book(
    data: Data<AppState>,
    path: Path<i32>,
) -> Result<Json<LibraryIssueResponse>, APIError> {
    let res = library_service::return_book(data, path.into_inner()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Renew a book",
    tag = "Library Ops",
    operation_id = "renew_book"
)]
pub async fn renew_book(
    data: Data<AppState>,
    path: Path<i32>,
) -> Result<Json<LibraryIssueResponse>, APIError> {
    let res = library_service::renew_book(data, path.into_inner()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Pay a fine",
    tag = "Library Ops",
    operation_id = "pay_fine"
)]
pub async fn pay_fine(
    data: Data<AppState>,
    path: Path<i32>,
) -> Result<Json<LibraryIssueResponse>, APIError> {
    let res = library_service::pay_fine(data, path.into_inner()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Waive a fine",
    tag = "Library Ops",
    operation_id = "waive_fine"
)]
pub async fn waive_fine(
    data: Data<AppState>,
    path: Path<i32>,
) -> Result<Json<LibraryIssueResponse>, APIError> {
    let res = library_service::waive_fine(data, path.into_inner()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Check overdue books",
    tag = "Library Ops",
    operation_id = "check_overdue"
)]
pub async fn check_overdue(
    data: Data<AppState>,
) -> Result<Json<MessageResponse>, APIError> {
    library_service::check_overdue_books(data).await?;
    Ok(Json(MessageResponse { message: "Overdue books checked successfully".into() }))
}

#[api_operation(
    summary = "Search books",
    tag = "Library Ops",
    operation_id = "search_books"
)]
pub async fn search_books(
    data: Data<AppState>,
    query: Query<LibraryBookQuery>,
) -> Result<Json<Vec<LibraryBookResponse>>, APIError> {
    let res = library_service::search_books(data, query.into_inner()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get books by category",
    tag = "Library Ops",
    operation_id = "get_books_by_category"
)]
pub async fn get_books_by_category(
    data: Data<AppState>,
    path: Path<i32>,
) -> Result<Json<Vec<LibraryBookResponse>>, APIError> {
    let res = library_service::get_books_by_category(data, path.into_inner()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get issued books by student",
    tag = "Library Ops",
    operation_id = "get_issued_books_by_student"
)]
pub async fn get_issued_books_by_student(
    data: Data<AppState>,
    path: Path<String>,
) -> Result<Json<Vec<LibraryIssueResponse>>, APIError> {
    let res = library_service::get_issued_books_by_student(data, path.into_inner()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get issued books by staff",
    tag = "Library Ops",
    operation_id = "get_issued_books_by_staff"
)]
pub async fn get_issued_books_by_staff(
    data: Data<AppState>,
    path: Path<String>,
) -> Result<Json<Vec<LibraryIssueResponse>>, APIError> {
    let res = library_service::get_issued_books_by_staff(data, path.into_inner()).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get overdue books",
    tag = "Library Ops",
    operation_id = "get_overdue_books"
)]
pub async fn get_overdue_books(
    data: Data<AppState>,
) -> Result<Json<Vec<LibraryIssueResponse>>, APIError> {
    let res = library_service::get_overdue_books(data).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get fine history",
    tag = "Library Ops",
    operation_id = "get_fine_history"
)]
pub async fn get_fine_history(
    data: Data<AppState>,
) -> Result<Json<Vec<LibraryIssueResponse>>, APIError> {
    let res = library_service::get_fine_history(data).await?;
    Ok(Json(res))
}

#[api_operation(
    summary = "Get library stats",
    tag = "Library Ops",
    operation_id = "get_library_stats"
)]
pub async fn get_library_stats(
    data: Data<AppState>,
) -> Result<Json<LibraryStatsResponse>, APIError> {
    let res = library_service::get_library_stats(data).await?;
    Ok(Json(res))
}

pub fn config(cfg: &mut apistos_web::ServiceConfig) {
    cfg.service(
        apistos_web::scope("/library-categories")
            // .wrap(Authenticated)
            // .wrap(PermissionVerification { required_permission: PermissionEnum::LibraryManage })
            .route("", apistos_web::post().to(create_library_category))
            .route("/{id}", apistos_web::get().to(get_library_category_by_id))
            .route("", apistos_web::get().to(get_all_library_category))
            .route("/{id}", apistos_web::put().to(update_library_category))
            .route("/{id}", apistos_web::delete().to(delete_library_category))
            .route("/bulk", apistos_web::delete().to(bulk_delete_library_category))
            .route("/bulk", apistos_web::patch().to(bulk_update_library_category)),
    )
    .service(
        apistos_web::scope("/library-books")
            // .wrap(Authenticated)
            // .wrap(PermissionVerification { required_permission: PermissionEnum::LibraryManage })
            .route("", apistos_web::post().to(create_library_book))
            .route("/{id}", apistos_web::get().to(get_library_book_by_id))
            .route("", apistos_web::get().to(get_all_library_book))
            .route("/{id}", apistos_web::put().to(update_library_book))
            .route("/{id}", apistos_web::delete().to(delete_library_book))
            .route("/bulk", apistos_web::delete().to(bulk_delete_library_book))
            .route("/bulk", apistos_web::patch().to(bulk_update_library_book)),
    )
    .service(
        apistos_web::scope("/library-issues")
            // .wrap(Authenticated)
            // .wrap(PermissionVerification { required_permission: PermissionEnum::LibraryManage })
            .route("/{id}", apistos_web::get().to(get_library_issue_by_id))
            .route("", apistos_web::get().to(get_all_library_issue))
            .route("/{id}", apistos_web::put().to(update_library_issue))
            .route("/{id}", apistos_web::delete().to(delete_library_issue))
            .route("/bulk", apistos_web::delete().to(bulk_delete_library_issue))
            .route("/bulk", apistos_web::patch().to(bulk_update_library_issue)),
    )
    .service(
        apistos_web::scope("/library-settings")
            // .wrap(Authenticated)
            // .wrap(PermissionVerification { required_permission: PermissionEnum::LibraryManage })
            .route("", apistos_web::post().to(create_library_settings))
            .route("/{id}", apistos_web::get().to(get_library_settings_by_id))
            .route("", apistos_web::get().to(get_all_library_settings))
            .route("/{id}", apistos_web::put().to(update_library_settings))
            .route("/{id}", apistos_web::delete().to(delete_library_settings))
            .route("/bulk", apistos_web::delete().to(bulk_delete_library_settings))
            .route("/bulk", apistos_web::patch().to(bulk_update_library_settings)),
    )
    .service(
        apistos_web::scope("/library-ops")
            // .wrap(Authenticated)
            // .wrap(PermissionVerification { required_permission: PermissionEnum::LibraryManage })
            .route("/issue", apistos_web::post().to(issue_book))
            .route("/return/{id}", apistos_web::post().to(return_book))
            .route("/renew/{id}", apistos_web::post().to(renew_book))
            .route("/books/search", apistos_web::get().to(search_books))
            .route("/books/category/{category_id}", apistos_web::get().to(get_books_by_category))
            .route("/issues/student/{student_id}", apistos_web::get().to(get_issued_books_by_student))
            .route("/issues/staff/{staff_id}", apistos_web::get().to(get_issued_books_by_staff))
            .route("/issues/overdue", apistos_web::get().to(get_overdue_books))
            .route("/fines/pay/{id}", apistos_web::post().to(pay_fine))
            .route("/fines/waive/{id}", apistos_web::post().to(waive_fine))
            .route("/fines/history", apistos_web::get().to(get_fine_history))
            .route("/stats", apistos_web::get().to(get_library_stats)),
    );
}
