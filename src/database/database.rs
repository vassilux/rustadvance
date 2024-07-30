use oracle::Connection;

use std::sync::{Arc, Mutex};

use crate::errors::errors_handling::{AppOracleError, FromEnvVarError};

use crate::models::customer::{Customer, CustomerOps};

use anyhow::Error;

#[derive(Clone, Debug)]
pub struct OracleDatabase {
    pub connection: Arc<Mutex<Connection>>,
}

impl OracleDatabase {
    pub fn init(
        username: &str,
        password: &str,
        hostname: &str,
        service: &str,
    ) -> Result<Self, Error> {
        let oracle_url = format!("{}/{}", hostname, service);

        dbg!("Connecting to Oracle database at {}", oracle_url.clone());
        let conn = Connection::connect(username, password, oracle_url)?;

        Ok(Self {
            connection: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn add_customer(&self, customer: &Customer) -> Result<(), Error> {
        let conn = self.connection.lock().unwrap();
        conn.execute(
            "INSERT INTO M1USER.CUSTOMERS (FIRST_NAME, LAST_NAME, ADDRESS, EMAIL, password) VALUES (:1, :2, :3, :4, :5)",
            &[&customer.get_first_name(), &customer.get_last_name(), &customer.get_address(), &customer.get_email(), &customer.get_password()],
        )?;
        conn.commit()?;
        Ok(())
    }

    pub fn get_customer(&self, email: &str) -> Result<Option<Customer>, Error> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.statement("SELECT customer_id, first_name, last_name, address, email, password FROM M1USER.CUSTOMERS WHERE email = :1").build()?;
        let row = stmt.query_row(&[&email])?;

        let customer_id = row.get("customer_id")?;
        let first_name = row.get("first_name")?;
        let last_name = row.get("last_name")?;
        let address = row.get("address")?;
        let email = row.get("email")?;
        let password = row.get("password")?;

        let customer = Customer::new(customer_id, first_name, last_name, address, email, password);

        Ok(Some(customer))
    }

    pub fn delete_customer(&self, email: &str) -> Result<(), Error> {
        let conn = self.connection.lock().unwrap();
        conn.execute("DELETE FROM M1USER.CUSTOMERS WHERE email = :1", &[&email])?;
        conn.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenv::dotenv;
    use std::env;

    fn setup_db() -> OracleDatabase {
        dotenv().ok();
        let username = env::var("ORACLE_USER").expect("ORACLE_USER not set");
        let password = env::var("ORACLE_PASSWORD").expect("ORACLE_PASSWORD not set");
        let hostname = env::var("ORACLE_HOST").expect("ORACLE_HOST not set");
        let service = env::var("ORACLE_SERVICE").expect("ORACLE_SERVICE not set");

        OracleDatabase::init(&username, &password, &hostname, &service)
            .expect("Failed to initialize database")
    }

    #[test]
    fn test_oracle_database_init() {
        // Act: Initialize the
        dotenv().ok();
        let username = env::var("ORACLE_USER")
            .map_err(AppOracleError::from_env_var_error)
            .unwrap();
        let password = env::var("ORACLE_PASSWORD")
            .map_err(AppOracleError::from_env_var_error)
            .unwrap();
        let hostname = env::var("ORACLE_HOST")
            .map_err(AppOracleError::from_env_var_error)
            .unwrap();
        let service = env::var("ORACLE_SERVICE")
            .map_err(AppOracleError::from_env_var_error)
            .unwrap();
        //
        let result = OracleDatabase::init(&username, &password, &hostname, &service);
        // Assert: Check if the result is Ok
        match result {
            Ok(_) => println!("Database connection initialized successfully."),
            Err(err) => panic!("Failed to initialize database connection: {:?}", err),
        }
    }

    #[test]
    fn test_oracle_database_init_missing_env_var() {
        // Act: Initialize the OracleDatabase
        let result = OracleDatabase::init("test_user", "test_password", "localhost", "XE");

        // Assert: Check if the result is an error
        assert!(result.is_err());
        if let Err(err) = result {
            println!("Expected error occurred: {:?}", err);
        }
    }

    #[test]
    fn test_add_get_customer() {
        let db = setup_db();

        let customer = Customer::new(
            None,
            String::from("John"),
            String::from("Doe"),
            String::from("123 Main St"),
            String::from("john.doe@example.com"),
            String::from("securepassword"),
        );

        let result = db.add_customer(&customer);
        assert!(result.is_ok());

        let email = "john.doe@example.com";
        let fetched_customer = db.get_customer(email).expect("Failed to get customer");

        match fetched_customer {
            Some(c) => {
                assert_eq!(c.get_email(), email);
                println!("Customer found: {:?}", c);
            }
            None => panic!("Customer not found"),
        }
        let target_email = customer.get_email().clone();
        db.delete_customer(target_email)
            .expect("Failed to delete customer");
    }
}
