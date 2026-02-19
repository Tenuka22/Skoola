use actix_web::web;
use apistos::{api_operation, ApiComponent};
use diesel::prelude::*;
use uuid::Uuid;
use chrono::{Utc, NaiveDateTime};
use actix_multipart::Multipart;
use futures_util::stream::{StreamExt, TryStreamExt};
use std::io::Write;
use std::fs::create_dir_all;
use actix_web::web::Json;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    database::tables::{Staff},
    errors::APIError,
    models::staff::staff::{CreateStaffRequest, StaffChangeset, UpdateStaffRequest, StaffResponse, StaffQuery, PaginatedStaffResponse},
    models::{MessageResponse, Profile, NewProfile, UserProfile, NewUserProfile, auth_user::User}, // Added Profile, NewProfile, UserProfile, NewUserProfile, User
    schema::{staff, profiles, user_profiles, users}, // Added profiles, user_profiles, users
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
    pub gender: Option<String>,
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
    staff_id: web::Path<String>,
    mut payload: Multipart,
) -> Result<Json<StaffResponse>, APIError> {
    let staff_id_inner = staff_id.into_inner();
    let mut conn = data.db_pool.get()?;

    // Check if staff exists and get its profile_id
    let existing_staff: Staff = staff::table
        .find(&staff_id_inner)
        .select(Staff::as_select())
        .first(&mut conn)?;

    let profile_id = existing_staff.profile_id.ok_or_else(|| APIError::not_found("Profile not found for staff member"))?;

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
        use crate::schema::profiles;
        diesel::update(profiles::table.find(&profile_id))
            .set(profiles::photo_url.eq(&filepath))
            .execute(&mut conn)?;

        // Fetch updated staff, profile, and user info to construct StaffResponse
        let (updated_staff, profile, user_profile): (Staff, Profile, Option<User>) = staff::table
            .find(&staff_id_inner)
            .inner_join(profiles::table)
            .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
            .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
            .select((Staff::as_select(), Profile::as_select(), Option::<User>::as_select()))
            .first(&mut conn)?;

        Ok(Json(StaffResponse {
            id: updated_staff.id,
            employee_id: updated_staff.employee_id,
            nic: updated_staff.nic,
            dob: updated_staff.dob,
            gender: updated_staff.gender,
            employment_status: updated_staff.employment_status,
            staff_type: updated_staff.staff_type,
            created_at: updated_staff.created_at,
            updated_at: updated_staff.updated_at,
            profile_id: updated_staff.profile_id,
            profile_name: Some(profile.name),
            profile_address: profile.address,
            profile_phone: profile.phone,
            profile_photo_url: profile.photo_url,
            user_email: user_profile.map(|u| u.email),
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
        .into_boxed();

    let mut count_query_base = staff::table
        .inner_join(profiles::table.on(staff::profile_id.eq(profiles::id.nullable())))
        .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
        .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
        .into_boxed();


    if let Some(search_term) = &query.search {
        let pattern = format!("%{}%", search_term);
        base_query = base_query.filter(
            profiles::name.like(pattern.clone())
                .or(staff::employee_id.like(pattern.clone()))
                .or(staff::nic.like(pattern.clone()))
                .or(users::email.like(pattern.clone()))
                .or(profiles::phone.like(pattern.clone()))
                .or(profiles::address.like(pattern.clone()))
        );
        count_query_base = count_query_base.filter(
            profiles::name.like(pattern.clone())
                .or(staff::employee_id.like(pattern.clone()))
                .or(staff::nic.like(pattern.clone()))
                .or(users::email.like(pattern.clone()))
                .or(profiles::phone.like(pattern.clone()))
                .or(profiles::address.like(pattern.clone()))
        );
    }

    if let Some(employment_status) = &query.employment_status {
        base_query = base_query.filter(staff::employment_status.eq(employment_status.clone()));
        count_query_base = count_query_base.filter(staff::employment_status.eq(employment_status.clone()));
    }

    if let Some(staff_type) = &query.staff_type {
        base_query = base_query.filter(staff::staff_type.eq(staff_type.clone()));
        count_query_base = count_query_base.filter(staff::staff_type.eq(staff_type.clone()));
    }

    if let Some(after_str) = &query.created_after {
        if let Ok(after) = NaiveDateTime::parse_from_str(&format!("{} 00:00:00", after_str), "%Y-%m-%d %H:%M:%S") {
            base_query = base_query.filter(staff::created_at.ge(after));
            count_query_base = count_query_base.filter(staff::created_at.ge(after));
        }
    }
    if let Some(before_str) = &query.created_before {
        if let Ok(before) = NaiveDateTime::parse_from_str(&format!("{} 23:59:59", before_str), "%Y-%m-%d %H:%M:%S") {
            base_query = base_query.filter(staff::created_at.le(before));
            count_query_base = count_query_base.filter(staff::created_at.le(before));
        }
    }

    let total_staff_count = count_query_base
        .select(diesel::dsl::count(staff::id))
        .get_result::<i64>(&mut conn)?;

    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

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

    let staff_list_data: Vec<(Staff, Profile, Option<User>)> = base_query
        .select((Staff::as_select(), Profile::as_select(), Option::<User>::as_select()))
        .limit(limit)
        .offset(offset)
        .load::<(Staff, Profile, Option<User>)>(&mut conn)?;

    let staff_responses: Vec<StaffResponse> = staff_list_data.into_iter().map(|(staff, profile, user)| {
        StaffResponse {
            id: staff.id,
            employee_id: staff.employee_id,
            nic: staff.nic,
            dob: staff.dob,
            gender: staff.gender,
            employment_status: staff.employment_status,
            staff_type: staff.staff_type,
            created_at: staff.created_at,
            updated_at: staff.updated_at,
            profile_id: staff.profile_id,
            profile_name: Some(profile.name),
            profile_address: profile.address,
            profile_phone: profile.phone,
            profile_photo_url: profile.photo_url,
            user_email: user.map(|u| u.email),
        }
    }).collect();

    let total_pages = (total_staff_count as f64 / limit as f64).ceil() as i64;

    Ok(Json(PaginatedStaffResponse {
        total: total_staff_count,
        page,
        limit,
        total_pages,
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
    staff_id: web::Path<String>,
) -> Result<Json<StaffResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let staff_id_inner = staff_id.into_inner();

    use crate::schema::{profiles, user_profiles, users};

    let (staff_member, profile, user_profile): (Staff, Profile, Option<User>) = staff::table
        .find(&staff_id_inner)
        .inner_join(profiles::table)
        .left_join(user_profiles::table.on(profiles::id.eq(user_profiles::profile_id)))
        .left_join(users::table.on(user_profiles::user_id.eq(users::id)))
        .select((Staff::as_select(), Profile::as_select(), Option::<User>::as_select()))
        .first(&mut conn)?;

    Ok(Json(StaffResponse {
        id: staff_member.id,
        employee_id: staff_member.employee_id,
        nic: staff_member.nic,
        dob: staff_member.dob,
        gender: staff_member.gender,
        employment_status: staff_member.employment_status,
        staff_type: staff_member.staff_type,
        created_at: staff_member.created_at,
        updated_at: staff_member.updated_at,
        profile_id: staff_member.profile_id,
        profile_name: Some(profile.name),
        profile_address: profile.address,
        profile_phone: profile.phone,
        profile_photo_url: profile.photo_url,
        user_email: user_profile.map(|u| u.email),
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
    let existing_staff: Option<Staff> = staff::table
        .filter(staff::employee_id.eq(&body.employee_id))
        .select(Staff::as_select())
        .first(&mut conn)
        .optional()?;

    if existing_staff.is_some() {
        return Err(APIError::conflict("Staff with this employee ID already exists"));
    }

    // Check if an existing user with this email is already linked to a profile
    let existing_user_profile: Option<UserProfile> = users::table
        .inner_join(user_profiles::table)
        .filter(users::email.eq(&body.email))
        .select(UserProfile::as_select())
        .first(&mut conn)
        .optional()?;

    if existing_user_profile.is_some() {
        return Err(APIError::conflict("An existing user with this email is already linked to a profile."));
    }

    let new_staff_id = Uuid::new_v4().to_string(); // Generate staff ID here

    // Create a new Profile record for the staff member
    let new_profile_id = Uuid::new_v4().to_string();
    let new_profile = NewProfile {
        id: new_profile_id.clone(),
        name: body.name.clone(),
        address: Some(body.address.clone()),
        phone: Some(body.phone.clone()),
        photo_url: None,
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
    };
    diesel::insert_into(profiles::table)
        .values(&new_profile)
        .execute(&mut conn)?;

    let new_staff_record = Staff {
        id: new_staff_id.clone(),
        employee_id: body.employee_id.clone(),
        nic: body.nic.clone(),
        dob: body.dob,
        gender: body.gender.clone(),
        employment_status: body.employment_status.clone(),
        staff_type: body.staff_type.clone(),
        created_at: Utc::now().naive_utc(),
        updated_at: Utc::now().naive_utc(),
        profile_id: Some(new_profile_id.clone()),
    };

    diesel::insert_into(staff::table)
        .values(&new_staff_record)
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
        nic: new_staff_record.nic,
        dob: new_staff_record.dob,
        gender: new_staff_record.gender,
        employment_status: new_staff_record.employment_status,
        staff_type: new_staff_record.staff_type,
        created_at: new_staff_record.created_at,
        updated_at: new_staff_record.updated_at,
        profile_id: new_staff_record.profile_id,
        profile_name: Some(new_profile.name),
        profile_address: new_profile.address,
        profile_phone: new_profile.phone,
        profile_photo_url: new_profile.photo_url,
        user_email: user_email_str,
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
    staff_id: web::Path<String>,
    body: web::Json<UpdateStaffRequest>,
) -> Result<Json<StaffResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let staff_id_inner = staff_id.into_inner();

    // Check if the new NIC already exists for another staff member
    if let Some(ref nic) = body.nic {
        let existing_staff: Option<Staff> = staff::table
            .filter(staff::nic.eq(nic))
            .filter(staff::id.ne(&staff_id_inner))
            .select(Staff::as_select())
            .first(&mut conn)
            .optional()?;
        if existing_staff.is_some() {
            return Err(APIError::conflict("Another staff member with this NIC already exists"));
        }
    }

    let existing_staff: Staff = staff::table
        .find(&staff_id_inner)
        .select(Staff::as_select())
        .first(&mut conn)?;

    let profile_id = existing_staff.profile_id.ok_or_else(|| APIError::not_found("Profile not found for staff member"))?;
    
    // Update profile-specific fields in the profiles table
    use crate::schema::profiles;
    diesel::update(profiles::table.find(&profile_id))
        .set((
            body.name.map(|n| profiles::name.eq(n)),
            body.address.map(|a| profiles::address.eq(a)),
            body.phone.map(|p| profiles::phone.eq(p)),
            profiles::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    // Update staff-specific fields in the staff table
    diesel::update(staff::table.find(&staff_id_inner))
        .set((
            body.nic.map(|nic| staff::nic.eq(nic)),
            body.dob.map(|dob| staff::dob.eq(dob)),
            body.gender.map(|g| staff::gender.eq(g)),
            staff::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;

    let updated_staff = staff::table
        .find(&staff_id_inner)
        .select(Staff::as_select())
        .first::<Staff>(&mut conn)?;


    Ok(Json(StaffResponse::from(updated_staff)))
}

#[api_operation(
    summary = "Delete a staff member",
    description = "Deletes a staff member by ID.",
    tag = "staff",
    operation_id = "delete_staff"
)]
pub async fn delete_staff(
    data: web::Data<AppState>,
    staff_id: web::Path<String>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    diesel::delete(staff::table.find(staff_id.into_inner()))
        .execute(&mut conn)?;
    Ok(Json(MessageResponse { message: "Staff member deleted successfully".to_string() }))
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
    crate::services::staff::staff::bulk_delete_staff(data.clone(), body.into_inner().staff_ids).await?;
    Ok(Json(MessageResponse { message: "Staff members deleted successfully".to_string() }))
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
    Ok(Json(MessageResponse { message: "Staff members updated successfully".to_string() }))
}
