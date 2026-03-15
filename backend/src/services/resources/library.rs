use chrono::{Local, Utc};
use diesel::SqliteConnection;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{QueryDsl, RunQueryDsl};

use crate::errors::APIError;
use crate::database::enums::LibraryIssueStatus;
use crate::models::resources::library::*;
use crate::models::staff::staff::Staff;
use crate::models::student::student::Student;
use crate::schema::{
    library_books, library_categories, library_issues, library_settings, staff, students,
};

use crate::{AppState, impl_admin_entity_service_i32};
use crate::services::admin_db::AdminQuery;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

impl_admin_entity_service_i32!(
    LibraryCategoryService,
    library_categories::table,
    LibraryCategory,
    LibraryCategory,
    library_categories::id,
    LibraryCategoryQuery,
    |q: library_categories::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(library_categories::category_name.like(search))
    },
    |q: library_categories::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(library_categories::category_name.asc())
    }
);

impl_admin_entity_service_i32!(
    LibraryBookService,
    library_books::table,
    LibraryBook,
    LibraryBook,
    library_books::id,
    LibraryBookQuery,
    |q: library_books::BoxedQuery<'static, diesel::sqlite::Sqlite>, search| {
        q.filter(library_books::title.like(search))
    },
    |q: library_books::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(library_books::title.asc())
    }
);

impl_admin_entity_service_i32!(
    LibraryIssueService,
    library_issues::table,
    LibraryIssue,
    LibraryIssue,
    library_issues::id,
    AdminQuery,
    |q: library_issues::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: library_issues::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(library_issues::issue_date.desc())
    }
);

impl_admin_entity_service_i32!(
    LibrarySettingsService,
    library_settings::table,
    LibrarySettings,
    LibrarySettingsResponse,
    library_settings::id,
    LibrarySettingsQuery,
    |q: library_settings::BoxedQuery<'static, diesel::sqlite::Sqlite>, _search| { q },
    |q: library_settings::BoxedQuery<'static, diesel::sqlite::Sqlite>, _sort_by, _sort_order| {
        q.order(library_settings::id.asc())
    }
);

impl LibrarySettingsService {
    pub async fn create_with_logic(
        data: actix_web::web::Data<AppState>,
        req: LibrarySettings,
    ) -> Result<LibrarySettingsResponse, APIError> {
        let now = Utc::now().naive_utc();
        let mut new_item = req;
        new_item.created_at = now;
        new_item.updated_at = now;
        Self::generic_create(data, new_item).await
    }
}

impl LibraryCategoryService {
    pub async fn create_with_logic(
        data: actix_web::web::Data<AppState>,
        req: CreateLibraryCategoryRequest,
    ) -> Result<LibraryCategory, APIError> {
        let now = Utc::now().naive_utc();
        let new_item = LibraryCategory {
            id: 0, // Auto-increment
            category_name: req.category_name,
            description: req.description,
            created_at: now,
            updated_at: now,
        };
        Self::generic_create(data, new_item).await
    }
}

impl LibraryBookService {
    pub async fn create_with_logic(
        data: actix_web::web::Data<AppState>,
        req: CreateLibraryBookRequest,
    ) -> Result<LibraryBook, APIError> {
        let now = Utc::now().naive_utc();
        let new_item = LibraryBook {
            id: 0, // Auto-increment
            isbn: req.isbn,
            title: req.title,
            author: req.author,
            publisher: req.publisher,
            category_id: req.category_id,
            quantity: req.quantity,
            available_quantity: req.quantity,
            rack_number: req.rack_number,
            added_date: Local::now().date_naive(),
            created_at: now,
            updated_at: now,
        };
        Self::generic_create(data, new_item).await
    }
}

impl LibraryIssueService {
    pub async fn create_with_logic(
        data: actix_web::web::Data<AppState>,
        req: IssueBookRequest,
        issued_by_id: String,
    ) -> Result<LibraryIssue, APIError> {
        let mut conn = data.db_pool.get()?;
        let now_dt = Utc::now().naive_utc();
        
        // Get library settings for due date calculation
        let settings = library_settings::table
            .select(LibrarySettings::as_select())
            .first(&mut conn)?;

        let issue_date = Local::now().date_naive();
        let duration_days = if req.student_id.is_some() {
            settings.issue_duration_days_student
        } else {
            settings.issue_duration_days_staff
        };
        let due_date = issue_date + chrono::Duration::days(duration_days as i64);

        let new_item = LibraryIssue {
            id: 0, // Auto-increment
            book_id: req.book_id,
            student_id: req.student_id,
            staff_id: req.staff_id,
            issue_date,
            due_date,
            return_date: None,
            issued_by: issued_by_id,
            fine_amount: None,
            fine_paid: false,
            status: LibraryIssueStatus::Issued,
            remarks: Some(req.remarks),
            created_at: now_dt,
            updated_at: now_dt,
        };
        Self::generic_create(data, new_item).await
    }
}

pub async fn issue_book(
    data: actix_web::web::Data<AppState>,
    req: IssueBookRequest,
    issued_by_id: String,
) -> Result<LibraryIssue, APIError> {
    LibraryIssueService::create_with_logic(data, req, issued_by_id).await
}

pub async fn get_issued_books_by_student(
    data: actix_web::web::Data<AppState>,
    student_id: String,
) -> Result<Vec<LibraryIssueResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

    let issues = library_issues::table
        .filter(library_issues::student_id.eq(&student_id))
        .order(library_issues::issue_date.desc())
        .select(LibraryIssue::as_select())
        .load::<LibraryIssue>(&mut conn)?;

    let mut res = Vec::new();
    for issue in issues {
        res.push(get_issue_by_id_internal(data.clone(), &mut conn, issue.id)?);
    }
    Ok(res)
}

pub async fn get_issued_books_by_staff(
    data: actix_web::web::Data<AppState>,
    staff_id: String,
) -> Result<Vec<LibraryIssueResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

    let issues = library_issues::table
        .filter(library_issues::staff_id.eq(&staff_id))
        .order(library_issues::issue_date.desc())
        .select(LibraryIssue::as_select())
        .load::<LibraryIssue>(&mut conn)?;

    let mut res = Vec::new();
    for issue in issues {
        res.push(get_issue_by_id_internal(data.clone(), &mut conn, issue.id)?);
    }
    Ok(res)
}

pub async fn get_overdue_books(data: actix_web::web::Data<AppState>) -> Result<Vec<LibraryIssueResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

    let today = Local::now().date_naive();

    let issues = library_issues::table
        .filter(library_issues::return_date.is_null())
        .filter(library_issues::due_date.lt(today))
        .order(library_issues::due_date.asc())
        .select(LibraryIssue::as_select())
        .load::<LibraryIssue>(&mut conn)?;

    let mut res = Vec::new();
    for issue in issues {
        res.push(get_issue_by_id_internal(data.clone(), &mut conn, issue.id)?);
    }
    Ok(res)
}

pub async fn pay_fine(
    data: actix_web::web::Data<AppState>,
    issue_id: i32,
) -> Result<LibraryIssueResponse, APIError> {
    let mut conn = data.db_pool.get()?;

    let issue = library_issues::table
        .find(issue_id)
        .select(LibraryIssue::as_select())
        .first::<LibraryIssue>(&mut conn)?;

    if issue.fine_amount.unwrap_or(0.0) == 0.0 {
        return Err(APIError::bad_request("No fine to pay"));
    }

    if issue.fine_paid {
        return Err(APIError::bad_request("Fine has already been paid"));
    }

    diesel::update(library_issues::table.find(issue_id))
        .set((
            library_issues::fine_paid.eq(true),
            library_issues::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    get_issue_by_id_internal(data, &mut conn, issue_id)
}

pub async fn waive_fine(data: actix_web::web::Data<AppState>, issue_id: i32) -> Result<LibraryIssueResponse, APIError> {
    let mut conn = data.db_pool.get()?;

    diesel::update(library_issues::table.find(issue_id))
        .set((
            library_issues::fine_amount.eq(Some(0.0)),
            library_issues::fine_paid.eq(true),
            library_issues::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    get_issue_by_id_internal(data, &mut conn, issue_id)
}

pub async fn get_library_settings(data: actix_web::web::Data<AppState>) -> Result<LibrarySettings, APIError> {
    let mut conn = data.db_pool.get()?;

    Ok(library_settings::table
        .select(LibrarySettings::as_select())
        .first(&mut conn)?)
}

pub async fn update_library_settings(
    data: actix_web::web::Data<AppState>,
    req: UpdateLibrarySettingsRequest,
) -> Result<LibrarySettings, APIError> {
    let mut conn = data.db_pool.get()?;

    let changeset = UpdateLibrarySettingsChangeset {
        max_books_per_student: req.max_books_per_student,
        max_books_per_staff: req.max_books_per_staff,
        issue_duration_days_student: req.issue_duration_days_student,
        issue_duration_days_staff: req.issue_duration_days_staff,
        fine_per_day: req.fine_per_day,
        updated_at: Utc::now().naive_utc(),
    };

    diesel::update(library_settings::table)
        .set(&changeset)
        .execute(&mut conn)?;

    get_library_settings(data).await
}

pub async fn get_library_stats(data: actix_web::web::Data<AppState>) -> Result<LibraryStatsResponse, APIError> {
    let mut conn = data.db_pool.get()?;

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

    let total_categories = library_categories::table.count().get_result(&mut conn)?;

    Ok(LibraryStatsResponse {
        total_books,
        total_available,
        total_issued,
        total_overdue,
        total_categories,
    })
}

pub async fn get_fine_history(data: actix_web::web::Data<AppState>) -> Result<Vec<LibraryIssueResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

    let issues = library_issues::table
        .filter(library_issues::fine_amount.is_not_null())
        .filter(library_issues::fine_amount.gt(0.0))
        .order(library_issues::return_date.desc())
        .select(LibraryIssue::as_select())
        .load::<LibraryIssue>(&mut conn)?;

    let mut res = Vec::new();
    for issue in issues {
        res.push(get_issue_by_id_internal(data.clone(), &mut conn, issue.id)?);
    }
    Ok(res)
}

pub async fn search_books(data: actix_web::web::Data<AppState>, query: LibraryBookQuery) -> Result<Vec<LibraryBookResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

    let mut q = library_books::table
        .inner_join(library_categories::table)
        .into_boxed();

    if let Some(search) = query.search {
        let search_pattern = format!("%{}%", search);
        q = q.filter(
            library_books::title
                .like(search_pattern.clone())
                .or(library_books::author.like(search_pattern.clone()))
                .or(library_books::isbn.like(search_pattern)),
        );
    }

    if let Some(cat_id) = query.category_id {
        q = q.filter(library_books::category_id.eq(cat_id));
    }

    let results = q.select((LibraryBook::as_select(), LibraryCategory::as_select()))
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

pub async fn get_books_by_category(
    data: actix_web::web::Data<AppState>,
    category_id: i32,
) -> Result<Vec<LibraryBookResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

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

pub async fn get_library_book_by_id(data: actix_web::web::Data<AppState>, id: i32) -> Result<LibraryBookResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    let (book, category) = library_books::table
        .inner_join(library_categories::table)
        .filter(library_books::id.eq(id))
        .select((LibraryBook::as_select(), LibraryCategory::as_select()))
        .first::<(LibraryBook, LibraryCategory)>(&mut conn)?;

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

pub async fn get_issue_by_id(data: actix_web::web::Data<AppState>, issue_id: i32) -> Result<LibraryIssueResponse, APIError> {
    let mut conn = data.db_pool.get()?;
    get_issue_by_id_internal(data, &mut conn, issue_id)
}

fn get_issue_by_id_internal(_data: actix_web::web::Data<AppState>, conn: &mut SqliteConnection, issue_id: i32) -> Result<LibraryIssueResponse, APIError> {
    let issue = library_issues::table
        .find(issue_id)
        .select(LibraryIssue::as_select())
        .first::<LibraryIssue>(conn)?;

    let book = library_books::table
        .find(issue.book_id)
        .select(LibraryBook::as_select())
        .first::<LibraryBook>(conn)?;

    let (student_name, staff_name) = if let Some(sid) = &issue.student_id {
        let student = students::table
            .find(sid)
            .select(Student::as_select())
            .first::<Student>(conn)
            .ok();
        (student.map(|s| s.name_english), None)
    } else if let Some(stid) = &issue.staff_id {
        let staff_member = staff::table
            .find(stid)
            .select(Staff::as_select())
            .first::<Staff>(conn)
            .ok();
        (None, staff_member.map(|s| s.name))
    } else {
        (None, None)
    };

    let issued_by_staff = staff::table
        .find(&issue.issued_by)
        .select(Staff::as_select())
        .first::<Staff>(conn)?;

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

pub async fn return_book(
    data: actix_web::web::Data<AppState>,
    issue_id: i32,
) -> Result<LibraryIssueResponse, APIError> {
    let mut conn = data.db_pool.get()?;

    // Get issue record
    let issue = library_issues::table
        .find(issue_id)
        .select(LibraryIssue::as_select())
        .first::<LibraryIssue>(&mut conn)?;

    if issue.return_date.is_some() {
        return Err(APIError::bad_request("Book has already been returned"));
    }

    let return_date = Local::now().date_naive();
    let mut fine_amount = 0.0;
    let mut status = LibraryIssueStatus::Returned;

    // Calculate fine if overdue
    if return_date > issue.due_date {
        let settings = library_settings::table
            .select(LibrarySettings::as_select())
            .first::<LibrarySettings>(&mut conn)?;

        let overdue_days = return_date.signed_duration_since(issue.due_date).num_days();
        fine_amount = overdue_days as f32 * settings.fine_per_day;
        status = LibraryIssueStatus::Overdue;
    }

    // Update issue record
    diesel::update(library_issues::table.find(issue_id))
        .set((
            library_issues::return_date.eq(Some(return_date)),
            library_issues::fine_amount.eq(Some(fine_amount)),
            library_issues::status.eq(status),
            library_issues::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    // Update available quantity
    diesel::update(library_books::table.find(issue.book_id))
        .set((
            library_books::available_quantity.eq(library_books::available_quantity + 1),
            library_books::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    get_issue_by_id_internal(data, &mut conn, issue_id)
}

pub async fn renew_book(
    data: actix_web::web::Data<AppState>,
    issue_id: i32,
) -> Result<LibraryIssueResponse, APIError> {
    let mut conn = data.db_pool.get()?;

    let issue = library_issues::table
        .find(issue_id)
        .select(LibraryIssue::as_select())
        .first::<LibraryIssue>(&mut conn)?;

    if issue.return_date.is_some() {
        return Err(APIError::bad_request("Returned books cannot be renewed"));
    }

    let settings = library_settings::table
        .select(LibrarySettings::as_select())
        .first::<LibrarySettings>(&mut conn)?;

    let duration_days = if issue.student_id.is_some() {
        settings.issue_duration_days_student
    } else {
        settings.issue_duration_days_staff
    };

    let new_due_date = issue.due_date + chrono::Duration::days(duration_days as i64);

    diesel::update(library_issues::table.find(issue_id))
        .set((
            library_issues::due_date.eq(new_due_date),
            library_issues::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    get_issue_by_id_internal(data, &mut conn, issue_id)
}

pub async fn check_overdue_books(data: actix_web::web::Data<AppState>) -> Result<(), APIError> {
    let mut conn = data.db_pool.get()?;
    let today = Local::now().date_naive();

    diesel::update(library_issues::table)
        .filter(library_issues::return_date.is_null())
        .filter(library_issues::due_date.lt(today))
        .filter(library_issues::status.eq(LibraryIssueStatus::Issued))
        .set((
            library_issues::status.eq(LibraryIssueStatus::Overdue),
            library_issues::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    Ok(())
}
