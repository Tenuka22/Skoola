use actix_web::{
    HttpResponse,
    error::ResponseError,
    http::{StatusCode, header::ContentType},
};

use apistos::ApiErrorComponent;
use derive_more::{Display, Error};
use diesel::result::Error as DieselError;
use schemars::JsonSchema;
use serde::{Serialize, Serializer};
use serde_json::json;
use tracing::error;
use r2d2::Error as R2d2Error;

fn serialize_status_code_as_u16<S>(
    status_code: &StatusCode,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_u16(status_code.as_u16())
}

#[derive(Debug, Display, Error, Serialize, JsonSchema, ApiErrorComponent)]
#[display("API Error: {name} ({status_code})")]
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
        APIError::internal("An internal database error occurred.")
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
