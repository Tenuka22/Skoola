use crate::errors::APIError;
use bcrypt::{DEFAULT_COST, hash, verify};

pub fn hash_password(password: &str) -> Result<String, APIError> {
    Ok(hash(password, DEFAULT_COST)?)
}

pub fn verify_password(password: &str, hashed_password: &str) -> Result<bool, APIError> {
    Ok(verify(password, hashed_password)?)
}
