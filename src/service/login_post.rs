use std::sync::Arc;

use crate::{database::database::OracleDatabase, models::customer::CustomerOps};

use crate::models::customer::Customer;
use crate::models::login_credentials::LoginCredentials;

use anyhow::{Error, Ok};

#[derive(Debug, Clone)]
pub struct LoginService {
    db: Arc<OracleDatabase>,
}

impl LoginService {
    pub fn new(db: Arc<OracleDatabase>) -> Self {
        Self { db }
    }
}

impl LoginService {
    pub fn authenticate_user(&self, credentials: &LoginCredentials) -> Result<Customer, Error> {
        let user = self.db.get_customer(&credentials.email)?;

        if let Some(user) = user {
            if user.get_password() == credentials.password {
                return Ok(user);
            } else {
                return Err(Error::msg("Authentication failed! Incorrect password."));
            }
        }
        Err(Error::msg("User not found"))
    }
}
