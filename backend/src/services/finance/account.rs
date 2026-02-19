use diesel::prelude::*;
use crate::{
    errors::APIError,
    AppState,
    models::finance::account::{ChartOfAccount, NewChartOfAccount, ChartOfAccountChangeset},
};
use actix_web::web;
use uuid::Uuid;
use chrono::Utc;

pub async fn create_account(
    pool: web::Data<AppState>,
    new_account_request: NewChartOfAccount,
) -> Result<ChartOfAccount, APIError> {
    let mut conn = pool.db_pool.get()?;
    let id = Uuid::new_v4().to_string();

    let new_account = NewChartOfAccount {
        id: id.clone(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        ..new_account_request
    };

    diesel::insert_into(crate::schema::chart_of_accounts::table)
        .values(&new_account)
        .execute(&mut conn)?;

    Ok(crate::schema::chart_of_accounts::table
        .find(&id)
        .first(&mut conn)?)
}

pub async fn get_all_accounts(
    pool: web::Data<AppState>,
) -> Result<Vec<ChartOfAccount>, APIError> {
    let mut conn = pool.db_pool.get()?;
    Ok(crate::schema::chart_of_accounts::table.load::<ChartOfAccount>(&mut conn)?)
}

pub async fn get_account_by_id(
    pool: web::Data<AppState>,
    account_id: String,
) -> Result<ChartOfAccount, APIError> {
    let mut conn = pool.db_pool.get()?;
    Ok(crate::schema::chart_of_accounts::table
        .find(&account_id)
        .first(&mut conn)?)
}

pub async fn update_account(
    pool: web::Data<AppState>,
    account_id: String,
    update_request: ChartOfAccountChangeset,
) -> Result<ChartOfAccount, APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::update(crate::schema::chart_of_accounts::table.find(&account_id))
        .set((update_request, crate::schema::chart_of_accounts::updated_at.eq(Utc::now().naive_utc())))
        .execute(&mut conn)?;
    Ok(crate::schema::chart_of_accounts::table
        .find(&account_id)
        .first(&mut conn)?)
}

pub async fn delete_account(
    pool: web::Data<AppState>,
    account_id: String,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::delete(crate::schema::chart_of_accounts::table.find(&account_id))
        .execute(&mut conn)?;
    Ok(())
}