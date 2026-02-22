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
        account_code: new_account_request.account_code,
        account_name: new_account_request.account_name,
        account_type: new_account_request.account_type,
        normal_balance: new_account_request.normal_balance,
        description: new_account_request.description,
        parent_account_id: new_account_request.parent_account_id,
        is_active: new_account_request.is_active,
    };

    diesel::insert_into(crate::schema::chart_of_accounts::table)
        .values(&new_account)
        .execute(&mut conn)?;

    Ok(crate::schema::chart_of_accounts::table
        .find(&id)
        .select(ChartOfAccount::as_select())
        .first(&mut conn)?)
}

pub async fn get_all_accounts(
    pool: web::Data<AppState>,
) -> Result<Vec<ChartOfAccount>, APIError> {
    let mut conn = pool.db_pool.get()?;
    Ok(crate::schema::chart_of_accounts::table
        .select(ChartOfAccount::as_select())
        .load::<ChartOfAccount>(&mut conn)?)
}

pub async fn get_account_by_id(
    pool: web::Data<AppState>,
    account_id: String,
) -> Result<ChartOfAccount, APIError> {
    let mut conn = pool.db_pool.get()?;
    Ok(crate::schema::chart_of_accounts::table
        .find(&account_id)
        .select(ChartOfAccount::as_select())
        .first(&mut conn)?)
}

pub async fn update_account(
    pool: web::Data<AppState>,
    account_id: String,
    update_request: ChartOfAccountChangeset,
) -> Result<ChartOfAccount, APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::update(crate::schema::chart_of_accounts::table.find(&account_id))
        .set((
            update_request.account_code.map(|c| crate::schema::chart_of_accounts::account_code.eq(c)),
            update_request.account_name.map(|n| crate::schema::chart_of_accounts::account_name.eq(n)),
            update_request.account_type.map(|t| crate::schema::chart_of_accounts::account_type.eq(t)),
            update_request.normal_balance.map(|n| crate::schema::chart_of_accounts::normal_balance.eq(n)),
            update_request.description.map(|d| crate::schema::chart_of_accounts::description.eq(d)),
            update_request.parent_account_id.map(|p| crate::schema::chart_of_accounts::parent_account_id.eq(p)),
            update_request.is_active.map(|a| crate::schema::chart_of_accounts::is_active.eq(a)),
            crate::schema::chart_of_accounts::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;
    Ok(crate::schema::chart_of_accounts::table
        .find(&account_id)
        .select(ChartOfAccount::as_select())
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