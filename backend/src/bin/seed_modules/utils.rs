use fake::Fake;
use fake::faker::address::en::{CityName, StateName, StreetName};
use fake::faker::name::en::Name;
use fake::faker::phone_number::en::PhoneNumber;
use rand::seq::SliceRandom;
use std::collections::HashSet;

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

pub fn get_random_id(ids: &[String]) -> String {
    if ids.is_empty() {
        panic!("Attempted to get a random ID from an empty list. Check seeding order.");
    }
    ids.choose(&mut rand::thread_rng()).unwrap().clone()
}

pub fn generate_random_email_prefix() -> String {
    use fake::faker::internet::en::FreeEmailProvider;
    let email: String = FreeEmailProvider().fake();
    email.split('@').next().unwrap_or("user").to_string()
}

pub fn generate_random_email_unique(used_emails: &mut HashSet<String>, prefix: &str) -> String {
    generate_random_email_unique_with_domain(used_emails, prefix, "@example.com")
}

pub fn generate_random_email_unique_with_domain(
    used_emails: &mut HashSet<String>,
    prefix: &str,
    domain: &str,
) -> String {
    let mut email = format!("{prefix}{domain}");
    let mut counter = 1;
    while used_emails.contains(&email) {
        email = format!("{prefix}{counter}{domain}");
        counter += 1;
    }
    used_emails.insert(email.clone());
    email
}
