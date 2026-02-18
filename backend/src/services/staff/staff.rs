use diesel::prelude::*;
use diesel::{QueryDsl, RunQueryDsl};
use crate::{
    errors::APIError,
    AppState,
    schema::staff,
};
use actix_web::web;
use chrono::Utc;
use crate::handlers::staff::staff::{BulkUpdateStaffRequest};

pub async fn bulk_delete_staff(
    pool: web::Data<AppState>,
    staff_ids: Vec<String>,
) -> Result<(), APIError> {
    let mut conn = pool.db_pool.get()?;
    diesel::delete(staff::table.filter(staff::id.eq_any(staff_ids)))
        .execute(&mut conn)?;
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
                body.name.map(|n| staff::name.eq(n)),
                body.employee_id.map(|ei| staff::employee_id.eq(ei)),
                body.nic.map(|nic| staff::nic.eq(nic)),
                body.dob.map(|dob| staff::dob.eq(dob)),
                body.gender.map(|g| staff::gender.eq(g)),
                body.address.map(|a| staff::address.eq(a)),
                body.phone.map(|p| staff::phone.eq(p)),
                body.email.map(|e| staff::email.eq(e)),
                body.photo_url.map(|pu| staff::photo_url.eq(Some(pu))), // Assuming photo_url can be None
                body.employment_status.map(|es| staff::employment_status.eq(es)),
                body.staff_type.map(|st| staff::staff_type.eq(st)),
                staff::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(conn)?;
        
        Ok(())
    })
}