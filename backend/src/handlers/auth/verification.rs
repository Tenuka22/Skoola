use actix_web::web;
use actix_web::web::Json;
use apistos::api_operation;
use chrono::{Duration, Utc};
use diesel::prelude::*;
use tracing::{info, warn};

use crate::database::tables::{User, VerificationToken};
use crate::database::enums::VerificationPurpose;
use crate::models::MessageResponse;
use crate::models::ids::{IdPrefix, generate_prefixed_id};
use crate::models::auth::user::ResendVerificationEmailRequest;
use crate::services::system::email::send_verification_email;
use crate::{
    AppState, errors::APIError, schema::{user_security, user_status, users, verification_tokens},
};
use rand::distributions::{Alphanumeric, DistString};

#[api_operation(
    summary = "Verify user email",
    description = "Verifies a user's email address using a verification token.",
    tag = "auth",
    operation_id = "verify_email"
)]
pub async fn verify_email(
    data: web::Data<AppState>,
    token: web::Path<String>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let raw_token = token.into_inner();
    let now = Utc::now().naive_utc();

    let token_row: Option<VerificationToken> = verification_tokens::table
        .filter(verification_tokens::token_hash.eq(&raw_token))
        .filter(verification_tokens::is_active.eq(true))
        .filter(verification_tokens::consumed_at.is_null())
        .filter(verification_tokens::expires_at.gt(now))
        .select(VerificationToken::as_select())
        .first(&mut conn)
        .optional()?;

    match token_row {
        Some(vt) => {
            diesel::update(user_status::table.filter(user_status::user_id.eq(&vt.user_id)))
                .set((
                    user_status::is_verified.eq(true),
                    user_status::updated_at.eq(now),
                ))
                .execute(&mut conn)?;

            diesel::update(verification_tokens::table.find(&vt.id))
                .set((
                    verification_tokens::consumed_at.eq(Some(now)),
                    verification_tokens::is_active.eq(false),
                ))
                .execute(&mut conn)?;

            info!("User email verified successfully | user_id: {}", vt.user_id);
            Ok(Json(MessageResponse {
                message: "Email verified successfully! You can now log in.".to_string(),
            }))
        }
        None => {
            warn!("Email verification failed | reason: invalid or expired token");
            Err(APIError::bad_request("Invalid or expired verification token"))
        }
    }
}

#[api_operation(
    summary = "Resend verification email",
    description = "Allows a user to request a new verification email after a one-minute cooldown.",
    tag = "auth",
    operation_id = "resend_verification_email"
)]
pub async fn resend_verification_email(
    data: web::Data<AppState>,
    body: web::Json<ResendVerificationEmailRequest>,
) -> Result<Json<MessageResponse>, APIError> {
    let mut conn = data.db_pool.get()?;
    let now = Utc::now().naive_utc();

    let user: User = users::table
        .filter(users::email.eq(&body.email))
        .select(User::as_select())
        .first(&mut conn)?;

    let is_verified: bool = user_status::table
        .filter(user_status::user_id.eq(&user.id))
        .select(user_status::is_verified)
        .first(&mut conn)?;
    if is_verified {
        return Ok(Json(MessageResponse {
            message: "Email already verified.".to_string(),
        }));
    }

    let sent_at: Option<chrono::NaiveDateTime> = user_security::table
        .filter(user_security::user_id.eq(&user.id))
        .select(user_security::verification_sent_at)
        .first(&mut conn)
        .optional()?
        .flatten();
    if let Some(sent_at) = sent_at {
        let elapsed = now - sent_at;
        if elapsed < Duration::minutes(1) {
            let wait_time = Duration::minutes(1) - elapsed;
            return Err(APIError::bad_request(
                format!(
                    "Please wait {} seconds before requesting another verification email.",
                    wait_time.num_seconds()
                )
                .as_str(),
            ));
        }
    }

    let verification_token: String = Alphanumeric.sample_string(&mut rand::thread_rng(), 30);
    diesel::insert_into(verification_tokens::table)
        .values(&VerificationToken {
            id: generate_prefixed_id(&mut conn, IdPrefix::VERIFICATION_TOKEN)?,
            user_id: user.id.clone(),
            token_hash: verification_token.clone(),
            purpose: VerificationPurpose::EmailVerification,
            issued_at: now,
            expires_at: now + Duration::hours(24),
            consumed_at: None,
            is_active: true,
            metadata: None,
        })
        .execute(&mut conn)?;

    diesel::update(user_security::table.filter(user_security::user_id.eq(&user.id)))
        .set((
            user_security::verification_sent_at.eq(Some(now)),
            user_security::updated_at.eq(now),
        ))
        .execute(&mut conn)?;

    let _ = send_verification_email(&data.config, &user.email, &verification_token).await?;
    info!("Verification email resent | user_id: {}", user.id);

    Ok(Json(MessageResponse {
        message: "Verification email sent! Please check your inbox.".to_string(),
    }))
}
