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
    match regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$") {
        Ok(re) => re.is_match(email),
        Err(_) => false,
    }
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

pub fn validate_optional_string_not_empty(s: &Option<String>) -> Result<(), validator::ValidationError> {
    if let Some(val) = s {
        if val.trim().is_empty() {
            return Err(validator::ValidationError::new("cannot_be_empty"));
        }
    }
    Ok(())
}
