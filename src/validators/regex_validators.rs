use validator::ValidationError;

pub fn validate_hex_color(value: &String) -> Result<(), ValidationError> {
    let re = regex::Regex::new(r"^#[A-Fa-f0-9]{6}$").unwrap();

    if re.is_match(value) {
        Ok(())
    } else {
        Err(ValidationError::new("Invalid hex color format"))
    }
}
