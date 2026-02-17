use diesel::prelude::*;
use crate::schema::users;
use crate::AppState;
use chrono::{Duration, Utc};
use tracing::{info, error};
use actix_web::web;

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

    match diesel::delete(
        users::table
            .filter(users::is_verified.eq(false))
            .filter(users::created_at.lt(one_hour_ago))
    )
    .execute(&mut conn)
    {
        Ok(num_deleted) => {
            if num_deleted > 0 {
                info!("Removed {} unverified users older than one hour.", num_deleted);
            } else {
                info!("No unverified users older than one hour found to remove.");
            }
        }
        Err(e) => {
            error!("Error removing unverified users: {:?}", e);
        }
    }
}
