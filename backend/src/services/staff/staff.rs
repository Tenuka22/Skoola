use crate::AppState;
use crate::errors::APIError;
use crate::models::staff::staff::{CreateStaffRequest, Staff, StaffQuery, StaffResponse, StaffContact, StaffContactResponse, StaffContactQuery, CreateStaffContactRequest, StaffMedia, StaffMediaResponse, CreateStaffMediaRequest, StaffRewardSnapshot, StaffRewardSnapshotResponse, CreateStaffRewardSnapshotRequest};
use crate::models::ids::{generate_prefixed_id, IdPrefix};
use diesel::prelude::*;
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;

use crate::schema::{staff, profiles, staff_contacts, staff_media, staff_identity, staff_employment_status, staff_reward_snapshots};
use crate::models::staff::staff::{UpdateStaffRequest};
use crate::services::admin_db::BulkUpdateRequest;

impl_admin_entity_service!(
    StaffService,
    staff::table,
    Staff,
    StaffResponse,
    staff::id,
    StaffQuery,
    |q: staff::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        let pattern = format!("%{}%", search);
        q.filter(staff::name.like(pattern))
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
        let id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;
        
        let new_item = Staff {
            id,
            employee_id: req.employee_id,
            name: req.name,
            dob: req.dob,
            gender: req.gender,
            staff_type: req.staff_type,
            profile_id: None,
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        Self::generic_create(pool, new_item).await
    }

    pub async fn bulk_update_staff(
        pool: web::Data<AppState>,
        req: BulkUpdateRequest<UpdateStaffRequest>,
    ) -> Result<(), APIError> {
        for update in req.updates {
            Self::update_with_logic(pool.clone(), update.id, update.data).await?;
        }
        Ok(())
    }

    pub async fn update_with_logic(
        pool: web::Data<AppState>,
        id: String,
        req: UpdateStaffRequest,
    ) -> Result<StaffResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        
        conn.transaction::<StaffResponse, APIError, _>(|conn| {
            let now = Utc::now().naive_utc();
            
            // 1. Update staff table
            if req.employee_id.is_some() || req.name.is_some() || req.dob.is_some() || req.gender.is_some() || req.staff_type.is_some() || req.profile_id.is_some() {
                diesel::update(staff::table.filter(staff::id.eq(&id)))
                    .set(staff::updated_at.eq(now))
                    .execute(conn)?;
                
                if let Some(employee_id) = &req.employee_id {
                    diesel::update(staff::table.filter(staff::id.eq(&id))).set(staff::employee_id.eq(employee_id)).execute(conn)?;
                }
                if let Some(name) = &req.name {
                    diesel::update(staff::table.filter(staff::id.eq(&id))).set(staff::name.eq(name)).execute(conn)?;
                }
                if let Some(dob) = req.dob {
                    diesel::update(staff::table.filter(staff::id.eq(&id))).set(staff::dob.eq(dob)).execute(conn)?;
                }
                if let Some(gender) = req.gender {
                    diesel::update(staff::table.filter(staff::id.eq(&id))).set(staff::gender.eq(gender)).execute(conn)?;
                }
                if let Some(staff_type) = req.staff_type {
                    diesel::update(staff::table.filter(staff::id.eq(&id))).set(staff::staff_type.eq(staff_type)).execute(conn)?;
                }
                if let Some(profile_id) = &req.profile_id {
                    diesel::update(staff::table.filter(staff::id.eq(&id))).set(staff::profile_id.eq(profile_id)).execute(conn)?;
                }
            }

            // 2. Profiles (if profile_id exists)
            let profile_id_opt: Option<String> = staff::table
                .filter(staff::id.eq(&id))
                .select(staff::profile_id)
                .first(conn)
                .optional()?
                .flatten();

            if let Some(profile_id_val) = profile_id_opt {
                if let Some(name) = &req.profile_name {
                    diesel::update(profiles::table.filter(profiles::id.eq(&profile_id_val)))
                        .set(profiles::name.eq(name))
                        .execute(conn)?;
                }
            }

            // 3. staff_contacts
            if req.address.is_some() || req.phone.is_some() || req.email.is_some() {
                if let Some(address) = &req.address {
                    diesel::update(staff_contacts::table.filter(staff_contacts::staff_id.eq(&id)))
                        .set(staff_contacts::address.eq(address))
                        .execute(conn)?;
                }
                if let Some(phone) = &req.phone {
                    diesel::update(staff_contacts::table.filter(staff_contacts::staff_id.eq(&id)))
                        .set(staff_contacts::phone.eq(phone))
                        .execute(conn)?;
                }
                if let Some(email) = &req.email {
                    diesel::update(staff_contacts::table.filter(staff_contacts::staff_id.eq(&id)))
                        .set(staff_contacts::email.eq(email))
                        .execute(conn)?;
                }
            }

            // 4. staff_media
            if let Some(photo_url) = &req.photo_url {
                diesel::update(staff_media::table.filter(staff_media::staff_id.eq(&id)))
                    .set(staff_media::photo_url.eq(photo_url))
                    .execute(conn)?;
            }

            // 5. staff_identity
            if let Some(nic) = &req.nic {
                diesel::update(staff_identity::table.filter(staff_identity::staff_id.eq(&id)))
                    .set(staff_identity::nic.eq(nic))
                    .execute(conn)?;
            }

            // 6. staff_employment_status
            if let Some(status) = req.employment_status {
                diesel::update(staff_employment_status::table.filter(staff_employment_status::staff_id.eq(&id)))
                    .set(staff_employment_status::employment_status.eq(status))
                    .execute(conn)?;
            }

            let updated: Staff = staff::table.filter(staff::id.eq(&id)).first(conn)?;
            Ok(StaffResponse::from(updated))
        })
    }
}

impl_admin_entity_service!(
    StaffContactService,
    staff_contacts::table,
    StaffContact,
    StaffContactResponse,
    staff_contacts::staff_id,
    staff_id,
    StaffContactQuery,
    |q: staff_contacts::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(staff_contacts::address.like(search))
    },
    |q: staff_contacts::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(staff_contacts::created_at.desc()),
        }
    }
);

impl StaffContactService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateStaffContactRequest,
    ) -> Result<StaffContactResponse, APIError> {
        let now = Utc::now().naive_utc();
        let new_item = StaffContact {
            staff_id: req.staff_id,
            address: req.address,
            phone: req.phone,
            email: req.email,
            address_latitude: req.address_latitude,
            address_longitude: req.address_longitude,
            created_at: now,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    StaffMediaService,
    staff_media::table,
    StaffMedia,
    StaffMediaResponse,
    staff_media::staff_id,
    staff_id,
    crate::services::admin_db::AdminQuery,
    |q: staff_media::BoxedQuery<'static, diesel::sqlite::Sqlite>, _pattern: String| {
        q
    },
    |q: staff_media::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(staff_media::created_at.desc()),
        }
    }
);

impl StaffMediaService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateStaffMediaRequest,
    ) -> Result<StaffMediaResponse, APIError> {
        let now = Utc::now().naive_utc();
        let new_item = StaffMedia {
            staff_id: req.staff_id,
            photo_url: req.photo_url,
            created_at: now,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}

impl_admin_entity_service!(
    StaffRewardSnapshotService,
    staff_reward_snapshots::table,
    StaffRewardSnapshot,
    StaffRewardSnapshotResponse,
    staff_reward_snapshots::staff_id,
    staff_id,
    crate::services::admin_db::AdminQuery,
    |q: staff_reward_snapshots::BoxedQuery<'static, diesel::sqlite::Sqlite>, _pattern: String| {
        q
    },
    |q: staff_reward_snapshots::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            _ => q.order(staff_reward_snapshots::updated_at.desc()),
        }
    }
);

impl StaffRewardSnapshotService {
    pub async fn create_with_logic(
        pool: web::Data<AppState>,
        req: CreateStaffRewardSnapshotRequest,
    ) -> Result<StaffRewardSnapshotResponse, APIError> {
        let now = Utc::now().naive_utc();
        let new_item = StaffRewardSnapshot {
            staff_id: req.staff_id,
            reward_points_balance: req.reward_points_balance,
            updated_at: now,
        };

        Self::generic_create(pool, new_item).await
    }
}
