use actix_multipart::MultipartError;
use actix_web::{
    HttpResponse,
    error::{BlockingError, ResponseError},
    http::{StatusCode, header::ContentType},
};

use anyhow::Error as AnyhowError;
use apistos::ApiErrorComponent;
use derive_more::{Display, Error};
use diesel::result::Error as DieselError;
use r2d2::Error as R2d2Error;
use schemars::JsonSchema;
use serde::{Serialize, Serializer};
use serde_json::json;
use tracing::error;

// New use statements for error types
use bcrypt::BcryptError;
use jsonwebtoken::errors::Error as JwtError;
use lettre::address::AddressError;
use lettre::error::Error as LettreError;
use lettre::transport::smtp::Error as SmtpError;
use reqwest::Error as ReqwestError;
use std::env::VarError;
use std::num::ParseIntError;

pub mod iam;

fn serialize_status_code_as_u16<S>(
    status_code: &StatusCode,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u16(status_code.as_u16())
}

#[derive(Debug, Display, Error, Serialize, JsonSchema, ApiErrorComponent, Clone)]
#[display(fmt = "API Error: {name} ({status_code}): {message}")]
#[openapi_error(
    status(code = 400, description = "Bad Request"),
    status(code = 401, description = "Unauthorized"),
    status(code = 403, description = "Forbidden"),
    status(code = 404, description = "Not Found"),
    status(code = 405, description = "Method Not Allowed"),
    status(code = 406, description = "Not Acceptable"),
    status(code = 408, description = "Request Timeout"),
    status(code = 409, description = "Conflict"),
    status(code = 410, description = "Gone"),
    status(code = 415, description = "Unsupported Media Type"),
    status(code = 422, description = "Unprocessable Entity"),
    status(code = 429, description = "Too Many Requests"),
    status(code = 500, description = "Internal Server Error"),
    status(code = 501, description = "Not Implemented"),
    status(code = 502, description = "Bad Gateway"),
    status(code = 503, description = "Service Unavailable"),
    status(code = 504, description = "Gateway Timeout")
)]
pub struct APIError {
    pub name: String,
    pub message: String,

    #[serde(serialize_with = "serialize_status_code_as_u16")]
    #[schemars(with = "u16")]
    pub status_code: StatusCode,
}

impl APIError {
    pub fn new(name: &str, message: &str, status_code: StatusCode) -> Self {
        Self {
            name: name.to_string(),
            message: message.to_string(),
            status_code,
        }
    }

    pub fn bad_request(message: &str) -> Self {
        Self::new("BadRequest", message, StatusCode::BAD_REQUEST)
    }

    pub fn unauthorized(message: &str) -> Self {
        Self::new("Unauthorized", message, StatusCode::UNAUTHORIZED)
    }

    pub fn forbidden(message: &str) -> Self {
        Self::new("Forbidden", message, StatusCode::FORBIDDEN)
    }

    pub fn not_found(message: &str) -> Self {
        Self::new("NotFound", message, StatusCode::NOT_FOUND)
    }

    pub fn conflict(message: &str) -> Self {
        Self::new("Conflict", message, StatusCode::CONFLICT)
    }

    pub fn internal(message: &str) -> Self {
        Self::new(
            "InternalServerError",
            message,
            StatusCode::INTERNAL_SERVER_ERROR,
        )
    }

    pub fn unprocessable_entity(message: &str) -> Self {
        Self::new(
            "UnprocessableEntity",
            message,
            StatusCode::UNPROCESSABLE_ENTITY,
        )
    }

    pub fn service_unavailable(message: &str) -> Self {
        Self::new(
            "ServiceUnavailable",
            message,
            StatusCode::SERVICE_UNAVAILABLE,
        )
    }
}

impl ResponseError for APIError {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code)
            .insert_header(ContentType::json())
            .json(json!({
                "name": self.name,
                "status": self.status_code.as_u16(),
                "message": self.message,
            }))
    }
}

impl From<APIError> for std::io::Error {
    fn from(error: APIError) -> Self {
        std::io::Error::new(std::io::ErrorKind::Other, error.message)
    }
}

impl From<DieselError> for APIError {
    fn from(error: DieselError) -> Self {
        error!("Database error: {:?}", error);
        match error {
            DieselError::NotFound => APIError::not_found("Resource not found."),
            DieselError::DatabaseError(_, info) => {
                error!("Database error details: {}", info.message());
                APIError::internal(&format!(
                    "An internal database error occurred: {}",
                    info.message()
                ))
            }
            _ => APIError::internal(&format!("An internal database error occurred: {}", error)),
        }
    }
}

impl From<MultipartError> for APIError {
    fn from(error: MultipartError) -> Self {
        error!("Multipart error: {:?}", error);
        APIError::bad_request("Invalid multipart form data.")
    }
}

impl From<BlockingError> for APIError {
    fn from(error: BlockingError) -> Self {
        error!("Blocking error: {:?}", error);
        APIError::internal("An internal error occurred.")
    }
}

impl From<std::io::Error> for APIError {
    fn from(error: std::io::Error) -> Self {
        error!("I/O error: {:?}", error);
        APIError::internal("An internal I/O error occurred.")
    }
}

impl From<R2d2Error> for APIError {
    fn from(error: R2d2Error) -> Self {
        error!("Database connection pool error: {:?}", error);
        APIError::internal("Failed to get database connection.")
    }
}

impl From<actix_web::Error> for APIError {
    fn from(error: actix_web::Error) -> Self {
        error!("Actix-web error: {:?}", error);

        let status_code = error.as_response_error().status_code();

        match status_code {
            StatusCode::UNAUTHORIZED => APIError::unauthorized("Authentication failed."),
            StatusCode::FORBIDDEN => APIError::forbidden("Access denied."),
            StatusCode::NOT_FOUND => APIError::not_found("Resource not found."),
            StatusCode::BAD_REQUEST => APIError::bad_request("Bad request."),
            _ => APIError::internal("An internal error occurred."),
        }
    }
}

impl From<&'static str> for APIError {
    fn from(error: &'static str) -> Self {
        error!("Static string error: {:?}", error);
        APIError::internal(error)
    }
}

impl From<AnyhowError> for APIError {
    fn from(error: AnyhowError) -> Self {
        APIError::internal(&format!("Internal service error: {}", error))
    }
}

// New implementations for various error types
impl From<ReqwestError> for APIError {
    fn from(error: ReqwestError) -> Self {
        error!("Reqwest error: {:?}", error);
        APIError::internal(&format!("External service request failed: {}", error))
    }
}

impl From<AddressError> for APIError {
    fn from(error: AddressError) -> Self {
        error!("Lettre Address error: {:?}", error);
        APIError::bad_request(&format!("Invalid email address: {}", error))
    }
}

impl From<LettreError> for APIError {
    fn from(error: LettreError) -> Self {
        error!("Lettre error: {:?}", error);
        APIError::internal(&format!("Email composition error: {}", error))
    }
}

impl From<SmtpError> for APIError {
    fn from(error: SmtpError) -> Self {
        error!("Lettre SMTP error: {:?}", error);
        APIError::internal(&format!("SMTP transport error: {}", error))
    }
}

impl From<BcryptError> for APIError {
    fn from(error: BcryptError) -> Self {
        error!("Bcrypt error: {:?}", error);
        APIError::internal("Password hashing failed.")
    }
}

impl From<JwtError> for APIError {
    fn from(error: JwtError) -> Self {
        error!("JWT error: {:?}", error);
        APIError::unauthorized(&format!("Invalid token: {}", error))
    }
}

impl From<ParseIntError> for APIError {
    fn from(error: ParseIntError) -> Self {
        error!("ParseInt error: {:?}", error);
        APIError::bad_request(&format!("Invalid number format: {}", error))
    }
}

impl From<VarError> for APIError {
    fn from(error: VarError) -> Self {
        error!("Env Var error: {:?}", error);
        APIError::internal(&format!("Missing environment variable: {}", error))
    }
}
