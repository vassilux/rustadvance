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
    let mut client = self.db.client.lock().await()?;
    let query_result = client
        .query(
            "SELECT count(*) FROM customers WHERE email = @p1",
            &[&customer.get_email()],
        )
        .await
        .with_context(|| "error execution query")?;

    if let Some(row) = query_result.into_row().await? {
        let count: Option<i32> = row.get(0);

        if count.unwrap() > 0 {
            return Err(Error::msg("Email alraedy exists!").into());
        }
    } else {
        return Err(Error::msg("No rows returned by query").into());
    }

    if let Err(e) = client.execute(
		"INSERT INTO customers (first_name, last_name, address, email, password) VALUES (@p1, @p2, @p3, @p4, @p5)",
		&[
			&customer.get_first_name(),
			&customer.get_last_name(),
			&customer.get_address(),
			&customer.get_email(),
			&customer.get_password()
		],
	).await {
		println!("Error executing query!: {}", e);
		return Err(e.into());
	}

    let created_customer = Customer::new(
        customer.get_first_name().to_string(),
        customer.get_last_name().to_string(),
        customer.get_address().to_string(),
        customer.get_email().to_string(),
        customer.get_password().to_string(),
    );
    Ok(Some(created_customer))
}
