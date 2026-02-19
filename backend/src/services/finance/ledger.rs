use diesel::prelude::*;
use std::collections::HashMap; // Added
use crate::{
    errors::APIError,
    AppState,
    models::finance::ledger::{GeneralLedgerEntry, NewGeneralLedgerEntry},
    models::finance::account::ChartOfAccount, // Added
};
use actix_web::web;
use uuid::Uuid;
use chrono::Utc;

pub async fn record_transaction(
    pool: web::Data<AppState>,
    transaction_date: chrono::NaiveDate,
    description: Option<String>,
    debit_account_id: String,
    credit_account_id: String,
    amount: f32,
) -> Result<GeneralLedgerEntry, APIError> {
    let mut conn = pool.db_pool.get()?;
    let id = Uuid::new_v4().to_string();

    // Basic validation: ensure amount is positive
    if amount <= 0.0 {
        return Err(APIError::bad_request("Transaction amount must be positive"));
    }

    // Basic validation: ensure debit and credit accounts are different
    if debit_account_id == credit_account_id {
        return Err(APIError::bad_request("Debit and credit accounts cannot be the same"));
    }

    // TODO: Add more robust validation, e.g., checking if accounts exist and are of correct type

    let new_entry = NewGeneralLedgerEntry {
        id: id.clone(),
        transaction_date,
        description,
        debit_account_id,
        credit_account_id,
        amount,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };

    diesel::insert_into(crate::schema::general_ledger::table)
        .values(&new_entry)
        .execute(&mut conn)?;

    Ok(crate::schema::general_ledger::table
        .find(&id)
        .first(&mut conn)?)
}

pub async fn generate_trial_balance(
    pool: web::Data<AppState>,
) -> Result<HashMap<String, f32>, APIError> {
    let mut conn = pool.db_pool.get()?;
    
    let all_accounts = crate::schema::chart_of_accounts::table
        .load::<ChartOfAccount>(&mut conn)?;

    let all_ledger_entries = crate::schema::general_ledger::table
        .load::<GeneralLedgerEntry>(&mut conn)?;

    let mut balances: HashMap<String, f32> = HashMap::new();

    for account in all_accounts {
        balances.insert(account.id.clone(), 0.0);
    }

    for entry in all_ledger_entries {
        *balances.entry(entry.debit_account_id).or_insert(0.0) += entry.amount;
        *balances.entry(entry.credit_account_id).or_insert(0.0) -= entry.amount;
    }

    Ok(balances)
}
