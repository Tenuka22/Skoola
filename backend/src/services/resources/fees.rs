use crate::database::tables::{FeeCategory, FeeStructure, StudentFee, FeePayment};
use crate::errors::APIError;
use crate::models::fees::{
    CreateFeeCategoryRequest, UpdateFeeCategoryRequest, CreateFeeStructureRequest, 
    AssignFeeToStudentRequest, RecordFeePaymentRequest, ApplyWaiverRequest, BulkAssignFeesRequest,
    FeeReceiptResponse, ExportReportResponse, UpdateFeeCategoryChangeset
};

use crate::schema::{fee_categories, fee_structures, student_fees, fee_payments, students, student_class_assignments, grade_levels};
use diesel::prelude::*;
use diesel::SqliteConnection;
use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};

pub async fn export_fee_reports(
    _conn: &mut SqliteConnection,
) -> Result<ExportReportResponse, APIError> {
    Ok(ExportReportResponse {
        csv_data: "placeholder_csv_data".to_string(),
        filename: "fee_report.csv".to_string(),
    })
}

pub async fn create_category(
    conn: &mut SqliteConnection,
    req: CreateFeeCategoryRequest,
) -> Result<FeeCategory, APIError> {
    let new_category = FeeCategory {
        id: Uuid::new_v4().to_string(),
        name: req.name,
        description: req.description,
        is_mandatory: req.is_mandatory,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(fee_categories::table)
        .values(&new_category)
        .execute(conn)
        .map_err(|e| APIError::internal(&format!("Failed to create fee category: {}", e)))?;

    Ok(new_category)
}

pub async fn get_all_categories(
    conn: &mut SqliteConnection,
) -> Result<Vec<FeeCategory>, APIError> {
    Ok(fee_categories::table
        .select(FeeCategory::as_select())
        .load(conn)?)
}

pub async fn get_all_categories_paginated(
    conn: &mut SqliteConnection,
    query: crate::handlers::resources::fees::FeeCategoryQuery,
) -> Result<(Vec<FeeCategory>, i64, i64), APIError> {
    use crate::schema::fee_categories::dsl::{fee_categories, name, is_mandatory};

    let mut data_query = fee_categories.into_boxed();
    let mut count_query = fee_categories.into_boxed();

    if let Some(search) = &query.search {
        data_query = data_query.filter(name.like(format!("%{}%", search)));
        count_query = count_query.filter(name.like(format!("%{}%", search)));
    }
    if let Some(mandatory) = query.is_mandatory {
        data_query = data_query.filter(is_mandatory.eq(mandatory));
        count_query = count_query.filter(is_mandatory.eq(mandatory));
    }

    let sort_by = query.sort_by.as_deref().unwrap_or("name");
    let sort_order = query.sort_order.as_deref().unwrap_or("asc");

    data_query = match (sort_by, sort_order) {
        ("name", "asc") => data_query.order(name.asc()),
        ("name", "desc") => data_query.order(name.desc()),
        _ => data_query.order(name.asc()),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let total_categories = count_query.count().get_result(conn)?;
    let total_pages = (total_categories as f64 / limit as f64).ceil() as i64;

    let categories_list = data_query
        .limit(limit)
        .offset(offset)
        .load::<FeeCategory>(conn)?;

    Ok((categories_list, total_categories, total_pages))
}

pub async fn bulk_delete_fee_categories(
    conn: &mut SqliteConnection,
    category_ids_to_delete: Vec<String>,
) -> Result<(), APIError> {
    use crate::schema::fee_categories::dsl::{fee_categories, id};

    diesel::delete(fee_categories.filter(id.eq_any(category_ids_to_delete)))
        .execute(conn)?;

    Ok(())
}

pub async fn bulk_update_fee_categories(
    conn: &mut SqliteConnection,
    req: crate::handlers::resources::fees::BulkUpdateFeeCategoriesRequest,
) -> Result<(), APIError> {
    use crate::schema::fee_categories::dsl::{fee_categories, id};

    let changeset = UpdateFeeCategoryChangeset {
        name: req.name,
        description: req.description,
        is_mandatory: req.is_mandatory,
    };

    diesel::update(fee_categories.filter(id.eq_any(req.category_ids)))
        .set(&changeset)
        .execute(conn)?;

    Ok(())
}

pub async fn update_category(
    conn: &mut SqliteConnection,
    category_id: &str,
    req: UpdateFeeCategoryRequest,
) -> Result<FeeCategory, APIError> {
    let mut target = fee_categories::table
        .filter(fee_categories::id.eq(category_id))
        .select(FeeCategory::as_select())
        .first(conn)
        .map_err(|e| APIError::internal(&format!("Failed to find fee category: {}", e)))?;

    if let Some(name) = req.name { target.name = name; }
    if let Some(desc) = req.description { target.description = Some(desc); }
    if let Some(mandatory) = req.is_mandatory { target.is_mandatory = mandatory; }
    target.updated_at = Utc::now().naive_utc();

    diesel::update(fee_categories::table.filter(fee_categories::id.eq(category_id)))
        .set((
            fee_categories::name.eq(&target.name),
            fee_categories::description.eq(&target.description),
            fee_categories::is_mandatory.eq(target.is_mandatory),
            fee_categories::updated_at.eq(target.updated_at),
        ))
        .execute(conn)
        .map_err(|e| APIError::internal(&format!("Failed to update fee category: {}", e)))?;

    Ok(target)
}

pub async fn create_structure(
    conn: &mut SqliteConnection,
    req: CreateFeeStructureRequest,
) -> Result<FeeStructure, APIError> {
    let new_structure = FeeStructure {
        id: Uuid::new_v4().to_string(),
        grade_id: req.grade_id,
        academic_year_id: req.academic_year_id,
        category_id: req.category_id,
        amount: req.amount,
        due_date: req.due_date,
        frequency: req.frequency,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(fee_structures::table)
        .values(&new_structure)
        .execute(conn)
        .map_err(|e| APIError::internal(&format!("Failed to create fee structure: {}", e)))?;

    Ok(new_structure)
}

pub async fn get_structures_by_grade(
    conn: &mut SqliteConnection,
    grade_id: &str,
) -> Result<Vec<FeeStructure>, APIError> {
    Ok(fee_structures::table
        .filter(fee_structures::grade_id.eq(grade_id))
        .select(FeeStructure::as_select())
        .load(conn)?)
}

pub async fn update_structure(
    conn: &mut SqliteConnection,
    structure_id: &str,
    req: crate::models::fees::UpdateFeeStructureRequest,
) -> Result<FeeStructure, APIError> {
    let mut target = fee_structures::table
        .filter(fee_structures::id.eq(structure_id))
        .select(FeeStructure::as_select())
        .first(conn)
        .map_err(|e| APIError::internal(&format!("Failed to find fee structure: {}", e)))?;

    if let Some(amount) = req.amount { target.amount = amount; }
    if let Some(due_date) = req.due_date { target.due_date = due_date; }
    if let Some(frequency) = req.frequency { target.frequency = frequency; }
    target.updated_at = Utc::now().naive_utc();

    diesel::update(fee_structures::table.filter(fee_structures::id.eq(structure_id)))
        .set((
            fee_structures::amount.eq(target.amount),
            fee_structures::due_date.eq(target.due_date),
            fee_structures::frequency.eq(target.frequency.clone()),
            fee_structures::updated_at.eq(target.updated_at),
        ))
        .execute(conn)
        .map_err(|e| APIError::internal(&format!("Failed to update fee structure: {}", e)))?;

    Ok(target)
}

pub async fn get_structures_by_academic_year(
    conn: &mut SqliteConnection,
    academic_year_id: &str,
) -> Result<Vec<FeeStructure>, APIError> {
    Ok(fee_structures::table
        .filter(fee_structures::academic_year_id.eq(academic_year_id))
        .select(FeeStructure::as_select())
        .load(conn)?)
}

pub async fn get_all_fee_structures_paginated(
    conn: &mut SqliteConnection,
    query: crate::handlers::resources::fees::FeeStructureQuery,
) -> Result<(Vec<FeeStructure>, i64, i64), APIError> {
    use crate::schema::fee_structures::dsl::{fee_structures, grade_id, academic_year_id, category_id, amount, due_date};

    let mut data_query = fee_structures.into_boxed();
    let mut count_query = fee_structures.into_boxed();

    if let Some(extracted_grade_id) = &query.grade_id {
        data_query = data_query.filter(grade_id.eq(extracted_grade_id));
        count_query = count_query.filter(grade_id.eq(extracted_grade_id));
    }
    if let Some(extracted_academic_year_id) = &query.academic_year_id {
        data_query = data_query.filter(academic_year_id.eq(extracted_academic_year_id));
        count_query = count_query.filter(academic_year_id.eq(extracted_academic_year_id));
    }
    if let Some(extracted_category_id) = &query.category_id {
        data_query = data_query.filter(category_id.eq(extracted_category_id));
        count_query = count_query.filter(category_id.eq(extracted_category_id));
    }

    let sort_by = query.sort_by.as_deref().unwrap_or("due_date");
    let sort_order = query.sort_order.as_deref().unwrap_or("asc");

    data_query = match (sort_by, sort_order) {
        ("amount", "asc") => data_query.order(amount.asc()),
        ("amount", "desc") => data_query.order(amount.desc()),
        ("due_date", "asc") => data_query.order(due_date.asc()),
        ("due_date", "desc") => data_query.order(due_date.desc()),
        _ => data_query.order(due_date.asc()),
    };

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let total_structures = count_query.count().get_result(conn)?;
    let total_pages = (total_structures as f64 / limit as f64).ceil() as i64;

    let structures_list = data_query
        .limit(limit)
        .offset(offset)
        .load::<FeeStructure>(conn)?;

    Ok((structures_list, total_structures, total_pages))
}

pub async fn bulk_delete_fee_structures(
    conn: &mut SqliteConnection,
    structure_ids_to_delete: Vec<String>,
) -> Result<(), APIError> {
    use crate::schema::fee_structures::dsl::{fee_structures, id};

    diesel::delete(fee_structures.filter(id.eq_any(structure_ids_to_delete)))
        .execute(conn)?;

    Ok(())
}

pub async fn bulk_update_fee_structures(
    conn: &mut SqliteConnection,
    req: crate::handlers::resources::fees::BulkUpdateFeeStructuresRequest,
) -> Result<(), APIError> {
    use crate::schema::fee_structures::dsl::{fee_structures, id};

    let changeset = crate::models::fees::UpdateFeeStructure {
        grade_id: req.grade_id,
        academic_year_id: req.academic_year_id,
        category_id: req.category_id,
        amount: req.amount,
        due_date: req.due_date,
        frequency: req.frequency.and_then(|f| f.parse().ok()), // Convert Option<String> to Option<FeeFrequency>
    };

    diesel::update(fee_structures.filter(id.eq_any(req.structure_ids)))
        .set(&changeset)
        .execute(conn)?;

    Ok(())
}

pub async fn assign_fee_to_student(
    conn: &mut SqliteConnection,
    req: AssignFeeToStudentRequest,
) -> Result<StudentFee, APIError> {
    let new_student_fee = StudentFee {
        id: Uuid::new_v4().to_string(),
        student_id: req.student_id,
        fee_structure_id: req.fee_structure_id,
        amount: req.amount,
        is_exempted: false,
        exemption_reason: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(student_fees::table)
        .values(&new_student_fee)
        .execute(conn)
        .map_err(|e| APIError::internal(&format!("Failed to assign fee to student: {}", e)))?;

    Ok(new_student_fee)
}

pub async fn record_payment(
    conn: &mut SqliteConnection,
    req: RecordFeePaymentRequest,
) -> Result<FeePayment, APIError> {
    let receipt_number = format!(
        "RCP-{}",
        Uuid::new_v4()
            .to_string()
            .split('-')
            .next()
            .unwrap_or_default()
            .to_uppercase()
    );
    
    let new_payment = FeePayment {
        id: Uuid::new_v4().to_string(),
        student_fee_id: req.student_fee_id,
        amount_paid: req.amount_paid,
        payment_date: req.payment_date.unwrap_or_else(|| Utc::now().naive_utc()),
        payment_method: req.payment_method,
        receipt_number,
        collected_by: req.collected_by,
        remarks: req.remarks,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(fee_payments::table)
        .values(&new_payment)
        .execute(conn)
        .map_err(|e| APIError::internal(&format!("Failed to record payment: {}", e)))?;

    Ok(new_payment)
}

pub async fn update_student_fee(
    conn: &mut SqliteConnection,
    fee_id: &str,
    req: crate::models::fees::ExemptFeeRequest,
) -> Result<StudentFee, APIError> {
    let mut target = student_fees::table
        .filter(student_fees::id.eq(fee_id))
        .select(StudentFee::as_select())
        .first(conn)?;

    target.is_exempted = req.is_exempted;
    target.exemption_reason = req.exemption_reason;
    target.updated_at = Utc::now().naive_utc();

    diesel::update(student_fees::table.filter(student_fees::id.eq(fee_id)))
        .set((
            student_fees::is_exempted.eq(target.is_exempted),
            student_fees::exemption_reason.eq(&target.exemption_reason),
            student_fees::updated_at.eq(target.updated_at),
        ))
        .execute(conn)
        .map_err(|e| APIError::internal(&format!("Failed to update student fee: {}", e)))?;

    Ok(target)
}

pub async fn get_fees_by_student(
    conn: &mut SqliteConnection,
    student_id: &str,
) -> Result<Vec<StudentFee>, APIError> {
    Ok(student_fees::table
        .filter(student_fees::student_id.eq(student_id))
        .select(StudentFee::as_select())
        .load(conn)?)
}

pub async fn get_exempted_students(
    conn: &mut SqliteConnection,
) -> Result<Vec<crate::models::fees::StudentFeeResponse>, APIError> {
    let fees = student_fees::table
        .filter(student_fees::is_exempted.eq(true))
        .select(StudentFee::as_select())
        .load(conn)
        .map_err(|e| APIError::internal(&format!("Failed to load exempted students: {}", e)))?;

    Ok(fees.into_iter().map(crate::models::fees::StudentFeeResponse::from).collect())
}

pub async fn get_student_balance(
    conn: &mut SqliteConnection,
    student_id: &str,
) -> Result<f32, APIError> {
    let fees = student_fees::table
        .filter(student_fees::student_id.eq(student_id))
        .select(StudentFee::as_select())
        .load(conn)?;

    let total_due: f32 = fees.iter().filter(|f| !f.is_exempted).map(|f| f.amount).sum();
    
    let fee_ids: Vec<String> = fees.into_iter().map(|f| f.id).collect();
    let payments = fee_payments::table
        .filter(fee_payments::student_fee_id.eq_any(fee_ids))
        .load::<FeePayment>(conn)
        .map_err(|e| APIError::internal(&format!("Failed to load payments for student balance: {}", e)))?;

    let total_paid: f32 = payments.iter().map(|p| p.amount_paid).sum();

    Ok(total_due - total_paid)
}

pub async fn get_defaulters(
    conn: &mut SqliteConnection,
) -> Result<Vec<crate::models::fees::FeeDefaulterResponse>, APIError> {
    let all_students = students::table
        .load::<crate::database::tables::Student>(conn)
        .map_err(|e| APIError::internal(&format!("Failed to load all students for defaulters: {}", e)))?;

    let mut defaulters = Vec::new();

    for student in all_students {
        let balance = get_student_balance(conn, &student.id).await?;
        if balance > 0.0 {
            defaulters.push(crate::models::fees::FeeDefaulterResponse {
                student_id: student.id,
                admission_number: student.admission_number,
                student_name: student.name_english,
                total_due: 0.0, // This would require more complex queries to populate correctly in one go
                total_paid: 0.0,
                balance,
            });
        }
    }

    Ok(defaulters)
}

pub async fn get_collection_report(
    conn: &mut SqliteConnection,
) -> Result<Vec<crate::models::fees::FeeCollectionReport>, APIError> {
    let categories = fee_categories::table
        .load::<FeeCategory>(conn)
        .map_err(|e| APIError::internal(&format!("Failed to load fee categories for report: {}", e)))?;

    let mut report = Vec::new();

    for category in categories {
        let structures = fee_structures::table
            .filter(fee_structures::category_id.eq(&category.id))
            .load::<FeeStructure>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to load fee structures for report: {}", e)))?;

        let structure_ids: Vec<String> = structures.into_iter().map(|s| s.id).collect();
        
        let fees = student_fees::table
            .filter(student_fees::fee_structure_id.eq_any(&structure_ids))
            .load::<StudentFee>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to load student fees for report: {}", e)))?;

        let total_expected: f32 = fees.iter().filter(|f| !f.is_exempted).map(|f| f.amount).sum();
        
        let fee_ids: Vec<String> = fees.into_iter().map(|f| f.id).collect();
        let payments = fee_payments::table
            .filter(fee_payments::student_fee_id.eq_any(fee_ids))
            .load::<FeePayment>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to load payments for report: {}", e)))?;

        let total_collected: f32 = payments.iter().map(|p| p.amount_paid).sum();
        
        let percentage = if total_expected > 0.0 { (total_collected / total_expected) * 100.0 } else { 0.0 };

        report.push(crate::models::fees::FeeCollectionReport {
            category_name: category.name,
            total_collected,
            total_expected,
            collection_percentage: percentage,
        });
    }

    Ok(report)
}

pub async fn get_payment_history_by_student(
    conn: &mut SqliteConnection,
    student_id: &str,
) -> Result<crate::models::fees::FeePaymentHistoryResponse, APIError> {
    let fees = student_fees::table
        .filter(student_fees::student_id.eq(student_id))
        .load::<StudentFee>(conn)
        .map_err(|e| APIError::internal(&format!("Failed to load student fees for history: {}", e)))?;

    let total_due: f32 = fees.iter().filter(|f| !f.is_exempted).map(|f| f.amount).sum();
    
    let fee_ids: Vec<String> = fees.into_iter().map(|f| f.id).collect();
    let payments = fee_payments::table
        .filter(fee_payments::student_fee_id.eq_any(&fee_ids))
        .load::<FeePayment>(conn)
        .map_err(|e| APIError::internal(&format!("Failed to load payments for history: {}", e)))?;

    let total_paid: f32 = payments.iter().map(|p| p.amount_paid).sum();
    let balance = total_due - total_paid;

    Ok(crate::models::fees::FeePaymentHistoryResponse {
        payments: payments.into_iter().map(crate::models::fees::FeePaymentResponse::from).collect(),
        total_paid,
        balance,
    })
}

pub async fn get_grade_collection_report(
    conn: &mut SqliteConnection,
) -> Result<Vec<crate::models::fees::GradeFeeCollectionReport>, APIError> {
    let all_grades = grade_levels::table
        .load::<crate::models::grade_level::GradeLevel>(conn)
        .map_err(|e| APIError::internal(&format!("Failed to load grade levels for report: {}", e)))?;

    let mut report = Vec::new();

    for grade in all_grades {
        let structures = fee_structures::table
            .filter(fee_structures::grade_id.eq(&grade.id))
            .load::<FeeStructure>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to load fee structures for grade report: {}", e)))?;

        let structure_ids: Vec<String> = structures.into_iter().map(|s| s.id).collect();
        
        let fees = student_fees::table
            .filter(student_fees::fee_structure_id.eq_any(&structure_ids))
            .load::<StudentFee>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to load student fees for grade report: {}", e)))?;

        let total_expected: f32 = fees.iter().filter(|f| !f.is_exempted).map(|f| f.amount).sum();
        
        let fee_ids: Vec<String> = fees.into_iter().map(|f| f.id).collect();
        let payments = fee_payments::table
            .filter(fee_payments::student_fee_id.eq_any(fee_ids))
            .load::<FeePayment>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to load payments for grade report: {}", e)))?;

        let total_collected: f32 = payments.iter().map(|p| p.amount_paid).sum();

        report.push(crate::models::fees::GradeFeeCollectionReport {
            grade_id: grade.id,
            grade_name: grade.grade_name,
            total_collected,
            total_expected,
        });
    }
    Ok(report)
}

pub async fn apply_waiver(
    conn: &mut SqliteConnection,
    fee_id: &str,
    req: ApplyWaiverRequest,
) -> Result<StudentFee, APIError> {
    let mut target = student_fees::table
        .filter(student_fees::id.eq(fee_id))
        .first::<StudentFee>(conn)
        .map_err(|e| APIError::internal(&format!("Failed to find student fee for waiver: {}", e)))?;

    target.amount -= req.discount_amount;
    if target.amount < 0.0 { target.amount = 0.0; }
    target.exemption_reason = Some(format!("Waiver applied: {}. Reason: {}", req.discount_amount, req.reason));
    target.updated_at = Utc::now().naive_utc();

    diesel::update(student_fees::table.filter(student_fees::id.eq(fee_id)))
        .set((
            student_fees::amount.eq(target.amount),
            student_fees::exemption_reason.eq(&target.exemption_reason),
            student_fees::updated_at.eq(target.updated_at),
        ))
        .execute(conn)
        .map_err(|e| APIError::internal(&format!("Failed to apply waiver to student fee: {}", e)))?;

    Ok(target)
}

pub async fn bulk_assign_fees(
    conn: &mut SqliteConnection,
    req: BulkAssignFeesRequest,
) -> Result<i32, APIError> {
    let student_ids = student_class_assignments::table
        .filter(student_class_assignments::grade_id.eq(&req.grade_id))
        .filter(student_class_assignments::academic_year_id.eq(&req.academic_year_id))
        .select(student_class_assignments::student_id)
        .load::<String>(conn)
        .map_err(|e| APIError::internal(&format!("Failed to load student IDs for bulk assignment: {}", e)))?;

    let structure = fee_structures::table
        .find(&req.fee_structure_id)
        .first::<FeeStructure>(conn)
        .map_err(|e| APIError::internal(&format!("Failed to find fee structure for bulk assignment: {}", e)))?;

    let mut count = 0;
    for sid in student_ids {
        let new_fee = StudentFee {
            id: Uuid::new_v4().to_string(),
            student_id: sid,
            fee_structure_id: req.fee_structure_id.clone(),
            amount: structure.amount,
            is_exempted: false,
            exemption_reason: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        diesel::insert_into(student_fees::table)
            .values(&new_fee)
            .execute(conn)
            .map_err(|e| APIError::internal(&format!("Failed to assign individual fee in bulk: {}", e)))
            .ok(); // Ignore duplicates if already assigned
        count += 1;
    }

    Ok(count)
}

pub async fn get_payments_by_date_range(
    conn: &mut SqliteConnection,
    start: NaiveDateTime,
    end: NaiveDateTime,
) -> Result<Vec<FeePayment>, APIError> {
    Ok(fee_payments::table
        .filter(fee_payments::payment_date.between(start, end))
        .select(FeePayment::as_select())
        .load(conn)?)
}

pub async fn get_receipt_data(
    conn: &mut SqliteConnection,
    payment_id: &str,
) -> Result<FeeReceiptResponse, APIError> {
    let payment = fee_payments::table.find(payment_id).first::<FeePayment>(conn)
        .map_err(|e| APIError::internal(&format!("Failed to find payment for receipt: {}", e)))?;
    let student_fee = student_fees::table.find(&payment.student_fee_id).first::<StudentFee>(conn)
        .map_err(|e| APIError::internal(&format!("Failed to find student fee for receipt: {}", e)))?;
    let student = students::table.find(&student_fee.student_id).first::<crate::database::tables::Student>(conn)
        .map_err(|e| APIError::internal(&format!("Failed to find student for receipt: {}", e)))?;
    
    let balance = get_student_balance(conn, &student.id).await?;

    Ok(FeeReceiptResponse {
        receipt_number: payment.receipt_number,
        student_name: student.name_english,
        admission_number: student.admission_number,
        amount_paid: payment.amount_paid,
        date: payment.payment_date,
        payment_method: payment.payment_method,
        collected_by: payment.collected_by,
        balance_remaining: balance,
    })
}
