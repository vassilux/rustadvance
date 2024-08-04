use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoginCredentials {
    pub customer_id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct LoginParams {
    #[validate(length(min = 1, max = 50), custom(function = "check_email"))]
    pub email: String,
    #[validate(length(min = 4))]
    pub password: String,
}

impl LoginParams {
    pub fn new(email: String, password: String) -> Result<Self, ValidationError> {
        let params = LoginParams { email, password };
        //params.validate()?;
        Ok(params)
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}

pub fn check_email(value: &str) -> Result<(), ValidationError> {
    //use regex crate for email validation create a regex pattern for email validation
    let email_regex =
        regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0=9.%+-]+\.[a-zA-Z]{2,}$").unwrap();

    if !email_regex.is_match(value) {
        let mut error = ValidationError::new("invalid email");
        error.message = Some("must be a valid email address".into());
        return Err(error);
    }

    Ok(())
}
