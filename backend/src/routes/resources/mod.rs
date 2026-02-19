use apistos::web;
use crate::handlers::resources::{co_curricular, fees, financial, library, property};
use crate::utils::jwt::Authenticated;
use crate::utils::permission_verification::PermissionVerification;
use crate::database::enums::PermissionEnum;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // Library Management Routes
    cfg.service(
        web::scope("/library")
            .wrap(PermissionVerification { required_permission: PermissionEnum::LibraryManage })
            .wrap(Authenticated)
            .route("/categories", web::get().to(library::get_all_categories))
            .route("/categories", web::post().to(library::create_category))
            .route("/categories/bulk", web::delete().to(library::bulk_delete_library_categories))
            .route("/categories/bulk", web::patch().to(library::bulk_update_library_categories))
            .route("/books", web::get().to(library::get_all_books))
            .route("/books/bulk", web::delete().to(library::bulk_delete_library_books))
            .route("/books/bulk", web::patch().to(library::bulk_update_library_books))
            .route("/books/search", web::get().to(library::search_books))
            .route("/books/{book_id}", web::get().to(library::get_book_by_id))
            .route("/books", web::post().to(library::create_book))
            .route("/books/{book_id}", web::put().to(library::update_book))
            .route("/books/{book_id}", web::delete().to(library::delete_book))
            .route("/books/category/{category_id}", web::get().to(library::get_books_by_category))
            .route("/issues", web::post().to(library::issue_book))
            .route("/issues/{issue_id}", web::get().to(library::get_issue_by_id))
            .route("/issues/{issue_id}/return", web::post().to(library::return_book))
            .route("/issues/student/{student_id}", web::get().to(library::get_issued_books_by_student))
            .route("/issues/staff/{staff_id}", web::get().to(library::get_issued_books_by_staff))
            .route("/issues/overdue", web::get().to(library::get_overdue_books))
            .route("/fines/{issue_id}/pay", web::post().to(library::pay_fine))
            .route("/fines/{issue_id}/waive", web::post().to(library::waive_fine))
            .route("/fines/history", web::get().to(library::get_fine_history))
            .route("/settings", web::get().to(library::get_library_settings))
            .route("/settings", web::put().to(library::update_library_settings))
            .route("/stats", web::get().to(library::get_library_stats)),
    );

    cfg.configure(property::config);
    cfg.configure(financial::config);
    cfg.configure(fees::config);
    cfg.configure(co_curricular::config);
}
