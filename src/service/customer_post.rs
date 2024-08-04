use crate::database::database::OracleDatabase;
use crate::models::customer::{Customer, CustomerOps};
use anyhow::Error;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CustomerPostService {
    db: Arc<OracleDatabase>,
}

impl CustomerPostService {
    pub fn new(db: Arc<OracleDatabase>) -> Self {
        CustomerPostService { db }
    }

    pub async fn insert_data_into_customers_table(
        &self,
        customer: Customer,
    ) -> Result<Customer, Error> {
        let is_customer = self.db.get_customer(customer.get_email()).unwrap();

        if is_customer.is_some() {
            return Err(Error::msg("Email already exists!"));
        }

        self.db.add_customer(&customer)?;

        let created_customer = self
            .db
            .get_customer(customer.get_email())?
            .ok_or_else(|| Error::msg("Failed to retrieve the created customer"))?;

        Ok(created_customer)
    }
}
