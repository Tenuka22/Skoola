use crate::AppState;
use crate::models::system::BulkDeleteUsersRequest;
use crate::schema::{user_status, users};
use actix_web::web;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use tracing::{error, info}; // Import the new DTO

pub async fn remove_unverified_users(data: web::Data<AppState>) {
    info!("Starting unverified user cleanup job.");
    let mut conn = match data.db_pool.get() {
        Ok(conn) => conn,
        Err(e) => {
            error!("Failed to get DB connection for cleanup job: {:?}", e);
            return;
        }
    };

    let one_hour_ago = Utc::now().naive_utc() - Duration::hours(1);

    let candidate_ids: Vec<String> = users::table
        .inner_join(user_status::table.on(users::id.eq(user_status::user_id)))
        .filter(user_status::is_verified.eq(false))
        .filter(users::created_at.lt(one_hour_ago))
        .select(users::id)
        .load(&mut conn)
        .unwrap_or_default();

    match diesel::update(user_status::table.filter(user_status::user_id.eq_any(candidate_ids)))
        .set((
            user_status::is_active.eq(false),
            user_status::disabled_at.eq(Some(Utc::now().naive_utc())),
            user_status::disabled_reason.eq(Some("Unverified timeout".to_string())),
            user_status::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)
    {
        Ok(num_deleted) => {
            if num_deleted > 0 {
                info!(
                    "Disabled {} unverified users older than one hour.",
                    num_deleted
                );
            } else {
                info!("No unverified users older than one hour found to disable.");
            }
        }
        Err(e) => {
            error!("Error removing unverified users: {:?}", e);
        }
    }
}

pub async fn bulk_delete_users(
    data: web::Data<AppState>,
    delete_request: BulkDeleteUsersRequest,
) -> Result<(), anyhow::Error> {
    info!(
        "Attempting to bulk delete users: {:?}",
        delete_request.user_ids
    );

    let mut conn = data.db_pool.get()?;

    let num_deleted =
        diesel::delete(users::table.filter(users::id.eq_any(&delete_request.user_ids)))
            .execute(&mut conn)?;

    info!("Successfully deleted {} users.", num_deleted);
    Ok(())
}
