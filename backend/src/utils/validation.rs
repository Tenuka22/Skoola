pub fn is_valid_nic(nic: &str) -> bool {
    // This is a very basic NIC validation. 
    // A more robust implementation would check for the format and the checksum.
    nic.len() == 10 || nic.len() == 12
}

pub fn is_valid_phone(phone: &str) -> bool {
    // This is a very basic phone number validation.
    // A more robust implementation would check for country codes and formats.
    phone.len() == 10 && phone.chars().all(|c| c.is_digit(10))
}

pub fn is_valid_email(email: &str) -> bool {
    // Basic regex for email validation.
    // A more robust implementation might use a dedicated email validation crate.
    let email_regex = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}
