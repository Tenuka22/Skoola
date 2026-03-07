use crate::handlers::staff::staff::BulkUpdateStaffRequest;
use crate::{
    AppState,
    errors::APIError,
    schema::{
        profiles, staff, staff_contacts, staff_employment_status, staff_identity, staff_media,
    },
};
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};

pub async fn bulk_delete_staff(
    pool: web::Data<AppState>,
    staff_ids: Vec<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::delete(staff::table.filter(staff::id.eq_any(staff_ids))).execute(&mut conn)?;
    Ok(())
}

pub async fn bulk_update_staff(
    pool: web::Data<AppState>,
    body: BulkUpdateStaffRequest,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;

    conn.transaction::<_, APIError, _>(|conn| {
        let target = staff::table.filter(staff::id.eq_any(&body.staff_ids));

        diesel::update(target)
            .set((
                body.employee_id.map(|ei| staff::employee_id.eq(ei)),
                body.name.clone().map(|n| staff::name.eq(n)),
                body.dob.map(|dob| staff::dob.eq(dob)),
                body.gender.map(|g| staff::gender.eq(g)),
                body.staff_type.map(|st| staff::staff_type.eq(st)),
                staff::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;

        let profile_ids: Vec<String> = staff::table
            .filter(staff::id.eq_any(&body.staff_ids))
            .select(staff::profile_id)
            .load::<Option<String>>(conn)?
            .into_iter()
            .filter_map(|id| id)
            .collect();

        if !profile_ids.is_empty() {
            diesel::update(profiles::table.filter(profiles::id.eq_any(profile_ids)))
                .set((
                    body.name.map(|n| profiles::name.eq(n)),
                    profiles::updated_at.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)?;
        }

        Ok(())
    })
}
