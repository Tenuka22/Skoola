use chrono::{NaiveDate, NaiveDateTime, Utc, Datelike, DateTime};
use fake::faker::address::en::{CityName, StateName, StreetName};
use fake::faker::internet::en::SafeEmail;
use fake::faker::name::en::Name;
use fake::faker::phone_number::en::PhoneNumber;
use fake::Fake;
use rand::Rng;

pub fn generate_random_email() -> String {
    SafeEmail().fake()
}

pub fn generate_random_name() -> String {
    Name().fake()
}

pub fn generate_random_address() -> String {
    format!(
        "{} {} {}",
        StreetName().fake::<String>(),
        CityName().fake::<String>(),
        StateName().fake::<String>()
    )
}

pub fn generate_random_phone_number() -> String {
    PhoneNumber().fake()
}

// Re-exporting from seed.rs for now, until we move common utilities
pub use crate::{generate_uuid, random_date_in_past, random_datetime_in_past};

pub fn generate_random_number_range(min: u32, max: u32) -> u32 {
    rand::thread_rng().gen_range(min..=max)
}

pub fn generate_random_bool() -> bool {
    rand::thread_rng().gen_bool(0.5)
}

// Add more generic data generation utilities as needed
