#![recursion_limit = "512"]

pub mod config;
pub mod database;
pub mod errors;
pub mod faker;
pub mod handlers;
pub mod models;
pub mod routes;
pub mod schema;
pub mod services;
pub mod utils;

pub use config::AppState;
pub use errors::APIError;
