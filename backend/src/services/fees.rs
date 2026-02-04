use crate::database::tables::{FeeCategory, FeeStructure, StudentFee, FeePayment};
use crate::errors::APIError;
use crate::models::fees::{CreateFeeCategoryRequest, UpdateFeeCategoryRequest, CreateFeeStructureRequest, AssignFeeToStudentRequest, RecordFeePaymentRequest};
use crate::schema::{fee_categories, fee_structures, student_fees, fee_payments, students};
use diesel::prelude::*;
use diesel::SqliteConnection;
use uuid::Uuid;
use chrono::Utc;

pub struct FeeService;

impl FeeService {
    pub fn create_category(
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

    pub fn get_all_categories(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<FeeCategory>, APIError> {
        fee_categories::table
            .load::<FeeCategory>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to fetch fee categories: {}", e)))
    }

    pub fn update_category(
        conn: &mut SqliteConnection,
        category_id: &str,
        req: UpdateFeeCategoryRequest,
    ) -> Result<FeeCategory, APIError> {
        let mut target = fee_categories::table
            .filter(fee_categories::id.eq(category_id))
            .first::<FeeCategory>(conn)
            .map_err(|_| APIError::bad_request("Fee category not found"))?;

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

    pub fn create_structure(
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

    pub fn get_structures_by_grade(
        conn: &mut SqliteConnection,
        grade_id: &str,
    ) -> Result<Vec<FeeStructure>, APIError> {
        fee_structures::table
            .filter(fee_structures::grade_id.eq(grade_id))
            .load::<FeeStructure>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to fetch fee structures: {}", e)))
    }

    pub fn update_structure(
        conn: &mut SqliteConnection,
        structure_id: &str,
        req: crate::models::fees::UpdateFeeStructureRequest,
    ) -> Result<FeeStructure, APIError> {
        let mut target = fee_structures::table
            .filter(fee_structures::id.eq(structure_id))
            .first::<FeeStructure>(conn)
            .map_err(|_| APIError::bad_request("Fee structure not found"))?;

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

    pub fn get_structures_by_academic_year(
        conn: &mut SqliteConnection,
        academic_year_id: &str,
    ) -> Result<Vec<FeeStructure>, APIError> {
        fee_structures::table
            .filter(fee_structures::academic_year_id.eq(academic_year_id))
            .load::<FeeStructure>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to fetch fee structures: {}", e)))
    }

    pub fn assign_fee_to_student(
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

    pub fn record_payment(
        conn: &mut SqliteConnection,
        req: RecordFeePaymentRequest,
    ) -> Result<FeePayment, APIError> {
        let receipt_number = format!("RCP-{}", Uuid::new_v4().to_string().split('-').next().unwrap().to_uppercase());
        
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
            .map_err(|e| APIError::internal(&format!("Failed to record fee payment: {}", e)))?;

        Ok(new_payment)
    }

    pub fn update_student_fee(
        conn: &mut SqliteConnection,
        fee_id: &str,
        req: crate::models::fees::ExemptFeeRequest,
    ) -> Result<StudentFee, APIError> {
        let mut target = student_fees::table
            .filter(student_fees::id.eq(fee_id))
            .first::<StudentFee>(conn)
            .map_err(|_| APIError::bad_request("Student fee record not found"))?;

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

    pub fn get_fees_by_student(
        conn: &mut SqliteConnection,
        student_id: &str,
    ) -> Result<Vec<StudentFee>, APIError> {
        student_fees::table
            .filter(student_fees::student_id.eq(student_id))
            .load::<StudentFee>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to fetch student fees: {}", e)))
    }

    pub fn get_exempted_students(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<crate::models::fees::StudentFeeResponse>, APIError> {
        let fees = student_fees::table
            .filter(student_fees::is_exempted.eq(true))
            .load::<StudentFee>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to fetch exempted fees: {}", e)))?;

        Ok(fees.into_iter().map(crate::models::fees::StudentFeeResponse::from).collect())
    }

    pub fn get_student_balance(
        conn: &mut SqliteConnection,
        student_id: &str,
    ) -> Result<f32, APIError> {
        let fees = student_fees::table
            .filter(student_fees::student_id.eq(student_id))
            .load::<StudentFee>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to fetch student fees: {}", e)))?;

        let total_due: f32 = fees.iter().filter(|f| !f.is_exempted).map(|f| f.amount).sum();
        
        let fee_ids: Vec<String> = fees.into_iter().map(|f| f.id).collect();
        let payments = fee_payments::table
            .filter(fee_payments::student_fee_id.eq_any(fee_ids))
            .load::<FeePayment>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to fetch fee payments: {}", e)))?;

        let total_paid: f32 = payments.iter().map(|p| p.amount_paid).sum();

        Ok(total_due - total_paid)
    }

    pub fn get_defaulters(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<crate::models::fees::FeeDefaulterResponse>, APIError> {
        let all_students = students::table
            .load::<crate::database::tables::Student>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to fetch students: {}", e)))?;

        let mut defaulters = Vec::new();

        for student in all_students {
            let balance = Self::get_student_balance(conn, &student.id)?;
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

    pub fn get_collection_report(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<crate::models::fees::FeeCollectionReport>, APIError> {
        let categories = fee_categories::table
            .load::<FeeCategory>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to fetch categories: {}", e)))?;

        let mut report = Vec::new();

        for category in categories {
            let structures = fee_structures::table
                .filter(fee_structures::category_id.eq(&category.id))
                .load::<FeeStructure>(conn)
                .map_err(|e| APIError::internal(&format!("Failed to fetch structures: {}", e)))?;

            let structure_ids: Vec<String> = structures.into_iter().map(|s| s.id).collect();
            
            let fees = student_fees::table
                .filter(student_fees::fee_structure_id.eq_any(&structure_ids))
                .load::<StudentFee>(conn)
                .map_err(|e| APIError::internal(&format!("Failed to fetch student fees: {}", e)))?;

            let total_expected: f32 = fees.iter().filter(|f| !f.is_exempted).map(|f| f.amount).sum();
            
            let fee_ids: Vec<String> = fees.into_iter().map(|f| f.id).collect();
            let payments = fee_payments::table
                .filter(fee_payments::student_fee_id.eq_any(fee_ids))
                .load::<FeePayment>(conn)
                .map_err(|e| APIError::internal(&format!("Failed to fetch fee payments: {}", e)))?;

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

    pub fn get_payment_history_by_student(
        conn: &mut SqliteConnection,
        student_id: &str,
    ) -> Result<crate::models::fees::FeePaymentHistoryResponse, APIError> {
        let fees = student_fees::table
            .filter(student_fees::student_id.eq(student_id))
            .load::<StudentFee>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to fetch student fees: {}", e)))?;

        let total_due: f32 = fees.iter().filter(|f| !f.is_exempted).map(|f| f.amount).sum();
        
        let fee_ids: Vec<String> = fees.into_iter().map(|f| f.id).collect();
        let payments = fee_payments::table
            .filter(fee_payments::student_fee_id.eq_any(&fee_ids))
            .load::<FeePayment>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to fetch fee payments: {}", e)))?;

        let total_paid: f32 = payments.iter().map(|p| p.amount_paid).sum();
        let balance = total_due - total_paid;

        Ok(crate::models::fees::FeePaymentHistoryResponse {
            payments: payments.into_iter().map(crate::models::fees::FeePaymentResponse::from).collect(),
            total_paid,
            balance,
        })
    }

    pub fn get_grade_collection_report(
        conn: &mut SqliteConnection,
    ) -> Result<Vec<crate::models::fees::GradeFeeCollectionReport>, APIError> {
        let all_grades = crate::schema::grade_levels::table
            .load::<crate::models::grade_level::GradeLevel>(conn)
            .map_err(|e| APIError::internal(&format!("Failed to fetch grade levels: {}", e)))?;

        let mut report = Vec::new();

        for grade in all_grades {
            let structures = fee_structures::table
                .filter(fee_structures::grade_id.eq(&grade.id))
                .load::<FeeStructure>(conn)
                .map_err(|e| APIError::internal(&format!("Failed to fetch structures: {}", e)))?;

            let structure_ids: Vec<String> = structures.into_iter().map(|s| s.id).collect();
            
            let fees = student_fees::table
                .filter(student_fees::fee_structure_id.eq_any(&structure_ids))
                .load::<StudentFee>(conn)
                .map_err(|e| APIError::internal(&format!("Failed to fetch student fees: {}", e)))?;

            let total_expected: f32 = fees.iter().filter(|f| !f.is_exempted).map(|f| f.amount).sum();
            
            let fee_ids: Vec<String> = fees.into_iter().map(|f| f.id).collect();
            let payments = fee_payments::table
                .filter(fee_payments::student_fee_id.eq_any(fee_ids))
                .load::<FeePayment>(conn)
                .map_err(|e| APIError::internal(&format!("Failed to fetch fee payments: {}", e)))?;

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
}
