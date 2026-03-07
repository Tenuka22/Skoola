use actix_web::web::Json;
use actix_web::{http::StatusCode, web};
use apistos::{ApiComponent, api_operation};
use chrono::{Duration, NaiveDate, NaiveDateTime, Utc};
use diesel::prelude::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{error, info};

use crate::{
    AppState, database::enums::RoleEnum, database::tables::User, errors::APIError,
    models::MessageResponse, models::auth::user::UserResponse,
    models::system::BulkDeleteUsersRequest, schema::{users, user_security, user_status},
    services::system::cleanup::bulk_delete_users as service_bulk_delete_users,
    utils::serde_helpers::deserialize_option_option,
};

#[derive(Debug, Deserialize, JsonSchema, ApiComponent)]
pub struct UserQuery {
    pub search: Option<String>,
    pub is_verified: Option<bool>,
    pub auth_method: Option<String>,
    pub created_after: Option<String>,
    pub created_before: Option<String>,
    pub sort_by: Option<String>,
    pub sort_order: Option<String>,
    pub page: Option<i64>,
    pub limit: Option<i64>,
    pub last_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct PaginatedUserResponse {
    pub data: Vec<UserResponse>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub next_last_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct BulkUpdateRequest {
    pub user_ids: Vec<String>,
    pub is_verified: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_option_option")]
    pub lockout_until: Option<Option<NaiveDateTime>>,
    pub role: Option<RoleEnum>,
}

#[derive(Debug, Deserialize, Serialize, JsonSchema, ApiComponent)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub is_verified: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_option_option")]
    pub lockout_until: Option<Option<NaiveDateTime>>,
    pub role: Option<RoleEnum>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct UserStatsResponse {
    pub total_users: i64,
    pub verified_users: i64,
    pub pending_users: i64,
    pub locked_users: i64,
    pub auth_methods: AuthMethodStats,
    pub registration_trend: Vec<TrendPoint>,
    pub top_domains: Vec<DomainStat>,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct AuthMethodStats {
    pub google: i64,
    pub github: i64,
    pub password_only: i64,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct TrendPoint {
    pub date: String,
    pub count: i64,
}

#[derive(Debug, Serialize, Deserialize, ApiComponent, JsonSchema)]
pub struct DomainStat {
    pub domain: String,
    pub count: i64,
}

#[api_operation(
    summary = "Get all users",
    description = "Returns a list of all users with pagination, fuzzy search, filtering and sorting.",
    tag = "users",
    operation_id = "get_all_users"
)]
pub async fn get_all_users(
    data: web::Data<AppState>,
    query: web::Query<UserQuery>,
) -> Result<Json<PaginatedUserResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

    let mut data_query = users::table
        .left_join(user_security::table.on(users::id.eq(user_security::user_id)))
        .left_join(user_status::table.on(users::id.eq(user_status::user_id)))
        .into_boxed();
    let mut count_query = users::table
        .left_join(user_security::table.on(users::id.eq(user_security::user_id)))
        .left_join(user_status::table.on(users::id.eq(user_status::user_id)))
        .into_boxed();

    if let Some(search_term) = &query.search {
        let term = search_term.trim();
        if !term.is_empty() {
            for word in term.split_whitespace() {
                let pattern = format!("%{}%", word);
                let filter_expression = users::email
                    .like(pattern.clone())
                    .or(users::id.like(pattern.clone()))
                    .or(users::role.like(pattern));
                data_query = data_query.filter(filter_expression.clone());
                count_query = count_query.filter(filter_expression);
            }
        }
    }

    if let Some(verified) = query.is_verified {
        data_query = data_query.filter(user_status::is_verified.eq(verified));
        count_query = count_query.filter(user_status::is_verified.eq(verified));
    }

    if let Some(method) = &query.auth_method {
        match method.as_str() {
            "google" => {
                data_query = data_query.filter(user_security::google_id.is_not_null());
                count_query = count_query.filter(user_security::google_id.is_not_null());
            }
            "github" => {
                data_query = data_query.filter(user_security::github_id.is_not_null());
                count_query = count_query.filter(user_security::github_id.is_not_null());
            }
            "password" => {
                let filter_expression =
                    user_security::google_id
                        .is_null()
                        .and(user_security::github_id.is_null());
                data_query = data_query.filter(filter_expression.clone());
                count_query = count_query.filter(filter_expression);
            }
            _ => {}
        }
    }

    if let Some(after_str) = &query.created_after {
        if let Ok(after) =
            NaiveDateTime::parse_from_str(&format!("{} 00:00:00", after_str), "%Y-%m-%d %H:%M:%S")
        {
            data_query = data_query.filter(users::created_at.ge(after));
            count_query = count_query.filter(users::created_at.ge(after));
        }
    }
    if let Some(before_str) = &query.created_before {
        if let Ok(before) =
            NaiveDateTime::parse_from_str(&format!("{} 23:59:59", before_str), "%Y-%m-%d %H:%M:%S")
        {
            data_query = data_query.filter(users::created_at.le(before));
            count_query = count_query.filter(users::created_at.le(before));
        }
    }

    if let Some(last_id) = &query.last_id {
        data_query = data_query.filter(users::id.gt(last_id));
    }

    let total_users: i64 = count_query.count().get_result(&mut conn)?;

    let sort_col = query.sort_by.as_deref().unwrap_or("created_at");
    let sort_order = query.sort_order.as_deref().unwrap_or("desc");

    data_query = match (sort_col, sort_order) {
        ("email", "asc") => data_query.order(users::email.asc()),
        ("email", "desc") => data_query.order(users::email.desc()),
        ("is_verified", "asc") => data_query.order(user_status::is_verified.asc()),
        ("is_verified", "desc") => data_query.order(user_status::is_verified.desc()),
        ("created_at", "asc") => data_query.order(users::created_at.asc()),
        _ => data_query.order(users::created_at.desc()),
    };

    let limit = query.limit.unwrap_or(10);

    let user_list = data_query
        .select((
            User::as_select(),
            Option::<crate::database::tables::UserSecurity>::as_select(),
            Option::<crate::database::tables::UserStatus>::as_select(),
        ))
        .limit(limit)
        .load::<(
            User,
            Option<crate::database::tables::UserSecurity>,
            Option<crate::database::tables::UserStatus>,
        )>(&mut conn)?;

    let total_pages = (total_users as f64 / limit as f64).ceil() as i64;
    let next_last_id = user_list
        .last()
        .map(|(u, _, _)| u.id.clone());

    Ok(Json(PaginatedUserResponse {
        data: user_list
            .into_iter()
            .map(|(user, security, status)| UserResponse {
                id: user.id,
                email: user.email,
                is_verified: status.as_ref().map(|s| s.is_verified),
                role: user.role,
                auth_method: if security.as_ref().and_then(|s| s.google_id.clone()).is_some() {
                    "Google".to_string()
                } else if security.as_ref().and_then(|s| s.github_id.clone()).is_some() {
                    "GitHub".to_string()
                } else {
                    "Password".to_string()
                },
                lockout_until: security.as_ref().and_then(|s| s.lockout_until),
                created_at: user.created_at,
                updated_at: user.updated_at,
            })
            .collect(),
        total: total_users,
        page: query.page.unwrap_or(1),
        limit,
        total_pages,
        next_last_id,
    }))
}

#[api_operation(
    summary = "Get user statistics",
    description = "Returns global statistics about users.",
    tag = "users",
    operation_id = "get_user_statistics"
)]
pub async fn get_user_stats(
    data: web::Data<AppState>,
) -> Result<Json<UserStatsResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let all_users = users::table
        .left_join(user_security::table.on(users::id.eq(user_security::user_id)))
        .left_join(user_status::table.on(users::id.eq(user_status::user_id)))
        .select((
            User::as_select(),
            Option::<crate::database::tables::UserSecurity>::as_select(),
            Option::<crate::database::tables::UserStatus>::as_select(),
        ))
        .load::<(
            User,
            Option<crate::database::tables::UserSecurity>,
            Option<crate::database::tables::UserStatus>,
        )>(&mut conn)?;

    let total_users = all_users.len() as i64;
    let mut verified_users = 0;
    let mut locked_users = 0;
    let mut google_auth = 0;
    let mut github_auth = 0;
    let mut password_only = 0;
    let mut domains: HashMap<String, i64> = HashMap::new();
    let mut trend: HashMap<NaiveDate, i64> = HashMap::new();

    let now = Utc::now().naive_utc();
    let thirty_days_ago = now - Duration::days(30);

    for (user, security, status) in &all_users {
        if status.as_ref().map(|s| s.is_verified).unwrap_or(false) {
            verified_users += 1;
        }
        if let Some(lockout) = security.as_ref().and_then(|s| s.lockout_until) {
            if lockout > now {
                locked_users += 1;
            }
        }

        if security.as_ref().and_then(|s| s.google_id.clone()).is_some() {
            google_auth += 1;
        }
        if security.as_ref().and_then(|s| s.github_id.clone()).is_some() {
            github_auth += 1;
        }
        if security.as_ref().and_then(|s| s.google_id.clone()).is_none()
            && security.as_ref().and_then(|s| s.github_id.clone()).is_none()
        {
            password_only += 1;
        }

        let domain = user
            .email
            .split('@')
            .last()
            .unwrap_or("unknown")
            .to_string();
        *domains.entry(domain).or_insert(0) += 1;

        if user.created_at >= thirty_days_ago {
            *trend.entry(user.created_at.date()).or_insert(0) += 1;
        }
    }

    let mut registration_trend: Vec<TrendPoint> = trend
        .into_iter()
        .map(|(date, count)| TrendPoint {
            date: date.to_string(),
            count,
        })
        .collect();
    registration_trend.sort_by(|a, b| a.date.cmp(&b.date));

    let mut top_domains: Vec<DomainStat> = domains
        .into_iter()
        .map(|(domain, count)| DomainStat { domain, count })
        .collect();
    top_domains.sort_by(|a, b| b.count.cmp(&a.count));
    top_domains.truncate(5);

    Ok(Json(UserStatsResponse {
        total_users,
        verified_users,
        pending_users: total_users - verified_users,
        locked_users,
        auth_methods: AuthMethodStats {
            google: google_auth,
            github: github_auth,
            password_only,
        },
        registration_trend,
        top_domains,
    }))
}

#[api_operation(
    summary = "Delete a user",
    description = "Deletes a user by ID.",
    tag = "users",
    operation_id = "delete_user"
)]
pub async fn delete_user(
    data: web::Data<AppState>,
    user_id: web::Path<crate::models::UserId>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let id = user_id.into_inner().0;
    diesel::update(user_status::table.filter(user_status::user_id.eq(&id)))
        .set((
            user_status::is_active.eq(false),
            user_status::disabled_at.eq(Utc::now().naive_utc()),
            user_status::disabled_reason.eq(Some("Deactivated by admin".to_string())),
            user_status::updated_at.eq(Utc::now().naive_utc()),
        ))
        .execute(&mut conn)?;
    Ok(Json(MessageResponse {
        message: "User deactivated successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Bulk delete users",
    description = "Deletes multiple users by their IDs.",
    tag = "users",
    operation_id = "bulk_delete_users"
)]
pub async fn bulk_delete_users(
    data: web::Data<AppState>,
    body: web::Json<BulkDeleteUsersRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    info!("Received request to bulk delete users: {:?}", body.user_ids);

    match service_bulk_delete_users(data, body.into_inner()).await {
        Ok(_) => {
            info!("Successfully bulk deleted users.");
            Ok(Json(MessageResponse {
                message: "Users deleted successfully.".to_string(),
            }))
        }
        Err(e) => {
            error!("Failed to bulk delete users: {:?}", e);
            Err(APIError::new(
                "Internal Server Error",
                &format!("Failed to bulk delete users: {}", e),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

#[api_operation(
    summary = "Update a user",
    description = "Updates user status (verification, lockout, role).",
    tag = "users",
    operation_id = "update_user"
)]
pub async fn update_user(
    data: web::Data<AppState>,
    user_id: web::Path<crate::models::UserId>,
    body: web::Json<UpdateUserRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let id = user_id.into_inner();
    let id = id.0;

    conn.transaction::<_, APIError, _>(|conn| {
        if let Some(e) = &body.email {
            diesel::update(users::table.find(&id))
                .set(users::email.eq(e))
                .execute(conn)?;
        }

        if let Some(v) = body.is_verified {
            diesel::update(user_status::table.filter(user_status::user_id.eq(&id)))
                .set(user_status::is_verified.eq(v))
                .execute(conn)?;
        }

        if let Some(lockout) = body.lockout_until {
            diesel::update(user_security::table.filter(user_security::user_id.eq(&id)))
                .set(user_security::lockout_until.eq(lockout))
                .execute(conn)?;
        }

        if let Some(role) = &body.role {
            diesel::update(users::table.find(&id))
                .set(users::role.eq(role.to_string()))
                .execute(conn)?;
        }
        Ok(())
    })?;

    Ok(Json(MessageResponse {
        message: "User updated successfully".to_string(),
    }))
}

#[api_operation(
    summary = "Bulk update users",
    description = "Updates multiple users' status.",
    tag = "users",
    operation_id = "bulk_update_users"
)]
pub async fn bulk_update_users(
    data: web::Data<AppState>,
    body: web::Json<BulkUpdateRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;

    conn.transaction::<_, APIError, _>(|conn| {
        if let Some(v) = body.is_verified {
            diesel::update(user_status::table.filter(user_status::user_id.eq_any(&body.user_ids)))
                .set(user_status::is_verified.eq(v))
                .execute(conn)?;
        }

        if let Some(lockout) = body.lockout_until {
            diesel::update(
                user_security::table.filter(user_security::user_id.eq_any(&body.user_ids)),
            )
            .set(user_security::lockout_until.eq(lockout))
                .execute(conn)?;
        }

        if let Some(role) = &body.role {
            diesel::update(users::table.filter(users::id.eq_any(&body.user_ids)))
                .set(users::role.eq(role.to_string()))
                .execute(conn)?;
        }
        Ok(())
    })?;

    Ok(Json(MessageResponse {
        message: "Users updated successfully".to_string(),
    }))
}
