use actix_multipart::Multipart;
use actix_web::web;
use actix_web::web::Json;
use apistos::{ApiComponent, api_operation};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use futures_util::stream::{StreamExt, TryStreamExt};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fs::create_dir_all;
use std::io::Write;

use crate::{
    AppState,
    database::tables::Staff as DbStaff,
    errors::APIError,
    models::staff::staff::{
        CreateStaffRequest, PaginatedStaffResponse, StaffQuery, StaffResponse, UpdateStaffRequest,
    },
    models::{
        MessageResponse, NewProfile, NewUserProfile, Profile, UserProfile, auth::user::User,
        ids::{IdPrefix, generate_prefixed_id},
    },
    schema::{
        profiles, staff, staff_contacts, staff_employment_status, staff_identity, staff_media,
        staff_reward_snapshots, user_profiles, users,
    },
    utils::validation::{is_valid_email, is_valid_nic, is_valid_phone},
};

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkDeleteStaffRequest {
    pub staff_ids: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateStaffRequest {
    pub staff_ids: Vec<String>,
    pub name: Option<String>,
    pub employee_id: Option<String>,
    pub nic: Option<String>,
    pub dob: Option<chrono::NaiveDate>,
    pub gender: Option<crate::database::enums::Gender>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub photo_url: Option<String>,
    pub employment_status: Option<crate::database::enums::EmploymentStatus>,
    pub staff_type: Option<crate::database::enums::StaffType>,
}

#[api_operation(
    summary = "Upload a staff photo",
    description = "Uploads a photo for a staff member.",
    tag = "staff",
    operation_id = "upload_staff_photo"
)]
pub async fn upload_staff_photo(
    data: web::Data<AppState>,
    staff_id: web::Path<crate::models::StaffId>,
    mut payload: Multipart,
) -> Result<Json<StaffResponse>, APIError> {
    let staff_id_inner = staff_id.into_inner().0;
    let mut conn = data.db_pool.get()?;

    let _existing_staff: DbStaff = staff::table
        .find(&staff_id_inner)
        .select(DbStaff::as_select())
        .first(&mut conn)?;

    // Create uploads directory if it doesn't exist
    create_dir_all("./uploads")?;

    let mut file_path = None;

    while let Some(mut field) = payload.try_next().await? {
        if let Some(content_disposition) = field.content_disposition() {
            if let Some(filename) = content_disposition.get_filename() {
                let sanitized_filename = sanitize_filename::sanitize(filename);
                let filepath = format!("./uploads/{}", sanitized_filename);
                let filepath_clone = filepath.clone();
                let mut f = web::block(move || std::fs::File::create(&filepath_clone)).await??;
                while let Some(chunk) = field.next().await {
                    let data = chunk?;
                    f = web::block(move || f.write_all(&data).map(|_| f)).await??;
                }
                file_path = Some(filepath);
                break; // Process only the first file
            }
        }
    }

    if let Some(filepath) = file_path {
        use crate::schema::staff_media;
        diesel::insert_into(staff_media::table)
            .values((
                staff_media::staff_id.eq(&staff_id_inner),
                staff_media::photo_url.eq(Some(filepath.clone())),
                staff_media::created_at.eq(Utc::now().naive_utc()),
                staff_media::updated_at.eq(Utc::now().naive_utc()),
            ))
            .on_conflict(staff_media::staff_id)
            .do_update()
            .set((
                staff_media::photo_url.eq(Some(filepath.clone())),
                staff_media::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut conn)?;

        // Fetch updated staff, profile, and user info to construct StaffResponse
        let (updated_staff, profile, user_profile, media, identity, employment_status, contact, reward): (
            DbStaff,
            Profile,
            Option<User>,
            Option<crate::database::tables::StaffMedia>,
            Option<crate::database::tables::StaffIdentity>,
            Option<crate::database::tables::StaffEmploymentStatus>,
            Option<crate::database::tables::StaffContact>,
            Option<crate::database::tables::StaffRewardSnapshot>,
        ) = staff::table
            .find(&staff_id_inner)
            .inner_join(profiles::table)
            .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
            .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
            .left_join(staff_media::table.on(staff::id.eq(staff_media::staff_id)))
            .left_join(staff_identity::table.on(staff::id.eq(staff_identity::staff_id)))
            .left_join(
                staff_employment_status::table.on(
                    staff::id.eq(staff_employment_status::staff_id),
                ),
            )
            .left_join(staff_contacts::table.on(staff::id.eq(staff_contacts::staff_id)))
            .left_join(
                staff_reward_snapshots::table.on(staff::id.eq(staff_reward_snapshots::staff_id)),
            )
            .select((
                DbStaff::as_select(),
                Profile::as_select(),
                Option::<User>::as_select(),
                Option::<crate::database::tables::StaffMedia>::as_select(),
                Option::<crate::database::tables::StaffIdentity>::as_select(),
                Option::<crate::database::tables::StaffEmploymentStatus>::as_select(),
                Option::<crate::database::tables::StaffContact>::as_select(),
                Option::<crate::database::tables::StaffRewardSnapshot>::as_select(),
            ))
            .first(&mut conn)?;

        Ok(Json(StaffResponse {
            id: updated_staff.id,
            employee_id: updated_staff.employee_id,
            name: profile.name.clone(),
            address: contact.as_ref().map(|c| c.address.clone()),
            phone: contact.as_ref().map(|c| c.phone.clone()),
            email: contact.as_ref().map(|c| c.email.clone()),
            photo_url: media.as_ref().and_then(|m| m.photo_url.clone()),
            nic: identity.as_ref().map(|i| i.nic.clone()),
            dob: updated_staff.dob,
            gender: updated_staff.gender,
            employment_status: employment_status.map(|e| e.employment_status),
            staff_type: updated_staff.staff_type,
            created_at: updated_staff.created_at,
            updated_at: updated_staff.updated_at,
            profile_id: updated_staff.profile_id,
            profile_name: Some(profile.name),
            profile_address: None,
            profile_phone: None,
            profile_photo_url: None,
            user_email: user_profile.map(|u| u.email),
            reward_points_balance: reward.map(|r| r.reward_points_balance),
        }))
    } else {
        Err(APIError::bad_request("No file was uploaded"))
    }
}

#[api_operation(
    summary = "Get all staff members",
    description = "Returns a list of all staff members with pagination, search, and filtering.",
    tag = "staff",
    operation_id = "get_all_staff"
)]
pub async fn get_all_staff(
    data: web::Data<AppState>,
    query: web::Query<StaffQuery>,
) -> Result<Json<PaginatedStaffResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    use crate::schema::{profiles, user_profiles, users};

    let mut base_query = staff::table
        .inner_join(profiles::table.on(staff::profile_id.eq(profiles::id.nullable())))
        .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
        .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
        .left_join(staff_identity::table.on(staff::id.eq(staff_identity::staff_id)))
        .left_join(staff_contacts::table.on(staff::id.eq(staff_contacts::staff_id)))
        .left_join(staff_employment_status::table.on(staff::id.eq(staff_employment_status::staff_id)))
        .left_join(staff_media::table.on(staff::id.eq(staff_media::staff_id)))
        .left_join(staff_reward_snapshots::table.on(staff::id.eq(staff_reward_snapshots::staff_id)))
        .into_boxed();

    let mut count_query_base = staff::table
        .inner_join(profiles::table.on(staff::profile_id.eq(profiles::id.nullable())))
        .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
        .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
        .left_join(staff_identity::table.on(staff::id.eq(staff_identity::staff_id)))
        .left_join(staff_contacts::table.on(staff::id.eq(staff_contacts::staff_id)))
        .left_join(staff_employment_status::table.on(staff::id.eq(staff_employment_status::staff_id)))
        .left_join(staff_media::table.on(staff::id.eq(staff_media::staff_id)))
        .left_join(staff_reward_snapshots::table.on(staff::id.eq(staff_reward_snapshots::staff_id)))
        .into_boxed();

    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        base_query = base_query.filter(
            profiles::name
                .like(pattern.clone())
                .or(staff::employee_id.like(pattern.clone()))
                .or(users::email.like(pattern.clone()))
                .or(staff_identity::nic.like(pattern.clone()))
                .or(staff_contacts::phone.like(pattern.clone()))
                .or(staff_contacts::address.like(pattern.clone())),
        );
        count_query_base = count_query_base.filter(
            profiles::name
                .like(pattern.clone())
                .or(staff::employee_id.like(pattern.clone()))
                .or(users::email.like(pattern.clone()))
                .or(staff_identity::nic.like(pattern.clone()))
                .or(staff_contacts::phone.like(pattern.clone()))
                .or(staff_contacts::address.like(pattern.clone())),
        );
    }

    if let Some(employment_status) = &query.employment_status {
        base_query =
            base_query.filter(staff_employment_status::employment_status.eq(employment_status.clone()));
        count_query_base = count_query_base
            .filter(staff_employment_status::employment_status.eq(employment_status.clone()));
    }

    if let Some(staff_type) = &query.staff_type {
        base_query = base_query.filter(staff::staff_type.eq(staff_type.clone()));
        count_query_base = count_query_base.filter(staff::staff_type.eq(staff_type.clone()));
    }

    if let Some(after_str) = &query.created_after {
        if let Ok(after) =
            NaiveDateTime::parse_from_str(&format!("{} 00:00:00", after_str), "%Y-%m-%d %H:%M:%S")
        {
            base_query = base_query.filter(staff::created_at.ge(after));
            count_query_base = count_query_base.filter(staff::created_at.ge(after));
        }
    }
    if let Some(before_str) = &query.created_before {
        if let Ok(before) =
            NaiveDateTime::parse_from_str(&format!("{} 23:59:59", before_str), "%Y-%m-%d %H:%M:%S")
        {
            base_query = base_query.filter(staff::created_at.le(before));
            count_query_base = count_query_base.filter(staff::created_at.le(before));
        }
    }

    let total_staff_count = count_query_base
        .select(diesel::dsl::count(staff::id))
        .get_result::<i64>(&mut conn)?;

    let limit = query.limit.unwrap_or(10);
    if let Some(last_id) = &query.last_id {
        base_query = base_query.filter(staff::id.gt(last_id));
    }

    let sort_col = query.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");

    base_query = match (sort_col, sort_order) {
        ("profile_name", "asc") => base_query.order(profiles::name.asc()),
        ("profile_name", "desc") => base_query.order(profiles::name.desc()),
        ("employee_id", "asc") => base_query.order(staff::employee_id.asc()),
        ("employee_id", "desc") => base_query.order(staff::employee_id.desc()),
        ("user_email", "asc") => base_query.order(users::email.asc()),
        ("user_email", "desc") => base_query.order(users::email.desc()),
        ("created_at", "asc") => base_query.order(staff::created_at.asc()),
        _ => base_query.order(staff::created_at.desc()),
    };

    let staff_list_data: Vec<(
        DbStaff,
        Profile,
        Option<User>,
        Option<crate::database::tables::StaffMedia>,
        Option<crate::database::tables::StaffIdentity>,
        Option<crate::database::tables::StaffEmploymentStatus>,
        Option<crate::database::tables::StaffContact>,
        Option<crate::database::tables::StaffRewardSnapshot>,
    )> = base_query
        .select((
            DbStaff::as_select(),
            Profile::as_select(),
            Option::<User>::as_select(),
            Option::<crate::database::tables::StaffMedia>::as_select(),
            Option::<crate::database::tables::StaffIdentity>::as_select(),
            Option::<crate::database::tables::StaffEmploymentStatus>::as_select(),
            Option::<crate::database::tables::StaffContact>::as_select(),
            Option::<crate::database::tables::StaffRewardSnapshot>::as_select(),
        ))
        .limit(limit)
        .load::<(
            DbStaff,
            Profile,
            Option<User>,
            Option<crate::database::tables::StaffMedia>,
            Option<crate::database::tables::StaffIdentity>,
            Option<crate::database::tables::StaffEmploymentStatus>,
            Option<crate::database::tables::StaffContact>,
            Option<crate::database::tables::StaffRewardSnapshot>,
        )>(&mut conn)?;

    let staff_responses: Vec<StaffResponse> = staff_list_data
        .into_iter()
        .map(|(staff, profile, user, media, identity, employment_status, contact, reward)| StaffResponse {
            id: staff.id,
            employee_id: staff.employee_id,
            name: profile.name.clone(),
            address: contact.as_ref().map(|c| c.address.clone()),
            phone: contact.as_ref().map(|c| c.phone.clone()),
            email: contact.as_ref().map(|c| c.email.clone()),
            photo_url: media.as_ref().and_then(|m| m.photo_url.clone()),
            nic: identity.as_ref().map(|i| i.nic.clone()),
            dob: staff.dob,
            gender: staff.gender,
            employment_status: employment_status.map(|e| e.employment_status),
            staff_type: staff.staff_type,
            created_at: staff.created_at,
            updated_at: staff.updated_at,
            profile_id: staff.profile_id,
            profile_name: Some(profile.name),
            profile_address: None,
            profile_phone: None,
            profile_photo_url: None,
            user_email: user.map(|u| u.email),
            reward_points_balance: reward.map(|r| r.reward_points_balance),
        })
        .collect();

    let total_pages = (total_staff_count as f64 / limit as f64).ceil() as i64;

    Ok(Json(PaginatedStaffResponse {
        total: total_staff_count,
        page: query.page.unwrap_or(1),
        limit,
        total_pages,
        next_last_id: staff_responses.last().map(|item| item.id.clone()),
        data: staff_responses,
    }))
}

#[api_operation(
    summary = "Get staff member by ID",
    description = "Returns a single staff member by ID.",
    tag = "staff",
    operation_id = "get_staff_by_id"
)]
pub async fn get_staff_by_id(
    data: web::Data<AppState>,
    staff_id: web::Path<crate::models::StaffId>,
) -> Result<Json<StaffResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let staff_id_inner = staff_id.into_inner().0;

    use crate::schema::{profiles, user_profiles, users};

    let (staff_member, profile, user_profile, media, identity, employment_status, contact, reward): (
        DbStaff,
        Profile,
        Option<User>,
        Option<crate::database::tables::StaffMedia>,
        Option<crate::database::tables::StaffIdentity>,
        Option<crate::database::tables::StaffEmploymentStatus>,
        Option<crate::database::tables::StaffContact>,
        Option<crate::database::tables::StaffRewardSnapshot>,
    ) = staff::table
        .find(&staff_id_inner)
        .inner_join(profiles::table)
        .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
        .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
        .left_join(staff_media::table.on(staff::id.eq(staff_media::staff_id)))
        .left_join(staff_identity::table.on(staff::id.eq(staff_identity::staff_id)))
        .left_join(
            staff_employment_status::table.on(staff::id.eq(staff_employment_status::staff_id)),
        )
        .left_join(staff_contacts::table.on(staff::id.eq(staff_contacts::staff_id)))
        .left_join(staff_reward_snapshots::table.on(staff::id.eq(staff_reward_snapshots::staff_id)))
        .select((
            DbStaff::as_select(),
            Profile::as_select(),
            Option::<User>::as_select(),
            Option::<crate::database::tables::StaffMedia>::as_select(),
            Option::<crate::database::tables::StaffIdentity>::as_select(),
            Option::<crate::database::tables::StaffEmploymentStatus>::as_select(),
            Option::<crate::database::tables::StaffContact>::as_select(),
            Option::<crate::database::tables::StaffRewardSnapshot>::as_select(),
        ))
        .first(&mut conn)?;

    Ok(Json(StaffResponse {
        id: staff_member.id,
        employee_id: staff_member.employee_id,
        name: profile.name.clone(),
        address: contact.as_ref().map(|c| c.address.clone()),
        phone: contact.as_ref().map(|c| c.phone.clone()),
        email: contact.as_ref().map(|c| c.email.clone()),
        photo_url: media.as_ref().and_then(|m| m.photo_url.clone()),
        nic: identity.as_ref().map(|i| i.nic.clone()),
        dob: staff_member.dob,
        gender: staff_member.gender,
        employment_status: employment_status.map(|e| e.employment_status),
        staff_type: staff_member.staff_type,
        created_at: staff_member.created_at,
        updated_at: staff_member.updated_at,
        profile_id: staff_member.profile_id,
        profile_name: Some(profile.name),
        profile_address: None,
        profile_phone: None,
        profile_photo_url: None,
        user_email: user_profile.map(|u| u.email),
        reward_points_balance: reward.map(|r| r.reward_points_balance),
    }))
}

#[api_operation(
    summary = "Create a new staff member",
    description = "Registers a new staff member in the system.",
    tag = "staff",
    operation_id = "create_staff"
)]
pub async fn create_staff(
    data: web::Data<AppState>,
    body: web::Json<CreateStaffRequest>,
) -> Result<Json<StaffResponse>, APIError> {
    if !is_valid_email(&body.email) {
        return Err(APIError::bad_request("Invalid email format"));
    }
    if !is_valid_nic(&body.nic) {
        return Err(APIError::bad_request("Invalid NIC format"));
    }
    if !is_valid_phone(&body.phone) {
        return Err(APIError::bad_request("Invalid phone number format"));
    }

    let mut conn = data.db_pool.get()?;

    // Check for existing employee_id
    let existing_staff: Option<DbStaff> = staff::table
        .filter(staff::employee_id.eq(&body.employee_id))
        .select(DbStaff::as_select())
        .first(&mut conn)
        .optional()?;

    if existing_staff.is_some() {
        return Err(APIError::conflict(
            "Staff with this employee ID already exists",
        ));
    }

    // Check if an existing user with this email is already linked to a profile
    let existing_user_profile: Option<UserProfile> = users::table
        .inner_join(user_profiles::table)
        .filter(users::email.eq(&body.email))
        .select(UserProfile::as_select())
        .first(&mut conn)
        .optional()?;

    if existing_user_profile.is_some() {
        return Err(APIError::conflict(
            "An existing user with this email is already linked to a profile.",
        ));
    }

    let new_staff_id = generate_prefixed_id(&mut conn, IdPrefix::STAFF)?;

    // Create a new Profile record for the staff member
    let new_profile_id = generate_prefixed_id(&mut conn, IdPrefix::PROFILE)?;
    let new_profile = NewProfile {
        id: new_profile_id.clone(),
        name: body.name.clone(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    diesel::insert_into(profiles::table)
        .values(&new_profile)
        .execute(&mut conn)?;

    let new_staff_record = DbStaff {
        id: new_staff_id.clone(),
        employee_id: body.employee_id.clone(),
        name: body.name.clone(),
        dob: body.dob,
        gender: body.gender.clone(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        staff_type: body.staff_type.clone(),
        profile_id: Some(new_profile_id.clone()),
    };

    diesel::insert_into(staff::table)
        .values(&new_staff_record)
        .execute(&mut conn)?;

    diesel::insert_into(staff_identity::table)
        .values((
            staff_identity::staff_id.eq(&new_staff_id),
            staff_identity::nic.eq(body.nic.clone()),
            staff_identity::created_at.eq(Utc::now().naive_utc()),
            staff_identity::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    diesel::insert_into(staff_contacts::table)
        .values((
            staff_contacts::staff_id.eq(&new_staff_id),
            staff_contacts::address.eq(body.address.clone()),
            staff_contacts::phone.eq(body.phone.clone()),
            staff_contacts::email.eq(body.email.clone()),
            staff_contacts::address_latitude.eq(None::<f32>),
            staff_contacts::address_longitude.eq(None::<f32>),
            staff_contacts::created_at.eq(Utc::now().naive_utc()),
            staff_contacts::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    diesel::insert_into(staff_employment_status::table)
        .values((
            staff_employment_status::staff_id.eq(&new_staff_id),
            staff_employment_status::employment_status.eq(body.employment_status.clone()),
            staff_employment_status::created_at.eq(Utc::now().naive_utc()),
            staff_employment_status::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    if let Some(photo_url) = body.photo_url.clone() {
        diesel::insert_into(staff_media::table)
            .values((
                staff_media::staff_id.eq(&new_staff_id),
                staff_media::photo_url.eq(Some(photo_url)),
                staff_media::created_at.eq(Utc::now().naive_utc()),
                staff_media::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut conn)?;
    }

    diesel::insert_into(staff_reward_snapshots::table)
        .values((
            staff_reward_snapshots::staff_id.eq(&new_staff_id),
            staff_reward_snapshots::reward_points_balance.eq(0),
            staff_reward_snapshots::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    let mut user_email_str: Option<String> = None;

    // Create a UserProfile entry linking the new Profile to an existing User if email matches
    let matching_user: Option<User> = users::table
        .filter(users::email.eq(body.email.clone()))
        .select(User::as_select())
        .first(&mut conn)
        .optional()?;

    if let Some(user) = matching_user {
        let new_user_profile = NewUserProfile {
            user_id: user.id.clone(),
            profile_id: new_profile_id.clone(),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };
        diesel::insert_into(user_profiles::table)
            .values(&new_user_profile)
            .execute(&mut conn)?;
        user_email_str = Some(user.email);
    }

    Ok(Json(StaffResponse {
        id: new_staff_record.id,
        employee_id: new_staff_record.employee_id,
        name: new_staff_record.name,
        address: Some(body.address.clone()),
        phone: Some(body.phone.clone()),
        email: Some(body.email.clone()),
        photo_url: body.photo_url.clone(),
        nic: Some(body.nic.clone()),
        dob: new_staff_record.dob,
        gender: new_staff_record.gender,
        employment_status: Some(body.employment_status.clone()),
        staff_type: new_staff_record.staff_type,
        created_at: new_staff_record.created_at,
        updated_at: new_staff_record.updated_at,
        profile_id: new_staff_record.profile_id,
        profile_name: Some(new_profile.name),
        profile_address: None,
        profile_phone: None,
        profile_photo_url: None,
        user_email: user_email_str,
        reward_points_balance: Some(0),
    }))
}
#[api_operation(
    summary = "Update a staff member",
    description = "Updates an existing staff member's profile.",
    tag = "staff",
    operation_id = "update_staff"
)]
pub async fn update_staff(
    data: web::Data<AppState>,
    staff_id: web::Path<crate::models::StaffId>,
    body: web::Json<UpdateStaffRequest>,
) -> Result<Json<StaffResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let staff_id_inner = staff_id.into_inner().0;

    // Check if the new NIC already exists for another staff member
    if let Some(ref nic) = body.nic {
        let existing_staff: Option<crate::database::tables::StaffIdentity> =
            staff_identity::table
                .filter(staff_identity::nic.eq(nic))
                .filter(staff_identity::staff_id.ne(&staff_id_inner))
                .select(crate::database::tables::StaffIdentity::as_select())
                .first(&mut conn)
                .optional()?;
        if existing_staff.is_some() {
            return Err(APIError::conflict(
                "Another staff member with this NIC already exists",
            ));
        }
    }

    let existing_staff: DbStaff = staff::table
        .find(&staff_id_inner)
        .select(DbStaff::as_select())
        .first(&mut conn)?;

    let profile_id = existing_staff
        .profile_id
        .ok_or_else(|| APIError::not_found("Profile not found for staff member"))?;

    // Update profile name (core profile table)
    use crate::schema::profiles;
    if let Some(ref name) = body.name {
        diesel::update(profiles::table.find(&profile_id))
            .set((
                profiles::name.eq(name),
                profiles::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut conn)?;
    }

    // Update staff core fields
    diesel::update(staff::table.find(&staff_id_inner))
        .set((
            body.name.as_ref().map(|n| staff::name.eq(n)),
            body.dob.map(|dob| staff::dob.eq(dob)),
            body.gender.clone().map(|g| staff::gender.eq(g)),
            body.staff_type.clone().map(|st| staff::staff_type.eq(st)),
            staff::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    // Update staff identity
    if let Some(nic) = &body.nic {
        diesel::insert_into(staff_identity::table)
            .values((
                staff_identity::staff_id.eq(&staff_id_inner),
                staff_identity::nic.eq(nic.clone()),
                staff_identity::created_at.eq(Utc::now().naive_utc()),
                staff_identity::updated_at.eq(Utc::now().naive_utc()),
            ))
            .on_conflict(staff_identity::staff_id)
            .do_update()
            .set((
                staff_identity::nic.eq(nic.clone()),
                staff_identity::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut conn)?;
    }

    // Update staff contacts
    if body.address.is_some() || body.phone.is_some() || body.email.is_some() {
        diesel::insert_into(staff_contacts::table)
            .values((
                staff_contacts::staff_id.eq(&staff_id_inner),
                staff_contacts::address.eq(body.address.clone().unwrap_or_default()),
                staff_contacts::phone.eq(body.phone.clone().unwrap_or_default()),
                staff_contacts::email.eq(body.email.clone().unwrap_or_default()),
                staff_contacts::address_latitude.eq(None::<f32>),
                staff_contacts::address_longitude.eq(None::<f32>),
                staff_contacts::created_at.eq(Utc::now().naive_utc()),
                staff_contacts::updated_at.eq(Utc::now().naive_utc()),
            ))
            .on_conflict(staff_contacts::staff_id)
            .do_update()
            .set((
                body.address
                    .as_ref()
                    .map(|a| staff_contacts::address.eq(a.clone())),
                body.phone
                    .as_ref()
                    .map(|p| staff_contacts::phone.eq(p.clone())),
                body.email
                    .as_ref()
                    .map(|e| staff_contacts::email.eq(e.clone())),
                staff_contacts::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut conn)?;
    }

    // Update employment status
    if let Some(es) = body.employment_status.clone() {
        diesel::insert_into(staff_employment_status::table)
            .values((
                staff_employment_status::staff_id.eq(&staff_id_inner),
                staff_employment_status::employment_status.eq(es.clone()),
                staff_employment_status::created_at.eq(Utc::now().naive_utc()),
                staff_employment_status::updated_at.eq(Utc::now().naive_utc()),
            ))
            .on_conflict(staff_employment_status::staff_id)
            .do_update()
            .set((
                staff_employment_status::employment_status.eq(es),
                staff_employment_status::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut conn)?;
    }

    // Update staff media
    if let Some(photo_url) = &body.photo_url {
        diesel::insert_into(staff_media::table)
            .values((
                staff_media::staff_id.eq(&staff_id_inner),
                staff_media::photo_url.eq(Some(photo_url.clone())),
                staff_media::created_at.eq(Utc::now().naive_utc()),
                staff_media::updated_at.eq(Utc::now().naive_utc()),
            ))
            .on_conflict(staff_media::staff_id)
            .do_update()
            .set((
                staff_media::photo_url.eq(Some(photo_url.clone())),
                staff_media::updated_at.eq(Utc::now().naive_utc()),
            ))
            .execute(&mut conn)?;
    }

    let (final_staff, final_profile, final_user_profile, media, identity, employment_status, contact, reward): (
        DbStaff,
        Profile,
        Option<User>,
        Option<crate::database::tables::StaffMedia>,
        Option<crate::database::tables::StaffIdentity>,
        Option<crate::database::tables::StaffEmploymentStatus>,
        Option<crate::database::tables::StaffContact>,
        Option<crate::database::tables::StaffRewardSnapshot>,
    ) = staff::table
        .find(&staff_id_inner)
        .inner_join(profiles::table)
        .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
        .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
        .left_join(staff_media::table.on(staff::id.eq(staff_media::staff_id)))
        .left_join(staff_identity::table.on(staff::id.eq(staff_identity::staff_id)))
        .left_join(
            staff_employment_status::table.on(staff::id.eq(staff_employment_status::staff_id)),
        )
        .left_join(staff_contacts::table.on(staff::id.eq(staff_contacts::staff_id)))
        .left_join(staff_reward_snapshots::table.on(staff::id.eq(staff_reward_snapshots::staff_id)))
        .select((
            DbStaff::as_select(),
            Profile::as_select(),
            Option::<User>::as_select(),
            Option::<crate::database::tables::StaffMedia>::as_select(),
            Option::<crate::database::tables::StaffIdentity>::as_select(),
            Option::<crate::database::tables::StaffEmploymentStatus>::as_select(),
            Option::<crate::database::tables::StaffContact>::as_select(),
            Option::<crate::database::tables::StaffRewardSnapshot>::as_select(),
        ))
        .first(&mut conn)?;

    Ok(Json(StaffResponse {
        id: final_staff.id,
        employee_id: final_staff.employee_id,
        name: final_profile.name.clone(),
        address: contact.as_ref().map(|c| c.address.clone()),
        phone: contact.as_ref().map(|c| c.phone.clone()),
        email: contact.as_ref().map(|c| c.email.clone()),
        photo_url: media.as_ref().and_then(|m| m.photo_url.clone()),
        nic: identity.as_ref().map(|i| i.nic.clone()),
        dob: final_staff.dob,
        gender: final_staff.gender,
        employment_status: employment_status.map(|e| e.employment_status),
        staff_type: final_staff.staff_type,
        created_at: final_staff.created_at,
        updated_at: final_staff.updated_at,
        profile_id: final_staff.profile_id,
        profile_name: Some(final_profile.name),
        profile_address: None,
        profile_phone: None,
        profile_photo_url: None,
        user_email: final_user_profile.map(|u| u.email),
        reward_points_balance: reward.map(|r| r.reward_points_balance),
    }))
}

#[api_operation(
    summary = "Delete a staff member",
    description = "Deletes a staff member by ID.",
    tag = "staff",
    operation_id = "delete_staff"
)]
pub async fn delete_staff(
    data: web::Data<AppState>,
    staff_id: web::Path<crate::models::StaffId>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    diesel::delete(staff::table.find(staff_id.into_inner().0)).execute(&mut conn)?;
    Ok(Json(MessageResponse {
        message: "Staff member deleted successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Bulk delete staff members",
    description = "Deletes multiple staff members by their IDs.",
    tag = "staff",
    operation_id = "bulk_delete_staff"
)]
pub async fn bulk_delete_staff(
    data: web::Data<AppState>,
    body: web::Json<BulkDeleteStaffRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    crate::services::staff::staff::bulk_delete_staff(data.clone(), body.into_inner().staff_ids)
        .await?;
    Ok(Json(MessageResponse {
        message: "Staff members deleted successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Bulk update staff members",
    description = "Updates multiple staff members' information.",
    tag = "staff",
    operation_id = "bulk_update_staff"
)]
pub async fn bulk_update_staff(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateStaffRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    crate::services::staff::staff::bulk_update_staff(data.clone(), body.into_inner()).await?;
    Ok(Json(MessageResponse {
        message: "Staff members updated successfully".to_string(),
    }))
}
