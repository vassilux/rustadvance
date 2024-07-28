use crate::database::OracleDatabase;
use crate::models::customers::{Costumer, CostumerOps};
use anyhow::{Context, Error};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CustomerPostService {
    db: Arc<OracleDatabase>,
}

impl CustomerPostService {
    pub fn new(db: Arc<DatabaseMSSQL>) -> Self {
        CustomerPostService { db }
    }
}

pub async fn insert_data_into_customers_table(
    &self,
    customer: Customer,
) -> Result<Option<Customer>, Error> {
    let db_lock = self.db.lock().await;
    let is_customer = db_lock.get_customer(&customer.get_email()).await?;

    if let Some(is_customer) = is_customer {
        return Err(Error::msg("Email alraedy exists!").into());
    }
    let created_customer = db_lock.add_customer(&customer).await?;
    Ok(Some(created_customer))
}
