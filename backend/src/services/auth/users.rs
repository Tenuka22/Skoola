use crate::database::enums::RoleEnum;
use crate::schema::{users, user_status, user_profiles, profiles, profile_contacts, profile_media, staff, students};
use crate::{
    AppState,
    errors::APIError,
    models::auth::user::{User, UserResponse, UpdateUserRequest, UserQuery},
};
use crate::impl_admin_entity_service;
use actix_web::web;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{Connection, QueryDsl, ExpressionMethods};
use crate::services::admin_db::BulkUpdateRequest;

impl_admin_entity_service!(
    UserService,
    users::table,
    User,
    UserResponse,
    users::id,
    UserQuery,
    |q: users::BoxedQuery<'static, diesel::sqlite::Sqlite>, search: String| {
        q.filter(users::email.like(search))
    },
    |q: users::BoxedQuery<'static, diesel::sqlite::Sqlite>, sort_by, sort_order| {
        match (sort_by, sort_order) {
            ("email", "asc") => q.order(users::email.asc()),
            ("email", "desc") => q.order(users::email.desc()),
            _ => q.order(users::created_at.desc()),
        }
    }
);

impl UserService {
    pub async fn bulk_update_users(
        pool: web::Data<AppState>,
        req: BulkUpdateRequest<UpdateUserRequest>,
    ) -> Result<(), APIError> {
        for update in req.updates {
            Self::update_with_logic(pool.clone(), update.id, update.data).await?;
        }
        Ok(())
    }

    pub async fn update_with_logic(
        pool: web::Data<AppState>,
        id: String,
        req: UpdateUserRequest,
    ) -> Result<UserResponse, APIError> {
        let mut conn = pool.db_pool.get()?;
        
        conn.transaction::<UserResponse, APIError, _>(|conn| {
            let now = Utc::now().naive_utc();
            
            // 1. Update users table
            if req.email.is_some() || req.password.is_some() || req.role.is_some() {
                diesel::update(users::table.filter(users::id.eq(&id)))
                    .set(users::updated_at.eq(now))
                    .execute(conn)?;
                
                if let Some(email) = &req.email {
                    diesel::update(users::table.filter(users::id.eq(&id))).set(users::email.eq(email)).execute(conn)?;
                }
                if let Some(password) = &req.password {
                    diesel::update(users::table.filter(users::id.eq(&id))).set(users::password_hash.eq(password)).execute(conn)?;
                }
                if let Some(role) = &req.role {
                    diesel::update(users::table.filter(users::id.eq(&id))).set(users::role.eq(role)).execute(conn)?;
                }
            }

            // 2. Update user_status
            if req.is_verified.is_some() || req.is_active.is_some() || req.disabled_reason.is_some() {
                if let Some(is_verified) = req.is_verified {
                    diesel::update(user_status::table.filter(user_status::user_id.eq(&id)))
                        .set(user_status::is_verified.eq(is_verified))
                        .execute(conn)?;
                }
                if let Some(is_active) = req.is_active {
                    diesel::update(user_status::table.filter(user_status::user_id.eq(&id)))
                        .set(user_status::is_active.eq(is_active))
                        .execute(conn)?;
                    if !is_active {
                        diesel::update(user_status::table.filter(user_status::user_id.eq(&id)))
                            .set(user_status::disabled_at.eq(now))
                            .execute(conn)?;
                    }
                }
                if let Some(reason) = &req.disabled_reason {
                    diesel::update(user_status::table.filter(user_status::user_id.eq(&id)))
                        .set(user_status::disabled_reason.eq(reason))
                        .execute(conn)?;
                }
            }

            // 3. Profiles and related
            let profile_id_opt: Option<String> = user_profiles::table
                .filter(user_profiles::user_id.eq(&id))
                .select(user_profiles::profile_id)
                .first(conn)
                .optional()?;

            if let Some(profile_id_val) = profile_id_opt {
                if let Some(name) = &req.name {
                    diesel::update(profiles::table.filter(profiles::id.eq(&profile_id_val)))
                        .set(profiles::name.eq(name))
                        .execute(conn)?;
                }
                
                if req.address.is_some() || req.phone.is_some() {
                    if let Some(address) = &req.address {
                         diesel::update(profile_contacts::table.filter(profile_contacts::profile_id.eq(&profile_id_val)))
                            .set(profile_contacts::address.eq(address))
                            .execute(conn)?;
                    }
                    if let Some(phone) = &req.phone {
                         diesel::update(profile_contacts::table.filter(profile_contacts::profile_id.eq(&profile_id_val)))
                            .set(profile_contacts::phone.eq(phone))
                            .execute(conn)?;
                    }
                }

                if let Some(photo_url) = &req.photo_url {
                    diesel::update(profile_media::table.filter(profile_media::profile_id.eq(&profile_id_val)))
                        .set(profile_media::photo_url.eq(photo_url))
                        .execute(conn)?;
                }

                // 4. Staff / Student specific
                let user_role: RoleEnum = users::table.filter(users::id.eq(&id)).select(users::role).first(conn)?;
                
                match user_role {
                    RoleEnum::Teacher | RoleEnum::Admin | RoleEnum::Principal | RoleEnum::VicePrincipal | RoleEnum::Accountant | RoleEnum::Librarian | RoleEnum::FullAdmin => {
                        // Update staff table
                        if let Some(name) = &req.name {
                            diesel::update(staff::table.filter(staff::profile_id.eq(&profile_id_val)))
                                .set(staff::name.eq(name))
                                .execute(conn)?;
                        }
                        if let Some(dob) = req.dob {
                            diesel::update(staff::table.filter(staff::profile_id.eq(&profile_id_val)))
                                .set(staff::dob.eq(dob))
                                .execute(conn)?;
                        }
                        if let Some(gender) = req.gender {
                            diesel::update(staff::table.filter(staff::profile_id.eq(&profile_id_val)))
                                .set(staff::gender.eq(gender))
                                .execute(conn)?;
                        }
                        if let Some(staff_type) = req.staff_type {
                            diesel::update(staff::table.filter(staff::profile_id.eq(&profile_id_val)))
                                .set(staff::staff_type.eq(staff_type))
                                .execute(conn)?;
                        }
                    },
                    RoleEnum::Student => {
                        // Update students table
                        if let Some(name) = &req.name {
                            diesel::update(students::table.filter(students::profile_id.eq(&profile_id_val)))
                                .set(students::name_english.eq(name))
                                .execute(conn)?;
                        }
                        if let Some(name_si) = &req.name_sinhala {
                            diesel::update(students::table.filter(students::profile_id.eq(&profile_id_val)))
                                .set(students::name_sinhala.eq(name_si))
                                .execute(conn)?;
                        }
                        if let Some(name_ta) = &req.name_tamil {
                            diesel::update(students::table.filter(students::profile_id.eq(&profile_id_val)))
                                .set(students::name_tamil.eq(name_ta))
                                .execute(conn)?;
                        }
                        if let Some(dob) = req.dob {
                            diesel::update(students::table.filter(students::profile_id.eq(&profile_id_val)))
                                .set(students::dob.eq(dob))
                                .execute(conn)?;
                        }
                        if let Some(gender) = req.gender {
                            diesel::update(students::table.filter(students::profile_id.eq(&profile_id_val)))
                                .set(students::gender.eq(gender))
                                .execute(conn)?;
                        }
                    },
                    _ => {}
                }
            }

            let updated_user: User = users::table.filter(users::id.eq(&id)).first(conn)?;
            Ok(UserResponse::from(updated_user))
        })
    }
}
