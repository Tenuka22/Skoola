use actix_web::{web};
use apistos::api_operation;
use diesel::prelude::*;
use tracing::{info, warn};
use actix_web::web::Json;

use crate::{AppState, errors::APIError, schema::users};
use crate::database::tables::User;
use crate::models::auth::ResendVerificationEmailRequest;
use chrono::{Duration, Utc};
use rand::distributions::{Alphanumeric, DistString};
use crate::services::email::EmailService;
use crate::models::MessageResponse;

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
    let verification_token = token.into_inner();

    let user_result: Option<User> = users::table
        .filter(users::verification_token.eq(&verification_token))
        .select(User::as_select())
        .first(&mut conn)
        .optional()?;

    match user_result {
        Some(user) => {
            if user.is_verified {
                warn!("ACTION: Email verification failed | reason: email already verified | user_id: {}", user.id);
                return Err(APIError::bad_request("Email already verified"));
            }

            diesel::update(users::table.filter(users::id.eq(&user.id)))
                .set((
                    users::is_verified.eq(true),
                    users::verification_token.eq::<Option<String>>(None),
                ))
                .execute(&mut conn)?;

            info!("ACTION: User email verified successfully | user_id: {}", user.id);
            Ok(Json(MessageResponse { message: "Email verified successfully! You can now log in.".to_string() }))
        }
        None => {
            warn!("ACTION: Email verification failed | reason: invalid or expired token");
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
    let email_service = EmailService::new(data.config.clone());

    let mut user: User = users::table
        .filter(users::email.eq(&body.email))
        .select(User::as_select())
        .first(&mut conn)?;

    if user.is_verified {
        return Ok(Json(MessageResponse { message: "Email already verified.".to_string() }));
    }

    if let Some(sent_at) = user.verification_sent_at {
        let elapsed = Utc::now().naive_utc() - sent_at;
        if elapsed < Duration::minutes(1) {
            let wait_time = Duration::minutes(1) - elapsed;
            return Err(APIError::bad_request(
                format!("Please wait {} seconds before requesting another verification email.", wait_time.num_seconds()).as_str()
            ));
        }
    }

    let verification_token: String =
        Alphanumeric.sample_string(&mut rand::thread_rng(), 30);

    user.verification_token = Some(verification_token.clone());
    user.verification_sent_at = Some(Utc::now().naive_utc());
    user.updated_at = Utc::now().naive_utc();

    diesel::update(users::table.filter(users::id.eq(&user.id)))
        .set((
            users::verification_token.eq(&user.verification_token),
            users::verification_sent_at.eq(&user.verification_sent_at),
            users::updated_at.eq(&user.updated_at),
        ))
        .execute(&mut conn)?;

    let email_sent = email_service.send_verification_email(&user.email, &verification_token).await?;

    if email_sent {
        info!("ACTION: Verification email resent successfully | user_id: {} | email: {}", user.id, user.email);
    } else {
        info!("ACTION: Verification email sending was skipped, but a new token was generated. | user_id: {} | email: {}", user.id, user.email);
    }
    
    Ok(Json(MessageResponse { message: "Verification email sent! Please check your inbox.".to_string() }))
}
