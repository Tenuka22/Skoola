use super::utils::*;
use super::{SeedModule, SeederContext};
use anyhow::Result;
use backend::config::Config;
use backend::database::enums::{ComponentType, FeeFrequency, PaymentMethod, TransactionType};
use backend::schema::*;
use diesel::insert_into;
use diesel::prelude::*;
use rand::seq::SliceRandom;
use std::collections::HashSet;

use backend::models::finance::account::ChartOfAccount;
use backend::models::finance::budget::Budget;
use backend::models::finance::budget_category::BudgetCategory;
use backend::models::finance::expense_category::ExpenseCategory;
use backend::models::finance::fees::FeeCategory;
use backend::models::finance::fees::FeePayment;
use backend::models::finance::fees::FeeStructure;
use backend::models::finance::fees::StudentFee;
use backend::models::finance::income_source::IncomeSource;
use backend::models::finance::ledger::GeneralLedgerEntry; // Corrected import
use backend::models::finance::petty_cash_transaction::PettyCashTransaction;
use backend::models::finance::salary::SalaryComponent;
use backend::models::finance::salary::SalaryPayment;
use backend::models::finance::salary::StaffSalary;
use backend::models::finance::transaction::ExpenseTransaction;
use backend::models::finance::transaction::IncomeTransaction;
use rand::Rng;

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
        seed_count_config: &crate::SeedCountConfig, // Add SeedCountConfig here
    ) -> Result<()> {
        println!("Seeding Finance module...");

        let mut rng = rand::thread_rng();

        // Seed Chart of Accounts (base accounts)
        let base_chart_of_accounts = vec![
            ("1000", "Cash", "Asset", "Debit"),
            ("1010", "Bank", "Asset", "Debit"),
            ("2000", "Accounts Payable", "Liability", "Credit"),
            ("3000", "Revenue", "Income", "Credit"),
            ("4000", "Expenses", "Expense", "Debit"),
        ];
        let chart_of_accounts_data: Vec<ChartOfAccount> = (0..seed_count_config.chart_of_accounts)
            .map(|i| {
                let (code_prefix, name_prefix, type_prefix, balance_prefix) =
                    base_chart_of_accounts[i % base_chart_of_accounts.len()];
                let account_code = format!(
                    "{}{:03}",
                    code_prefix.chars().take(2).collect::<String>(),
                    i + 1
                );
                let account_name = format!("{} Account {}", name_prefix, i + 1);
                ChartOfAccount {
                    id: generate_uuid(),
                    account_code,
                    account_name,
                    account_type: type_prefix.to_string(),
                    normal_balance: balance_prefix.to_string(),
                    description: None,
                    parent_account_id: None,
                    is_active: true,
                    created_at: random_datetime_in_past(3),
                    updated_at: random_datetime_in_past(2),
                }
            })
            .collect();

        insert_into(chart_of_accounts::table)
            .values(&chart_of_accounts_data)
            .execute(conn)?;
        context.chart_of_account_ids = chart_of_accounts_data
            .into_iter()
            .map(|coa| coa.id)
            .collect();
        println!(
            "Seeded {} chart of accounts.",
            context.chart_of_account_ids.len()
        );

        // Seed Budget Categories
        let budget_categories_data = (0..seed_count_config.budget_categories)
            .map(|i| BudgetCategory {
                id: generate_uuid(),
                name: format!("Budget Category {}", i + 1),
                description: Some(format!("Description for Budget Category {}", i + 1)),
                created_at: random_datetime_in_past(2),
                updated_at: random_datetime_in_past(1),
            })
            .collect::<Vec<BudgetCategory>>();

        insert_into(budget_categories::table)
            .values(&budget_categories_data)
            .execute(conn)?;
        context.budget_category_ids = budget_categories_data.into_iter().map(|bc| bc.id).collect();
        println!(
            "Seeded {} budget categories.",
            context.budget_category_ids.len()
        );

        // Seed Budgets
        if context.academic_year_ids.is_empty() || context.budget_category_ids.is_empty() {
            println!(
                "Skipping Budget seeding: academic_year_ids or budget_category_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let budgets_data = (0..seed_count_config.budgets)
                .map(|_| Budget {
                    id: generate_uuid(),
                    academic_year_id: get_random_id(&context.academic_year_ids),
                    category_id: get_random_id(&context.budget_category_ids),
                    allocated_amount: rng.gen_range(1000.0..=10000.0),
                    spent_amount: rng.gen_range(0.0..=5000.0),
                    created_at: random_datetime_in_past(1),
                    updated_at: random_datetime_in_past(0),
                })
                .collect::<Vec<Budget>>();

            insert_into(budgets::table)
                .values(&budgets_data)
                .execute(conn)?;
            println!("Seeded {} budgets.", budgets_data.len());
        }

        // Seed Income Sources
        let income_sources_data = (0..seed_count_config.income_sources)
            .map(|i| IncomeSource {
                id: generate_uuid(),
                name: format!("Income Source {}", i + 1),
                description: Some(format!("Description for Income Source {}", i + 1)),
                created_at: random_datetime_in_past(2),
                updated_at: random_datetime_in_past(1),
            })
            .collect::<Vec<IncomeSource>>();

        insert_into(income_sources::table)
            .values(&income_sources_data)
            .execute(conn)?;
        context.income_source_ids = income_sources_data.into_iter().map(|is| is.id).collect();
        println!("Seeded {} income sources.", context.income_source_ids.len());

        // Seed Income Transactions
        if context.income_source_ids.is_empty() || context.staff_ids.is_empty() {
            println!(
                "Skipping IncomeTransaction seeding: income_source_ids or staff_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let income_transactions_data = (0..seed_count_config.income_transactions)
                .map(|i| IncomeTransaction {
                    id: generate_uuid(),
                    source_id: get_random_id(&context.income_source_ids),
                    amount: rng.gen_range(500.0..=5000.0),
                    date: random_datetime_in_past(1),
                    description: Some(format!("Income transaction {}", i + 1)),
                    received_by: get_random_id(&context.staff_ids),
                    receipt_number: format!("INC-REC-{}", i + 1),
                    created_at: random_datetime_in_past(1),
                    updated_at: random_datetime_in_past(0),
                })
                .collect::<Vec<IncomeTransaction>>();

            insert_into(income_transactions::table)
                .values(&income_transactions_data)
                .execute(conn)?;
            println!(
                "Seeded {} income transactions.",
                income_transactions_data.len()
            );
        }

        // Seed Expense Categories
        let expense_categories_data = (0..seed_count_config.expense_categories)
            .map(|i| ExpenseCategory {
                id: generate_uuid(),
                name: format!("Expense Category {}", i + 1),
                description: Some(format!("Description for Expense Category {}", i + 1)),
                created_at: random_datetime_in_past(2),
                updated_at: random_datetime_in_past(1),
            })
            .collect::<Vec<ExpenseCategory>>();

        insert_into(expense_categories::table)
            .values(&expense_categories_data)
            .execute(conn)?;
        context.expense_category_ids = expense_categories_data
            .into_iter()
            .map(|ec| ec.id)
            .collect();
        println!(
            "Seeded {} expense categories.",
            context.expense_category_ids.len()
        );

        // Seed Expense Transactions
        if context.expense_category_ids.is_empty() || context.staff_ids.is_empty() {
            println!(
                "Skipping ExpenseTransaction seeding: expense_category_ids or staff_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let expense_transactions_data = (0..seed_count_config.expense_transactions)
                .map(|i| ExpenseTransaction {
                    id: generate_uuid(),
                    category_id: get_random_id(&context.expense_category_ids),
                    amount: rng.gen_range(100.0..=2000.0),
                    date: random_datetime_in_past(1),
                    description: Some(format!("Expense transaction {}", i + 1)),
                    vendor: Some(format!("Vendor {}", i + 1)),
                    payment_method: vec![
                        PaymentMethod::Cash,
                        PaymentMethod::BankTransfer,
                        PaymentMethod::Online,
                    ]
                    .choose(&mut rng)
                    .unwrap()
                    .clone(),
                    approved_by: Some(get_random_id(&context.staff_ids)),
                    receipt_url: if rng.gen_bool(0.5) {
                        Some(format!("http://example.com/receipts/{}.pdf", i + 1))
                    } else {
                        None
                    },
                    created_at: random_datetime_in_past(1),
                    updated_at: random_datetime_in_past(0),
                })
                .collect::<Vec<ExpenseTransaction>>();

            insert_into(expense_transactions::table)
                .values(&expense_transactions_data)
                .execute(conn)?;
            println!(
                "Seeded {} expense transactions.",
                expense_transactions_data.len()
            );
        }

        // Seed Fee Categories
        let fee_categories_data = (0..seed_count_config.fee_categories)
            .map(|i| FeeCategory {
                id: generate_uuid(),
                name: format!("Fee Category {}", i + 1),
                description: Some(format!("Description for Fee Category {}", i + 1)),
                is_mandatory: rng.gen_bool(0.8),
                created_at: random_datetime_in_past(2),
                updated_at: random_datetime_in_past(1),
            })
            .collect::<Vec<FeeCategory>>();

        insert_into(fee_categories::table)
            .values(&fee_categories_data)
            .execute(conn)?;
        context.fee_category_ids = fee_categories_data.into_iter().map(|fc| fc.id).collect();
        println!("Seeded {} fee categories.", context.fee_category_ids.len());

        // Seed Fee Structures
        if context.grade_level_ids.is_empty()
            || context.academic_year_ids.is_empty()
            || context.fee_category_ids.is_empty()
        {
            println!(
                "Skipping FeeStructure seeding: grade_level_ids, academic_year_ids, or fee_category_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let fee_structures_data = (0..seed_count_config.fee_structures)
                .map(|_| FeeStructure {
                    id: generate_uuid(),
                    grade_id: get_random_id(&context.grade_level_ids),
                    academic_year_id: get_random_id(&context.academic_year_ids),
                    category_id: get_random_id(&context.fee_category_ids),
                    amount: rng.gen_range(5000.0..=20000.0),
                    due_date: random_date_in_past(0),
                    frequency: vec![
                        FeeFrequency::Monthly,
                        FeeFrequency::Quarterly,
                        FeeFrequency::Annually,
                    ]
                    .choose(&mut rng)
                    .unwrap()
                    .clone(),
                    created_at: random_datetime_in_past(1),
                    updated_at: random_datetime_in_past(0),
                })
                .collect::<Vec<FeeStructure>>();

            insert_into(fee_structures::table)
                .values(&fee_structures_data)
                .execute(conn)?;
            context.fee_structure_ids = fee_structures_data.into_iter().map(|fs| fs.id).collect();
            println!("Seeded {} fee structures.", context.fee_structure_ids.len());
        }

        // Seed Student Fees
        if context.student_ids.is_empty() || context.fee_structure_ids.is_empty() {
            println!(
                "Skipping StudentFee seeding: student_ids or fee_structure_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let student_fees_data = (0..seed_count_config.student_fees)
                .map(|i| StudentFee {
                    id: generate_uuid(),
                    student_id: get_random_id(&context.student_ids),
                    fee_structure_id: get_random_id(&context.fee_structure_ids),
                    amount: rng.gen_range(5000.0..=20000.0),
                    is_exempted: rng.gen_bool(0.1),
                    exemption_reason: if rng.gen_bool(0.1) {
                        Some(format!("Financial Hardship {}", i + 1))
                    } else {
                        None
                    },
                    created_at: random_datetime_in_past(1),
                    updated_at: random_datetime_in_past(0),
                })
                .collect::<Vec<StudentFee>>();

            insert_into(student_fees::table)
                .values(&student_fees_data)
                .execute(conn)?;
            context.student_fee_ids = student_fees_data.into_iter().map(|sf| sf.id).collect();
            println!("Seeded {} student fees.", context.student_fee_ids.len());
        }

        // Seed Fee Payments
        if context.student_fee_ids.is_empty() || context.staff_ids.is_empty() {
            println!(
                "Skipping FeePayment seeding: student_fee_ids or staff_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let fee_payments_data = (0..seed_count_config.fee_payments)
                .map(|i| FeePayment {
                    id: generate_uuid(),
                    student_fee_id: get_random_id(&context.student_fee_ids),
                    amount_paid: rng.gen_range(1000.0..=15000.0),
                    payment_date: random_datetime_in_past(0),
                    payment_method: vec![
                        PaymentMethod::Cash,
                        PaymentMethod::BankTransfer,
                        PaymentMethod::Online,
                    ]
                    .choose(&mut rng)
                    .unwrap()
                    .clone(),
                    receipt_number: format!("FEE-REC-{}", i + 1),
                    collected_by: get_random_id(&context.staff_ids),
                    remarks: if rng.gen_bool(0.2) {
                        Some(format!("Paid for month {}", (i % 12) + 1))
                    } else {
                        None
                    },
                    created_at: random_datetime_in_past(0),
                    updated_at: random_datetime_in_past(0),
                })
                .collect::<Vec<FeePayment>>();

            insert_into(fee_payments::table)
                .values(&fee_payments_data)
                .execute(conn)?;
            println!("Seeded {} fee payments.", fee_payments_data.len());
        }

        // Seed Salary Components
        let salary_components_data = (0..seed_count_config.salary_components)
            .map(|i| SalaryComponent {
                id: generate_uuid(),
                name: format!("Component {}", i + 1),
                component_type: vec![
                    ComponentType::Allowance,
                    ComponentType::Deduction,
                    ComponentType::Bonus,
                ]
                .choose(&mut rng)
                .unwrap()
                .clone(),
                description: Some(format!("Description for Component {}", i + 1)),
                created_at: random_datetime_in_past(2),
                updated_at: random_datetime_in_past(1),
            })
            .collect::<Vec<SalaryComponent>>();

        insert_into(salary_components::table)
            .values(&salary_components_data)
            .execute(conn)?;
        context.salary_component_ids = salary_components_data.into_iter().map(|sc| sc.id).collect();
        println!(
            "Seeded {} salary components.",
            context.salary_component_ids.len()
        );

        // Seed Staff Salaries
        if context.staff_ids.is_empty() || context.salary_component_ids.is_empty() {
            println!(
                "Skipping StaffSalary seeding: staff_ids or salary_component_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let mut used_staff_salaries = HashSet::new();
            let staff_salaries_data = (0..seed_count_config.staff_salaries)
                .filter_map(|_| {
                    let staff_id = get_random_id(&context.staff_ids);
                    let component_id = get_random_id(&context.salary_component_ids);

                    if used_staff_salaries.contains(&(staff_id.clone(), component_id.clone())) {
                        return None;
                    }
                    used_staff_salaries.insert((staff_id.clone(), component_id.clone()));

                    Some(StaffSalary {
                        staff_id,
                        component_id,
                        amount: rng.gen_range(10000.0..=100000.0),
                        effective_from: random_date_in_past(1),
                        created_at: random_datetime_in_past(1),
                        updated_at: random_datetime_in_past(0),
                    })
                })
                .collect::<Vec<StaffSalary>>();

            insert_into(staff_salaries::table)
                .values(&staff_salaries_data)
                .execute(conn)?;
            println!("Seeded {} staff salaries.", staff_salaries_data.len());
        }

        // Seed Salary Payments
        if context.staff_ids.is_empty() {
            println!(
                "Skipping SalaryPayment seeding: staff_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let salary_payments_data = (0..seed_count_config.salary_payments)
                .map(|i| SalaryPayment {
                    id: generate_uuid(),
                    staff_id: get_random_id(&context.staff_ids),
                    payment_month: rng.gen_range(1..=12),
                    payment_year: rng.gen_range(2023..=2025),
                    gross_salary: rng.gen_range(50000.0..=150000.0),
                    total_deductions: rng.gen_range(5000.0..=20000.0),
                    net_salary: rng.gen_range(40000.0..=130000.0),
                    payment_date: random_datetime_in_past(0),
                    payment_method: PaymentMethod::BankTransfer
                        .to_string(),
                    remarks: if rng.gen_bool(0.2) {
                        Some(format!("Bonus included for Q{}", (i % 4) + 1))
                    } else {
                        None
                    },
                    created_at: random_datetime_in_past(0),
                    updated_at: random_datetime_in_past(0),
                })
                .collect::<Vec<SalaryPayment>>();

            insert_into(salary_payments::table)
                .values(&salary_payments_data)
                .execute(conn)?;
            println!("Seeded {} salary payments.", salary_payments_data.len());
        }

        // Seed Petty Cash Transactions
        if context.staff_ids.is_empty() {
            println!(
                "Skipping PettyCashTransaction seeding: staff_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let petty_cash_transactions_data = (0..seed_count_config.petty_cash_transactions)
                .map(|i| PettyCashTransaction {
                    id: generate_uuid(),
                    amount: rng.gen_range(100.0..=1000.0),
                    transaction_type: vec![
                        TransactionType::Received,
                        TransactionType::Spent,
                    ]
                    .choose(&mut rng)
                    .unwrap()
                    .clone(),
                    date: random_datetime_in_past(0),
                    description: Some(format!("Petty cash transaction {}", i + 1)),
                    handled_by: get_random_id(&context.staff_ids),
                    created_at: random_datetime_in_past(0),
                    updated_at: random_datetime_in_past(0),
                })
                .collect::<Vec<PettyCashTransaction>>();

            insert_into(petty_cash_transactions::table)
                .values(&petty_cash_transactions_data)
                .execute(conn)?;
            println!(
                "Seeded {} petty cash transactions.",
                petty_cash_transactions_data.len()
            );
        }

        // Seed General Ledger entries
        if context.chart_of_account_ids.is_empty() {
            println!(
                "Skipping GeneralLedger seeding: chart_of_account_ids are empty. Ensure relevant seeders run first."
            );
        } else {
            let general_ledger_data = (0..seed_count_config.general_ledger_entries)
                .map(|i| {
                    let debit_account_id = get_random_id(&context.chart_of_account_ids);
                    let credit_account_id = get_random_id(&context.chart_of_account_ids);
                    GeneralLedgerEntry {
                        id: generate_uuid(),
                        transaction_date: random_date_in_past(0),
                        description: Some(format!("GL entry {}", i + 1)),
                        debit_account_id,
                        credit_account_id,
                        amount: rng.gen_range(10.0..=5000.0),
                        created_at: random_datetime_in_past(0),
                        updated_at: random_datetime_in_past(0),
                    }
                })
                .collect::<Vec<GeneralLedgerEntry>>();

            insert_into(general_ledger::table)
                .values(&general_ledger_data)
                .execute(conn)?;
            println!(
                "Seeded {} general ledger entries.",
                general_ledger_data.len()
            );
        }

        Ok(())
    }
}
