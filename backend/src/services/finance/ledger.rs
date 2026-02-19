use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::finance::ledger::{GeneralLedgerEntry, NewGeneralLedgerEntry},
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
