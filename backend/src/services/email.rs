use crate::errors::APIError;
use lettre::message::{header::ContentType, Mailbox};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{SmtpTransport, Transport, Message};
use tracing::info;

use crate::config::Config;

#[derive(Clone)]
pub struct EmailService {
    config: Config,
}

impl EmailService {
    pub fn new(config: Config) -> Self {
        EmailService { config }
    }

    pub async fn send_verification_email(&self, recipient_email: &str, token: &str) -> Result<bool, APIError> {
        if !self.config.send_emails {
            info!("ACTION: Skipping verification email sending because email sending is disabled. Token: {}", token);
            return Ok(false);
        }

        let verification_link = format!(
            "{}/{}",
            self.config.email_verification_base_url, token
        );

        let email_body = format!(
            "Please click on the following link to verify your email: {}",
            verification_link
        );

        let sender = self.config.smtp_sender_email.clone().ok_or_else(|| APIError::internal("Sender email not configured"))?;
        let recipient = recipient_email.to_string();

        let email = Message::builder()
            .from(sender.parse::<Mailbox>()?)
            .to(recipient.parse::<Mailbox>()?)
            .subject("Verify your email for Skoola")
            .header(ContentType::TEXT_PLAIN)
            .body(email_body)
            ?;

        let mailer = if let (Some(username), Some(password)) = (
            &self.config.smtp_username,
            &self.config.smtp_password,
        ) {
            let creds = Credentials::new(username.to_owned(), password.to_owned());
            SmtpTransport::relay(self.config.smtp_host.as_deref().ok_or_else(|| APIError::internal("SMTP host not configured"))?)
                ?
                .port(self.config.smtp_port)
                .credentials(creds)
                .build()
        } else {
            SmtpTransport::relay(self.config.smtp_host.as_deref().ok_or_else(|| APIError::internal("SMTP host not configured"))?)
                ?
                .port(self.config.smtp_port)
                .build()
        };

        match mailer.send(&email) {
            Ok(_) => {
                info!("ACTION: Verification email sent to {}", recipient_email);
                Ok(true)
            }
            Err(e) => {
                Err(APIError::internal(format!("Failed to send verification email: {:?}", e).as_str()))
            }
        }
    }

    pub async fn send_password_reset_email(&self, recipient_email: &str, token: &str) -> Result<bool, APIError> {
        if !self.config.send_emails {
            info!("ACTION: Skipping password reset email sending because email sending is disabled. Token: {}", token);
            return Ok(false);
        }

        let reset_link = format!(
            "{}/{}",
            self.config.password_reset_base_url, token
        );

        let email_body = format!(
            "Please click on the following link to reset your password: {}",
            reset_link
        );

        let sender = self.config.smtp_sender_email.clone().ok_or_else(|| APIError::internal("Sender email not configured"))?;
        let recipient = recipient_email.to_string();

        let email = Message::builder()
            .from(sender.parse::<Mailbox>()?)
            .to(recipient.parse::<Mailbox>()?)
            .subject("Password Reset for Skoola")
            .header(ContentType::TEXT_PLAIN)
            .body(email_body)
            ?;

        let mailer = if let (Some(username), Some(password)) = (
            &self.config.smtp_username,
            &self.config.smtp_password,
        ) {
            let creds = Credentials::new(username.to_owned(), password.to_owned());
            SmtpTransport::relay(self.config.smtp_host.as_deref().ok_or_else(|| APIError::internal("SMTP host not configured"))?)
                ?
                .port(self.config.smtp_port)
                .credentials(creds)
                .build()
        } else {
            SmtpTransport::relay(self.config.smtp_host.as_deref().ok_or_else(|| APIError::internal("SMTP host not configured"))?)
                ?
                .port(self.config.smtp_port)
                .build()
        };

        match mailer.send(&email) {
            Ok(_) => {
                info!("ACTION: Password reset email sent to {}", recipient_email);
                Ok(true)
            }
            Err(e) => {
                Err(APIError::internal(&format!("Failed to send password reset email: {:?}", e)))
            }
        }
    }

    pub async fn send_email(
        &self,
        to_email: String,
        subject: String,
        body: String,
    ) -> Result<(), APIError> {
        if !self.config.send_emails {
            info!("ACTION: Skipping email sending because email sending is disabled.");
            return Ok(());
        }

        let sender = self.config.smtp_sender_email.clone().ok_or_else(|| APIError::internal("Sender email not configured"))?;
        let recipient = to_email;

        let email = Message::builder()
            .from(sender.parse::<Mailbox>()?)
            .to(recipient.parse::<Mailbox>()?)
            .subject(subject)
            .header(ContentType::TEXT_PLAIN)
            .body(body)
            ?;

        let mailer = if let (Some(username), Some(password)) = (
            &self.config.smtp_username,
            &self.config.smtp_password,
        ) {
            let creds = Credentials::new(username.to_owned(), password.to_owned());
            SmtpTransport::relay(self.config.smtp_host.as_deref().ok_or_else(|| APIError::internal("SMTP host not configured"))?)
                ?
                .port(self.config.smtp_port)
                .credentials(creds)
                .build()
        } else {
            SmtpTransport::relay(self.config.smtp_host.as_deref().ok_or_else(|| APIError::internal("SMTP host not configured"))?)
                ?
                .port(self.config.smtp_port)
                .build()
        };

        match mailer.send(&email) {
            Ok(_) => {
                info!("ACTION: Email sent to {}", recipient);
                Ok(())
            }
            Err(e) => {
                Err(APIError::internal(&format!("Failed to send email: {:?}", e)))
            }
        }
    }
}
