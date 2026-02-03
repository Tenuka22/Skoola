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

pub fn is_valid_admission_number(admission_number: &str) -> bool {
    // Assuming a simple length check for admission numbers for now.
    !admission_number.is_empty() && admission_number.len() <= 50
}

pub fn is_valid_gender(gender: &str) -> bool {
    matches!(gender, "Male" | "Female" | "Other")
}

pub fn is_valid_religion(religion: &str) -> bool {
    // Basic check for non-empty religion string.
    !religion.is_empty()
}

pub fn is_valid_ethnicity(ethnicity: &str) -> bool {
    // Basic check for non-empty ethnicity string.
    !ethnicity.is_empty()
}
