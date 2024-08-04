use oracle::{Connection, Error as OracleError};

use std::sync::{Arc, Mutex};

use crate::models::customer::{Customer, CustomerOps};

use anyhow::Error;

//use crate::errors::errors_handling::{AppOracleError, FromEnvVarError};

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

        println!("Connecting to Oracle database at {:?}", oracle_url.clone());
        let conn = Connection::connect(username, password, oracle_url)?;

        Ok(Self {
            connection: Arc::new(Mutex::new(conn)),
        })
    }

    pub fn add_customer(&self, customer: &Customer) -> Result<(), Error> {
        let conn = self.connection.lock().unwrap();
        println!("Customer {:?}", customer);
        conn.execute(
            "INSERT INTO customers (FIRST_NAME, LAST_NAME, ADDRESS, EMAIL, password) VALUES (:1, :2, :3, :4, :5)",
            &[&customer.get_first_name(), &customer.get_last_name(), &customer.get_address(), &customer.get_email(), &customer.get_password()],
        )?;
        conn.commit()?;
        Ok(())
    }

    pub fn get_customer(&self, email: &str) -> Result<Option<Customer>, Error> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn
            .statement("SELECT customer_id, first_name, last_name, address, email, password FROM customers WHERE email = :1")
            .build()?;

        match stmt.query_row(&[&email]) {
            Ok(row) => {
                let customer_id: Option<i32> = row.get("customer_id")?;
                let first_name: String = row.get("first_name")?;
                let last_name: String = row.get("last_name")?;
                let address: String = row.get("address")?;
                let email: String = row.get("email")?;
                let password: String = row.get("password")?;

                let customer =
                    Customer::new(customer_id, first_name, last_name, address, email, password);

                Ok(Some(customer))
            }
            Err(err) if err.kind() == oracle::ErrorKind::NoDataFound => Ok(None),
            Err(err) => Err(Error::msg(err.to_string())),
        }
    }

    pub fn delete_customer(&self, email: &str) -> Result<(), Error> {
        let conn = self.connection.lock().unwrap();
        conn.execute("DELETE FROM customers WHERE email = :1", &[&email])?;
        conn.commit()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Context, Result};
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
    fn test_oracle_database_init() -> Result<()> {
        // Act: Initialize
        dotenv().ok();
        let username = env::var("ORACLE_USER").context("ORACLE_USER not set in .env file")?;
        let password =
            env::var("ORACLE_PASSWORD").context("ORACLE_PASSWORD not set in .env file")?;
        let hostname = env::var("ORACLE_HOST").context("ORACLE_HOST not set in .env file")?;
        let service = env::var("ORACLE_SERVICE").context("ORACLE_SERVICE not set in .env file")?;
        //
        let result = OracleDatabase::init(&username, &password, &hostname, &service);
        // Assert: Check if the result is Ok
        match result {
            Ok(_) => {
                println!("Database connection initialized successfully.");
                Ok(())
            }
            Err(err) => {
                panic!("Failed to initialize database connection: {:?}", err);
            }
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

        // Vérifier que le client n'existe pas avant l'ajout
        let is_customer = db.get_customer(&customer.get_email()).unwrap();
        assert!(is_customer.is_none(), "Customer already exists");

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
