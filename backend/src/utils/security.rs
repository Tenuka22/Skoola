use bcrypt::{hash, verify, DEFAULT_COST, BcryptError};
use crate::errors::APIError;

pub fn hash_password(password: &str, pepper: &str) -> Result<String, APIError> {
    let password_with_pepper = format!("{}{}", password, pepper);
    hash(password_with_pepper, DEFAULT_COST).map_err(|e| {
        APIError::new(
            "Password Hashing Error",
            &format!("Failed to hash password: {}", e),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
    })
}

pub fn verify_password(password: &str, hashed_password: &str, pepper: &str) -> Result<bool, APIError> {
    let password_with_pepper = format!("{}{}", password, pepper);
    verify(password_with_pepper, hashed_password).map_err(|e| {
        APIError::new(
            "Password Verification Error",
            &format!("Failed to verify password: {}", e),
            actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
        )
    })
}
