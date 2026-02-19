use chrono::Local;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::SqliteConnection;

use crate::models::student::student::Student;
use crate::models::staff::staff::Staff;
use crate::models::resources::library::LibraryCategory;
use crate::errors::APIError;
use crate::models::resources::library::*;
use crate::schema::{library_books, library_categories, library_issues, library_settings, staff, students};
use crate::handlers::resources::library::{LibraryCategoryQuery, BulkUpdateLibraryCategoriesRequest, LibraryBookQuery, BulkUpdateLibraryBooksRequest};

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

// ============= Category Services =============
pub async fn get_all_categories_paginated(
    pool: &DbPool,
    query: LibraryCategoryQuery,
) -> Result<(Vec<LibraryCategory>, i64, i64), APIError> {
    let mut conn = pool.get()?;
    let mut data_query = library_categories::table.into_boxed();
    let mut count_query = library_categories::table.into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        data_query = data_query.filter(library_categories::category_name.like(pattern.clone()).or(library_categories::description.like(pattern.clone())));
        count_query = count_query.filter(library_categories::category_name.like(pattern.clone()).or(library_categories::description.like(pattern.clone())));
    }

    let sort_by = query.sort_by.as_deref().unwrap_or("category_name");
    let sort_order = query.sort_order.as_deref().unwrap_or("asc");

    data_query = match (sort_by, sort_order) {
        ("category_name", "asc") => data_query.order(library_categories::category_name.asc()),
        ("category_name", "desc") => data_query.order(library_categories::category_name.desc()),
        _ => data_query.order(library_categories::category_name.asc()),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let total_categories = count_query.count().get_result(&mut conn)?;
    let total_pages = (total_categories as f64 / limit as f64).ceil() as i64;

    let categories_list: Vec<LibraryCategory> = data_query
        .limit(limit)
        .offset(offset)
        .load::<LibraryCategory>(&mut conn)?;

    Ok((categories_list, total_categories, total_pages))
}

pub fn create_category(pool: &DbPool, req: CreateLibraryCategoryRequest) -> Result<LibraryCategory, APIError> {
    let mut conn = pool.get()?;

    let new_category = NewLibraryCategory {
        category_name: req.category_name,
        description: req.description,
    };

    diesel::insert_into(library_categories::table)
        .values(&new_category)
        .execute(&mut conn)?;

    Ok(library_categories::table
        .order(library_categories::id.desc())
        .select(LibraryCategory::as_select())
        .first(&mut conn)?)
}

pub async fn bulk_delete_library_categories(
    pool: &DbPool,
    category_ids: Vec<i32>,
) -> Result<(), APIError> {
    let mut conn = pool.get()?;
    diesel::delete(library_categories::table.filter(library_categories::id.eq_any(category_ids)))
        .execute(&mut conn)?;
    Ok(())
}

pub async fn bulk_update_library_categories(
    pool: &DbPool,
    body: BulkUpdateLibraryCategoriesRequest,
) -> Result<(), APIError> {
    let mut conn = pool.get()?;

    conn.transaction::<_, APIError, _>(|conn| {
        let target = library_categories::table.filter(library_categories::id.eq_any(&body.category_ids));
        
        diesel::update(target)
            .set((
                body.category_name.map(|cn| library_categories::category_name.eq(cn)),
                body.description.map(|d| library_categories::description.eq(d)),
            ))
            .execute(conn)?;
        
        Ok(())
    })
}


// ============= Book Services =============

pub async fn get_all_books_paginated(
    pool: &DbPool,
    query: LibraryBookQuery,
) -> Result<(Vec<LibraryBookResponse>, i64, i64), APIError> {
    let mut conn = pool.get()?;
    let mut data_query = library_books::table
        .inner_join(library_categories::table)
        .into_boxed();
    let mut count_query = library_books::table
        .inner_join(library_categories::table)
        .into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        data_query = data_query.filter(
            library_books::title
                .like(pattern.clone())
                .or(library_books::author.like(pattern.clone()))
                .or(library_books::isbn.like(pattern.clone()))
        );
        count_query = count_query.filter(
            library_books::title
                .like(pattern.clone())
                .or(library_books::author.like(pattern.clone()))
                .or(library_books::isbn.like(pattern.clone()))
        );
    }

    if let Some(category_id) = query.category_id {
        data_query = data_query.filter(library_books::category_id.eq(category_id));
        count_query = count_query.filter(library_books::category_id.eq(category_id));
    }

    let sort_by = query.sort_by.as_deref().unwrap_or("title");
    let sort_order = query.sort_order.as_deref().unwrap_or("asc");

    data_query = match (sort_by, sort_order) {
        ("title", "asc") => data_query.order(library_books::title.asc()),
        ("title", "desc") => data_query.order(library_books::title.desc()),
        ("author", "asc") => data_query.order(library_books::author.asc()),
        ("author", "desc") => data_query.order(library_books::author.desc()),
        _ => data_query.order(library_books::title.asc()),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let total_books = count_query.count().get_result(&mut conn)?;
    let total_pages = (total_books as f64 / limit as f64).ceil() as i64;

    let results = data_query
        .select((LibraryBook::as_select(), LibraryCategory::as_select()))
        .limit(limit)
        .offset(offset)
        .load::<(LibraryBook, LibraryCategory)>(&mut conn)?;

    Ok((results
        .into_iter()
        .map(|(book, category)| LibraryBookResponse {
            id: book.id,
            isbn: book.isbn,
            title: book.title,
            author: book.author,
            publisher: book.publisher,
            category_id: book.category_id,
            category_name: category.category_name,
            quantity: book.quantity,
            available_quantity: book.available_quantity,
            rack_number: book.rack_number,
            added_date: book.added_date,
        })
        .collect(), total_books, total_pages))
}


pub fn get_book_by_id(pool: &DbPool, book_id: i32) -> Result<LibraryBookResponse, APIError> {
    let mut conn = pool.get()?;

    let (book, category) = library_books::table
        .inner_join(library_categories::table)
        .filter(library_books::id.eq(book_id))
        .select((LibraryBook::as_select(), LibraryCategory::as_select()))
        .first::<(LibraryBook, LibraryCategory)>(&mut conn)
        ?;

    Ok(LibraryBookResponse {
        id: book.id,
        isbn: book.isbn,
        title: book.title,
        author: book.author,
        publisher: book.publisher,
        category_id: book.category_id,
        category_name: category.category_name,
        quantity: book.quantity,
        available_quantity: book.available_quantity,
        rack_number: book.rack_number,
        added_date: book.added_date,
    })
}

pub fn search_books(pool: &DbPool, query: &str) -> Result<Vec<LibraryBookResponse>, APIError> {
    let mut conn = pool.get()?;

    let search_pattern = format!("%{}%", query);

    let results = library_books::table
        .inner_join(library_categories::table)
        .filter(
            library_books::title
                .like(&search_pattern)
                .or(library_books::author.like(&search_pattern))
                .or(library_books::isbn.like(&search_pattern)),
        )
        .select((LibraryBook::as_select(), LibraryCategory::as_select()))
        .load::<(LibraryBook, LibraryCategory)>(&mut conn)?;

    Ok(results
        .into_iter()
        .map(|(book, category)| LibraryBookResponse {
            id: book.id,
            isbn: book.isbn,
            title: book.title,
            author: book.author,
            publisher: book.publisher,
            category_id: book.category_id,
            category_name: category.category_name,
            quantity: book.quantity,
            available_quantity: book.available_quantity,
            rack_number: book.rack_number,
            added_date: book.added_date,
        })
        .collect())
}

pub fn get_books_by_category(pool: &DbPool, category_id: i32) -> Result<Vec<LibraryBookResponse>, APIError> {
    let mut conn = pool.get()?;

    let results = library_books::table
        .inner_join(library_categories::table)
        .filter(library_books::category_id.eq(category_id))
        .select((LibraryBook::as_select(), LibraryCategory::as_select()))
        .load::<(LibraryBook, LibraryCategory)>(&mut conn)?;

    Ok(results
        .into_iter()
        .map(|(book, category)| LibraryBookResponse {
            id: book.id,
            isbn: book.isbn,
            title: book.title,
            author: book.author,
            publisher: book.publisher,
            category_id: book.category_id,
            category_name: category.category_name,
            quantity: book.quantity,
            available_quantity: book.available_quantity,
            rack_number: book.rack_number,
            added_date: book.added_date,
        })
        .collect())
}

pub fn create_book(pool: &DbPool, req: CreateLibraryBookRequest) -> Result<LibraryBookResponse, APIError> {
    let mut conn = pool.get()?;

    // Verify category exists
    library_categories::table
        .find(req.category_id)
        .select(LibraryCategory::as_select())
        .first(&mut conn)
        ?;

    let new_book = NewLibraryBook {
        isbn: req.isbn,
        title: req.title,
        author: req.author,
        publisher: req.publisher,
        category_id: req.category_id,
        quantity: req.quantity,
        available_quantity: req.quantity,
        rack_number: req.rack_number,
        added_date: Local::now().date_naive(),
    };

    diesel::insert_into(library_books::table)
        .values(&new_book)
        .execute(&mut conn)?;

    let book: LibraryBook = library_books::table
        .order(library_books::id.desc())
        .select(LibraryBook::as_select())
        .first(&mut conn)?;

    get_book_by_id(pool, book.id)
}

pub fn update_book(pool: &DbPool, book_id: i32, req: UpdateLibraryBookRequest) -> Result<LibraryBookResponse, APIError> {
    let mut conn = pool.get()?;

    // Verify book exists
    library_books::table
        .find(book_id)
        .select(LibraryBook::as_select())
        .first(&mut conn)
        ?;

    // Verify category if provided
    if let Some(cat_id) = req.category_id {
        library_categories::table
            .find(cat_id)
            .select(LibraryCategory::as_select())
            .first(&mut conn)
            ?;
    }

    let changeset = UpdateLibraryBook {
        isbn: req.isbn,
        title: req.title,
        author: req.author,
        publisher: req.publisher,
        category_id: req.category_id,
        quantity: req.quantity,
        available_quantity: req.available_quantity,
        rack_number: req.rack_number,
        updated_at: Local::now().naive_local(),
    };

    diesel::update(library_books::table.find(book_id))
        .set(&changeset)
        .execute(&mut conn)?;

    get_book_by_id(pool, book_id)
}

pub fn delete_book(pool: &DbPool, book_id: i32) -> Result<(), APIError> {
    let mut conn = pool.get()?;

    // Check if book has active issues
    let active_issues = library_issues::table
        .filter(library_issues::book_id.eq(book_id))
        .filter(library_issues::return_date.is_null())
        .count()
        .get_result::<i64>(&mut conn)?;

    if active_issues > 0 {
        return Err(APIError::bad_request("Cannot delete book with active issues"));
    }

    diesel::delete(library_books::table.find(book_id))
        .execute(&mut conn)?;

    Ok(())
}

pub async fn bulk_delete_library_books(
    pool: &DbPool,
    book_ids: Vec<i32>,
) -> Result<(), APIError> {
    let mut conn = pool.get()?;
    diesel::delete(library_books::table.filter(library_books::id.eq_any(book_ids)))
        .execute(&mut conn)?;
    Ok(())
}

pub async fn bulk_update_library_books(
    pool: &DbPool,
    body: BulkUpdateLibraryBooksRequest,
) -> Result<(), APIError> {
    let mut conn = pool.get()?;

    conn.transaction::<_, APIError, _>(|conn| {
        let target = library_books::table.filter(library_books::id.eq_any(&body.book_ids));
        
        diesel::update(target)
            .set((
                body.isbn.map(|isbn| library_books::isbn.eq(isbn)),
                body.title.map(|title| library_books::title.eq(title)),
                body.author.map(|author| library_books::author.eq(author)),
                body.publisher.map(|publisher| library_books::publisher.eq(publisher)),
                body.category_id.map(|category_id| library_books::category_id.eq(category_id)),
                body.quantity.map(|quantity| library_books::quantity.eq(quantity)),
                body.available_quantity.map(|available_quantity| library_books::available_quantity.eq(available_quantity)),
                body.rack_number.map(|rack_number| library_books::rack_number.eq(rack_number)),
                library_books::updated_at.eq(Local::now().naive_local()),
            ))
            .execute(conn)?;
        
        Ok(())
    })
}

// ============= Issue/Return Services =============

pub fn issue_book(pool: &DbPool, req: IssueBookRequest, issued_by_id: String) -> Result<LibraryIssueResponse, APIError> {
    let mut conn = pool.get()?;

    // Validate that exactly one of student_id or staff_id is provided
    if (req.student_id.is_some() && req.staff_id.is_some()) || (req.student_id.is_none() && req.staff_id.is_none()) {
        return Err(APIError::bad_request("Exactly one of student_id or staff_id must be provided"));
    }

    // Get book and check availability
    let mut book = library_books::table
        .find(req.book_id)
        .select(LibraryBook::as_select())
        .first(&mut conn)
        ?;

    if book.available_quantity <= 0 {
        return Err(APIError::bad_request("Book is not available"));
    }

    // Get library settings
    let settings = library_settings::table
        .select(LibrarySettings::as_select())
        .first(&mut conn)?;

    // Check borrowing limits
    if let Some(student_id) = &req.student_id {
        let current_issues = library_issues::table
            .filter(library_issues::student_id.eq(student_id))
            .filter(library_issues::return_date.is_null())
            .count()
            .get_result::<i64>(&mut conn)?;

        if current_issues >= settings.max_books_per_student as i64 {
            return Err(APIError::bad_request("Student has reached maximum book limit"));
        }
    }

    if let Some(staff_id) = &req.staff_id {
        let current_issues = library_issues::table
            .filter(library_issues::staff_id.eq(staff_id))
            .filter(library_issues::return_date.is_null())
            .count()
            .get_result::<i64>(&mut conn)?;

        if current_issues >= settings.max_books_per_staff as i64 {
            return Err(APIError::bad_request("Staff has reached maximum book limit"));
        }
    }

    // Calculate due date
    let issue_date = Local::now().date_naive();
    let duration_days = if req.student_id.is_some() {
        settings.issue_duration_days_student
    } else {
        settings.issue_duration_days_staff
    };
    let due_date = issue_date + chrono::Duration::days(duration_days as i64);

    // Create issue record
    let new_issue = NewLibraryIssue {
        book_id: req.book_id,
        student_id: req.student_id,
        staff_id: req.staff_id,
        issue_date,
        due_date,
        issued_by: issued_by_id,
        status: "issued".to_string(),
        remarks: req.remarks,
    };

    diesel::insert_into(library_issues::table)
        .values(&new_issue)
        .execute(&mut conn)?;

    // Update available quantity
    book.available_quantity -= 1;
    book.updated_at = Local::now().naive_local(); // Update timestamp
    diesel::update(library_books::table.find(book.id))
        .set(&book) // Update using the modified book struct
        .execute(&mut conn)?;

    let issue: LibraryIssue = library_issues::table
        .order(library_issues::id.desc())
        .select(LibraryIssue::as_select())
        .first(&mut conn)?;

    get_issue_by_id(pool, issue.id)
}

pub fn return_book(pool: &DbPool, issue_id: i32, req: ReturnBookRequest) -> Result<LibraryIssueResponse, APIError> {
    let mut conn = pool.get()?;

    // Get issue record
    let issue = library_issues::table
        .find(issue_id)
        .select(LibraryIssue::as_select())
        .first(&mut conn)
        ?;

    if issue.return_date.is_some() {
        return Err(APIError::bad_request("Book has already been returned"));
    }

    let return_date = Local::now().date_naive();
    let mut fine_amount = 0.0;
    let mut status = "returned".to_string();

    // Calculate fine if overdue
    if return_date > issue.due_date {
        let settings = library_settings::table
            .select(LibrarySettings::as_select())
            .first(&mut conn)?;

        let overdue_days = (return_date - issue.due_date).num_days();
        fine_amount = overdue_days as f32 * settings.fine_per_day;
        status = "returned_with_fine".to_string();
    }

    // Update issue record
    diesel::update(library_issues::table.find(issue_id))
        .set((
            library_issues::return_date.eq(Some(return_date)),
            library_issues::fine_amount.eq(Some(fine_amount)),
            library_issues::status.eq(&status),
            library_issues::remarks.eq(req.remarks),
            library_issues::updated_at.eq(Local::now().naive_local()),
        ))
        .execute(&mut conn)?;

    // Update available quantity
    let mut book = library_books::table
        .find(issue.book_id)
        .select(LibraryBook::as_select())
        .first(&mut conn)?;

    book.available_quantity += 1;
    book.updated_at = Local::now().naive_local(); // Update timestamp
    diesel::update(library_books::table.find(book.id))
        .set(&book) // Update using the modified book struct
        .execute(&mut conn)?;

    get_issue_by_id(pool, issue_id)
}

pub fn get_issue_by_id(pool: &DbPool, issue_id: i32) -> Result<LibraryIssueResponse, APIError> {
    let mut conn = pool.get()?;

    let issue = library_issues::table
        .find(issue_id)
        .select(LibraryIssue::as_select())
        .first(&mut conn)
        ?;

    let book = library_books::table
        .find(issue.book_id)
        .select(LibraryBook::as_select())
        .first(&mut conn)?;

    let issued_by_staff = staff::table
        .find(&issue.issued_by)
        .select(Staff::as_select())
        .first(&mut conn)?;

    let (student_name, staff_name) = if let Some(sid) = &issue.student_id {
        let student = students::table
            .find(sid)
            .select(Student::as_select())
            .first(&mut conn)
            .ok();
        (student.map(|s| s.name_english), None)
    } else if let Some(stid) = &issue.staff_id {
        let staff_member = staff::table
            .find(stid)
            .select(Staff::as_select())
            .first(&mut conn)
            .ok();
        (None, staff_member.map(|s| s.name))
    } else {
        (None, None)
    };

    Ok(LibraryIssueResponse {
        id: issue.id,
        book_id: issue.book_id,
        book_title: book.title,
        student_id: issue.student_id,
        student_name,
        staff_id: issue.staff_id,
        staff_name,
        issue_date: issue.issue_date,
        due_date: issue.due_date,
        return_date: issue.return_date,
        issued_by: issue.issued_by,
        issued_by_name: issued_by_staff.name,
        fine_amount: issue.fine_amount,
        fine_paid: issue.fine_paid,
        status: issue.status,
        remarks: issue.remarks,
    })
}

pub fn get_issued_books_by_student(pool: &DbPool, student_id: String) -> Result<Vec<LibraryIssueResponse>, APIError> {
    let mut conn = pool.get()?;

    let issues = library_issues::table
        .filter(library_issues::student_id.eq(&student_id))
        .order(library_issues::issue_date.desc())
        .select(LibraryIssue::as_select())
        .load(&mut conn)?;

    issues.into_iter().map(|issue| get_issue_by_id(pool, issue.id)).collect::<Result<Vec<_>, _>>()
}

pub fn get_issued_books_by_staff(pool: &DbPool, staff_id: String) -> Result<Vec<LibraryIssueResponse>, APIError> {
    let mut conn = pool.get()?;

    let issues = library_issues::table
        .filter(library_issues::staff_id.eq(&staff_id))
        .order(library_issues::issue_date.desc())
        .load::<LibraryIssue>(&mut conn)?;

    issues.into_iter().map(|issue| get_issue_by_id(pool, issue.id)).collect::<Result<Vec<_>, _>>()
}

pub fn get_overdue_books(pool: &DbPool) -> Result<Vec<LibraryIssueResponse>, APIError> {
    let mut conn = pool.get()?;

    let today = Local::now().date_naive();

    let issues = library_issues::table
        .filter(library_issues::return_date.is_null())
        .filter(library_issues::due_date.lt(today))
        .order(library_issues::due_date.asc())
        .load::<LibraryIssue>(&mut conn)?;

    issues.into_iter().map(|issue| get_issue_by_id(pool, issue.id)).collect::<Result<Vec<_>, _>>()
}

pub fn pay_fine(pool: &DbPool, issue_id: i32, _req: PayFineRequest) -> Result<LibraryIssueResponse, APIError> {
    let mut conn = pool.get()?;

    let issue = library_issues::table
        .find(issue_id)
        .first::<LibraryIssue>(&mut conn)
        ?;

    if issue.fine_amount.unwrap_or(0.0) == 0.0 {
        return Err(APIError::bad_request("No fine to pay"));
    }

    if issue.fine_paid {
        return Err(APIError::bad_request("Fine has already been paid"));
    }

    diesel::update(library_issues::table.find(issue_id))
        .set((
            library_issues::fine_paid.eq(true),
            library_issues::updated_at.eq(Local::now().naive_local()),
        ))
        .execute(&mut conn)?;

    get_issue_by_id(pool, issue_id)
}

pub fn waive_fine(pool: &DbPool, issue_id: i32) -> Result<LibraryIssueResponse, APIError> {
    let mut conn = pool.get()?;

    diesel::update(library_issues::table.find(issue_id))
        .set((
            library_issues::fine_amount.eq(Some(0.0)),
            library_issues::fine_paid.eq(true),
            library_issues::updated_at.eq(Local::now().naive_local()),
        ))
        .execute(&mut conn)?;

    get_issue_by_id(pool, issue_id)
}

// ============= Settings Services =============

pub fn get_library_settings(pool: &DbPool) -> Result<LibrarySettings, APIError> {
    let mut conn = pool.get()?;

    Ok(library_settings::table
        .select(LibrarySettings::as_select())
        .first(&mut conn)?)
}

pub fn update_library_settings(pool: &DbPool, req: UpdateLibrarySettingsRequest) -> Result<LibrarySettings, APIError> {
    let mut conn = pool.get()?;

    let changeset = UpdateLibrarySettingsChangeset {
        max_books_per_student: req.max_books_per_student,
        max_books_per_staff: req.max_books_per_staff,
        issue_duration_days_student: req.issue_duration_days_student,
        issue_duration_days_staff: req.issue_duration_days_staff,
        fine_per_day: req.fine_per_day,
        updated_at: Local::now().naive_local(),
    };

    diesel::update(library_settings::table)
        .set(&changeset)
        .execute(&mut conn)?;

    get_library_settings(pool)
}

// ============= Statistics Services =============

pub fn get_library_stats(pool: &DbPool) -> Result<LibraryStatsResponse, APIError> {
    let mut conn = pool.get()?;

    let total_books = library_books::table
        .select(diesel::dsl::sum(library_books::quantity))
        .first::<Option<i64>>(&mut conn)?
        .unwrap_or(0);

    let total_available = library_books::table
        .select(diesel::dsl::sum(library_books::available_quantity))
        .first::<Option<i64>>(&mut conn)?
        .unwrap_or(0);

    let total_issued = library_issues::table
        .filter(library_issues::return_date.is_null())
        .count()
        .get_result(&mut conn)?;

    let today = Local::now().date_naive();
    let total_overdue = library_issues::table
        .filter(library_issues::return_date.is_null())
        .filter(library_issues::due_date.lt(today))
        .count()
        .get_result(&mut conn)?;

    let total_categories = library_categories::table
        .count()
        .get_result(&mut conn)?;

    Ok(LibraryStatsResponse {
        total_books,
        total_available,
        total_issued,
        total_overdue,
        total_categories,
    })
}

pub fn get_fine_history(pool: &DbPool) -> Result<Vec<LibraryIssueResponse>, APIError> {
    let mut conn = pool.get()?;

    let issues = library_issues::table
        .filter(library_issues::fine_amount.is_not_null())
        .filter(library_issues::fine_amount.gt(0.0))
        .order(library_issues::return_date.desc())
        .load::<LibraryIssue>(&mut conn)?;

    issues.into_iter().map(|issue| get_issue_by_id(pool, issue.id)).collect::<Result<Vec<_>, _>>()

}
