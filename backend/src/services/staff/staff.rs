use crate::models::staff::staff::{StaffQuery, StaffResponse, CreateStaffRequest, UpdateStaffRequest};
use crate::schema::{
    profiles, staff, staff_contacts, staff_employment_status, staff_identity, staff_media,
};
use crate::{
    AppState,
    errors::APIError,
    database::tables::Staff as DbStaff,
    models::{NewProfile, Profile},
};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;

impl_admin_entity_service!(
    StaffService,
    staff::table,
    DbStaff,
    StaffResponse,
    staff::id,
    StaffQuery,
    |q: staff::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(staff::name.like(search.clone())
            .or(staff::employee_id.like(search)))
    },
    |q: staff::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("name", "asc") => q.order(staff::name.asc()),
            ("name", "desc") => q.order(staff::name.desc()),
            _ => q.order(staff::created_at.desc()),
        }
    }
);

impl StaffService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateStaffRequest,
    ) -> Result<StaffResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        
        let profile_id = generate_prefixed_id(&mut conn, IdPrefix::PROFILE)?;
        let staff_id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;

        let res = conn.transaction::<_, APIError, _>(|conn| {
            let new_profile = NewProfile {
                id: profile_id.clone(),
                name: req.name.clone(),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            };
            diesel::insert_into(profiles::table).values(&new_profile).execute(conn)?;

            let new_staff = DbStaff {
                id: staff_id.clone(),
                employee_id: req.employee_id.clone(),
                name: req.name.clone(),
                dob: req.dob,
                gender: req.gender.clone(),
                staff_type: req.staff_type.clone(),
                profile_id: Some(profile_id.clone()),
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
            };
            diesel::insert_into(staff::table).values(&new_staff).execute(conn)?;

            Ok(new_staff)
        })?;

        Self::generic_create(pool, res).await
    }

    pub async fn update_with_logic(
        pool: web::Data<AppState>,
        id: String,
        req: UpdateStaffRequest,
    ) -> Result<StaffResponse, APIError> {
        Self::generic_update(pool, id, req).await
    }
}
