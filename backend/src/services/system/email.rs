use crate::errors::APIError;
use lettre::message::{Mailbox, header::ContentType};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use tracing::info;

use crate::config::Config;

/// Sends a verification email to a recipient with a given token.
pub async fn send_verification_email(
    config: &Config,
    recipient_email: &str,
    token: &str,
) -> Result<bool, APIError> {
    if !config.send_emails {
        info!(
            "ACTION: Skipping verification email sending because email sending is disabled. Token: {}",
            token
        );
        return Ok(false);
    }

    let verification_link = format!("{}/{}", config.email_verification_base_url, token);

    let email_body = format!(
        "Please click on the following link to verify your email: {}",
        verification_link
    );

    let sender = config
        .smtp_sender_email
        .clone()
        .ok_or_else(|| APIError::internal("Sender email not configured"))?;
    let recipient = recipient_email.to_string();

    let email = Message::builder()
        .from(sender.parse::<Mailbox>()?)
        .to(recipient.parse::<Mailbox>()?)
        .subject("Verify your email for Skoola")
        .header(ContentType::TEXT_PLAIN)
        .body(email_body)?;

    let mailer = create_mailer(config)?;

    match mailer.send(&email) {
        Ok(_) => {
            info!("ACTION: Verification email sent to {}", recipient_email);
            Ok(true)
        }
        Err(e) => Err(APIError::internal(&format!(
            "Failed to send verification email: {:?}",
            e
        ))),
    }
}

/// Sends a password reset email to a recipient with a given token.
pub async fn send_password_reset_email(
    config: &Config,
    recipient_email: &str,
    token: &str,
) -> Result<bool, APIError> {
    if !config.send_emails {
        info!(
            "ACTION: Skipping password reset email sending because email sending is disabled. Token: {}",
            token
        );
        return Ok(false);
    }

    let reset_link = format!("{}/{}", config.password_reset_base_url, token);

    let email_body = format!(
        "Please click on the following link to reset your password: {}",
        reset_link
    );

    let sender = config
        .smtp_sender_email
        .clone()
        .ok_or_else(|| APIError::internal("Sender email not configured"))?;
    let recipient = recipient_email.to_string();

    let email = Message::builder()
        .from(sender.parse::<Mailbox>()?)
        .to(recipient.parse::<Mailbox>()?)
        .subject("Password Reset for Skoola")
        .header(ContentType::TEXT_PLAIN)
        .body(email_body)?;

    let mailer = create_mailer(config)?;

    match mailer.send(&email) {
        Ok(_) => {
            info!("ACTION: Password reset email sent to {}", recipient_email);
            Ok(true)
        }
        Err(e) => Err(APIError::internal(&format!(
            "Failed to send password reset email: {:?}",
            e
        ))),
    }
}

/// Sends a general email to a recipient with a given subject and body.
pub async fn send_email(
    config: &Config,
    to_email: String,
    subject: String,
    body: String,
) -> Result<(), APIError> {
    if !config.send_emails {
        info!("ACTION: Skipping email sending because email sending is disabled.");
        return Ok(());
    }

    let sender = config
        .smtp_sender_email
        .clone()
        .ok_or_else(|| APIError::internal("Sender email not configured"))?;
    let recipient = to_email;

    let email = Message::builder()
        .from(sender.parse::<Mailbox>()?)
        .to(recipient.parse::<Mailbox>()?)
        .subject(subject)
        .header(ContentType::TEXT_PLAIN)
        .body(body)?;

    let mailer = create_mailer(config)?;

    match mailer.send(&email) {
        Ok(_) => {
            info!("ACTION: Email sent to {}", recipient);
            Ok(())
        }
        Err(e) => Err(APIError::internal(&format!(
            "Failed to send email: {:?}",
            e
        ))),
    }
}

/// Helper function to create an SmtpTransport from the configuration.
fn create_mailer(config: &Config) -> Result<SmtpTransport, APIError> {
    let host = config
        .smtp_host
        .as_deref()
        .ok_or_else(|| APIError::internal("SMTP host not configured"))?;

    let builder = SmtpTransport::relay(host)?.port(config.smtp_port);

    let mailer =
        if let (Some(username), Some(password)) = (&config.smtp_username, &config.smtp_password) {
            let creds = Credentials::new(username.to_owned(), password.to_owned());
            builder.credentials(creds).build()
        } else {
            builder.build()
        };

    Ok(mailer)
}
