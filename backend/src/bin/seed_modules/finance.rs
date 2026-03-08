use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::{
    FeeFrequency, FeeAmountType, FeeTypeEnum, AccountTypeEnum, NormalBalanceType, PaymentMethod, PaymentStatusType,
    TransactionType
};
use backend::models::finance::account::ChartOfAccount;
use backend::models::finance::budget_category::BudgetCategory;
use backend::models::finance::budget::Budget;
use backend::models::finance::fees::{FeeCategory, FeeStructure, FeeStructurePricing, FeeStructureSchedule, StudentFee, FeePayment, FeePaymentDetail};
use backend::models::ids::IdPrefix;
use backend::schema::*;
use chrono::Utc;
use diesel::insert_into;
use diesel::prelude::*;
use std::collections::HashSet;

pub struct FinanceSeeder;

impl FinanceSeeder {
    pub fn new() -> Self {
        Self
    }
}

impl SeedModule for FinanceSeeder {
    fn seed(
        &self,
        conn: &mut SqliteConnection,
        _config: &Config,
        _password_hash: &str,
        _used_emails: &mut HashSet<String>,
        context: &mut SeederContext,
        _seed_count_config: &crate::SeedCountConfig,
    ) -> Result<()> {
        println!("Seeding Finance module...");

        // 1. chart_of_accounts
        println!("Seeding chart_of_accounts...");
        let mut coa_list = Vec::new();
        for i in 0..200 {
            let id = next_id(conn, IdPrefix::ACCOUNT);
            coa_list.push(ChartOfAccount {
                id: id.clone(),
                account_code: format!("ACC-{:04}", i),
                account_name: format!("Account {}", i),
                account_type: if i % 2 == 0 { AccountTypeEnum::Asset } else { AccountTypeEnum::Expense },
                normal_balance: if i % 2 == 0 { NormalBalanceType::Debit } else { NormalBalanceType::Credit },
                description: None,
                parent_account_id: None,
                is_active: true,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                currency: "LKR".to_string(),
            });
            context.chart_of_account_ids.push(id);
        }
        insert_into(chart_of_accounts::table).values(&coa_list).execute(conn)?;

        // 2. budget_categories & budgets
        println!("Seeding budgets...");
        for i in 0..50 {
            let cat_id = next_id(conn, IdPrefix::FINANCIAL);
            insert_into(budget_categories::table)
                .values(&BudgetCategory {
                    id: cat_id.clone(),
                    name: format!("Budget Cat {}", i),
                    description: None,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;
            
            insert_into(budgets::table)
                .values(&Budget {
                    id: next_id(conn, IdPrefix::FINANCIAL),
                    academic_year_id: context.academic_year_ids[0].clone(),
                    category_id: cat_id,
                    allocated_amount: 50000.0,
                    spent_amount: 1000.0,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;
        }

        // 3. fee_categories, structures, pricing, schedule, items
        println!("Seeding fee structures...");
        let mut fee_structure_ids = Vec::new();
        for i in 0..20 {
            let cat_id = next_id(conn, IdPrefix::FEE);
            insert_into(fee_categories::table)
                .values(&FeeCategory {
                    id: cat_id.clone(),
                    name: format!("Fee Category {}", i),
                    description: None,
                    is_mandatory: true,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;

            for gl_id in context.grade_level_ids.iter().take(5) {
                let fs_id = next_id(conn, IdPrefix::FEE);
                insert_into(fee_structures::table)
                    .values(&FeeStructure {
                        id: fs_id.clone(),
                        grade_id: gl_id.clone(),
                        academic_year_id: context.academic_year_ids[0].clone(),
                        category_id: cat_id.clone(),
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    })
                    .execute(conn)?;

                insert_into(fee_structure_pricing::table)
                    .values(&FeeStructurePricing {
                        fee_structure_id: fs_id.clone(),
                        amount: 1000.0,
                        currency: "LKR".to_string(),
                        amount_type: FeeAmountType::Fixed,
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    })
                    .execute(conn)?;

                insert_into(fee_structure_schedule::table)
                    .values(&FeeStructureSchedule {
                        fee_structure_id: fs_id.clone(),
                        due_date: None,
                        frequency: FeeFrequency::Monthly,
                        fee_type: FeeTypeEnum::Recurring,
                        effective_from: None,
                        effective_to: None,
                        due_day_of_month: Some(5),
                        is_refundable: false,
                        late_fee_type: None,
                        late_fee_value: None,
                        is_active: true,
                        created_at: Utc::now().naive_utc(),
                        updated_at: Utc::now().naive_utc(),
                    })
                    .execute(conn)?;

                // 3.1 fee_structure_items
                for j in 0..3 {
                    let fsi_id = next_id(conn, IdPrefix::FEE);
                    insert_into(fee_structure_items::table)
                        .values(&(
                            fee_structure_items::id.eq(fsi_id.clone()),
                            fee_structure_items::fee_structure_id.eq(fs_id.clone()),
                            fee_structure_items::item_name.eq(format!("Item {}", j)),
                            fee_structure_items::amount.eq(333.33),
                            fee_structure_items::is_optional.eq(false),
                            fee_structure_items::order_index.eq(j as i32),
                            fee_structure_items::created_at.eq(Utc::now().naive_utc()),
                            fee_structure_items::updated_at.eq(Utc::now().naive_utc()),
                        ))
                        .execute(conn)?;
                }

                fee_structure_ids.push(fs_id);
            }
        }

        // 4. student_fees, fee_payments, allocations, invoices
        println!("Seeding student fees, payments and invoices...");
        for (i, stu_id) in context.student_ids.iter().take(500).enumerate() {
            let fs_id = &fee_structure_ids[i % fee_structure_ids.len()];
            let sf_id = next_id(conn, IdPrefix::FEE);
            insert_into(student_fees::table)
                .values(&StudentFee {
                    id: sf_id.clone(),
                    student_id: stu_id.clone(),
                    fee_structure_id: fs_id.clone(),
                    amount: 1000.0,
                    is_exempted: false,
                    exemption_reason: None,
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;

            let inv_id = next_id(conn, IdPrefix::FEE);
            insert_into(fee_invoices::table)
                .values(&(
                    fee_invoices::id.eq(inv_id.clone()),
                    fee_invoices::student_id.eq(stu_id.clone()),
                    fee_invoices::academic_year_id.eq(context.academic_year_ids[0].clone()),
                    fee_invoices::status.eq("Paid"),
                    fee_invoices::total_amount.eq(1000.0),
                    fee_invoices::balance_amount.eq(0.0),
                    fee_invoices::created_at.eq(Utc::now().naive_utc()),
                    fee_invoices::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(fee_invoice_items::table)
                .values(&(
                    fee_invoice_items::id.eq(next_id(conn, IdPrefix::FEE)),
                    fee_invoice_items::invoice_id.eq(inv_id.clone()),
                    fee_invoice_items::description.eq("Tuition Fee"),
                    fee_invoice_items::quantity.eq(1.0),
                    fee_invoice_items::unit_amount.eq(1000.0),
                    fee_invoice_items::total_amount.eq(1000.0),
                    fee_invoice_items::created_at.eq(Utc::now().naive_utc()),
                    fee_invoice_items::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            let p_id = next_id(conn, IdPrefix::FEE);
            insert_into(fee_payments::table)
                .values(&FeePayment {
                    id: p_id.clone(),
                    student_fee_id: sf_id,
                    amount_paid: 1000.0,
                    payment_date: Utc::now().naive_utc(),
                    collected_by: get_random_id(&context.staff_ids),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;

            insert_into(fee_payment_details::table)
                .values(&FeePaymentDetail {
                    payment_id: p_id.clone(),
                    payment_method: PaymentMethod::Cash,
                    payment_channel: None,
                    payment_status: PaymentStatusType::Completed,
                    receipt_number: format!("REC-{:06}", i),
                    transaction_reference: None,
                    remarks: None,
                    recorded_by: Some(get_random_id(&context.user_ids)),
                    created_at: Utc::now().naive_utc(),
                    updated_at: Utc::now().naive_utc(),
                })
                .execute(conn)?;

            insert_into(fee_payment_allocations::table)
                .values(&(
                    fee_payment_allocations::id.eq(next_id(conn, IdPrefix::FEE)),
                    fee_payment_allocations::payment_id.eq(p_id),
                    fee_payment_allocations::invoice_id.eq(inv_id),
                    fee_payment_allocations::amount.eq(1000.0),
                    fee_payment_allocations::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        // 5. vendors & purchase_orders
        println!("Seeding vendors and purchase orders...");
        let mut vendor_ids = Vec::new();
        for i in 0..50 {
            let id = next_id(conn, IdPrefix::PROPERTY);
            insert_into(vendors::table)
                .values(&(
                    vendors::id.eq(id.clone()),
                    vendors::name.eq(format!("Vendor {}", i)),
                    vendors::created_at.eq(Utc::now().naive_utc()),
                    vendors::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
            vendor_ids.push(id);
        }

        for _ in 0..100 {
            let po_id = next_id(conn, IdPrefix::PROPERTY);
            insert_into(purchase_orders::table)
                .values(&(
                    purchase_orders::id.eq(po_id.clone()),
                    purchase_orders::vendor_id.eq(get_random_id(&vendor_ids)),
                    purchase_orders::order_date.eq(Utc::now().date_naive()),
                    purchase_orders::status.eq("Draft"),
                    purchase_orders::total_amount.eq(5000.0),
                    purchase_orders::created_by.eq(get_random_id(&context.user_ids)),
                    purchase_orders::created_at.eq(Utc::now().naive_utc()),
                    purchase_orders::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
            
            insert_into(purchase_order_items::table)
                .values(&(
                    purchase_order_items::id.eq(next_id(conn, IdPrefix::PROPERTY)),
                    purchase_order_items::purchase_order_id.eq(po_id),
                    purchase_order_items::item_name.eq("Whiteboard Markers"),
                    purchase_order_items::quantity.eq(10.0),
                    purchase_order_items::unit_price.eq(500.0),
                    purchase_order_items::total_price.eq(5000.0),
                    purchase_order_items::created_at.eq(Utc::now().naive_utc()),
                    purchase_order_items::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        // 6. ledger_transactions & entries
        println!("Seeding ledger...");
        for i in 0..200 {
            let tx_id = next_id(conn, IdPrefix::LEDGER);
            insert_into(ledger_transactions::table)
                .values(&(
                    ledger_transactions::id.eq(tx_id.clone()),
                    ledger_transactions::transaction_date.eq(Utc::now().naive_utc()),
                    ledger_transactions::description.eq(format!("Transaction {}", i)),
                    ledger_transactions::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(ledger_entries::table)
                .values(&(
                    ledger_entries::id.eq(next_id(conn, IdPrefix::LEDGER)),
                    ledger_entries::transaction_id.eq(tx_id.clone()),
                    ledger_entries::account_id.eq(get_random_id(&context.chart_of_account_ids)),
                    ledger_entries::entry_type.eq("Debit"),
                    ledger_entries::amount.eq(1000.0),
                    ledger_entries::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(ledger_entries::table)
                .values(&(
                    ledger_entries::id.eq(next_id(conn, IdPrefix::LEDGER)),
                    ledger_entries::transaction_id.eq(tx_id),
                    ledger_entries::account_id.eq(get_random_id(&context.chart_of_account_ids)),
                    ledger_entries::entry_type.eq("Credit"),
                    ledger_entries::amount.eq(1000.0),
                    ledger_entries::created_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        // 7. income & expenses
        println!("Seeding income and expenses...");
        for i in 0..50 {
            let inc_src_id = next_id(conn, IdPrefix::FINANCIAL);
            insert_into(income_sources::table)
                .values(&(
                    income_sources::id.eq(inc_src_id.clone()),
                    income_sources::name.eq(format!("Income Source {}", i)),
                    income_sources::created_at.eq(Utc::now().naive_utc()),
                    income_sources::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(income_transactions::table)
                .values(&(
                    income_transactions::id.eq(next_id(conn, IdPrefix::FINANCIAL)),
                    income_transactions::source_id.eq(inc_src_id),
                    income_transactions::amount.eq(2000.0),
                    income_transactions::date.eq(Utc::now().naive_utc()),
                    income_transactions::received_by.eq(get_random_id(&context.staff_ids)),
                    income_transactions::receipt_number.eq(format!("INC-{:06}", i)),
                    income_transactions::created_at.eq(Utc::now().naive_utc()),
                    income_transactions::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            let exp_cat_id = next_id(conn, IdPrefix::FINANCIAL);
            insert_into(expense_categories::table)
                .values(&(
                    expense_categories::id.eq(exp_cat_id.clone()),
                    expense_categories::name.eq(format!("Expense Cat {}", i)),
                    expense_categories::created_at.eq(Utc::now().naive_utc()),
                    expense_categories::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;

            insert_into(expense_transactions::table)
                .values(&(
                    expense_transactions::id.eq(next_id(conn, IdPrefix::FINANCIAL)),
                    expense_transactions::category_id.eq(exp_cat_id),
                    expense_transactions::amount.eq(500.0),
                    expense_transactions::date.eq(Utc::now().naive_utc()),
                    expense_transactions::payment_method.eq(PaymentMethod::Cash),
                    expense_transactions::created_at.eq(Utc::now().naive_utc()),
                    expense_transactions::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        // 8. petty cash
        println!("Seeding petty cash...");
        for _ in 0..100 {
            insert_into(petty_cash_transactions::table)
                .values(&(
                    petty_cash_transactions::id.eq(next_id(conn, IdPrefix::FINANCIAL)),
                    petty_cash_transactions::amount.eq(100.0),
                    petty_cash_transactions::transaction_type.eq(TransactionType::Spent),
                    petty_cash_transactions::date.eq(Utc::now().naive_utc()),
                    petty_cash_transactions::handled_by.eq(get_random_id(&context.staff_ids)),
                    petty_cash_transactions::created_at.eq(Utc::now().naive_utc()),
                    petty_cash_transactions::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        // 9. inventory_transactions
        println!("Seeding inventory_transactions...");
        let item_ids: &[String] = &context.inventory_item_ids;
        for item_id in item_ids {
            for _ in 0..5 {
                insert_into(inventory_transactions::table)
                    .values(&(
                        inventory_transactions::id.eq(next_id(conn, IdPrefix::PROPERTY)),
                        inventory_transactions::item_id.eq(item_id.clone()),
                        inventory_transactions::transaction_type.eq("Restock"),
                        inventory_transactions::quantity.eq(10.0),
                        inventory_transactions::unit_cost.eq(Some(500.0)),
                        inventory_transactions::transaction_date.eq(Utc::now().naive_utc()),
                        inventory_transactions::created_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(conn)?;
            }
        }

        Ok(())
    }
}
