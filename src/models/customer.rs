use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Customer {
    customer_id: Option<i32>,
    first_name: String,
    last_name: String,
    address: String,
    #[validate(email)]
    email: String,
    #[validate(length(min = 1, max = 15))]
    password: String,
}

impl Customer {
    pub fn new(
        customer_id: Option<i32>,
        first_name: String,
        last_name: String,
        address: String,
        email: String,
        password: String,
    ) -> Self {
        //
        Customer {
            customer_id,
            first_name,
            last_name,
            address,
            email,
            password,
        }
    }
}

pub trait CustomerOps {
    fn get_first_name(&self) -> &str;
    fn get_last_name(&self) -> &str;
    fn get_email(&self) -> &str;
    fn get_address(&self) -> &str;
    fn get_customer_id(&self) -> Option<i32>;
    fn get_password(&self) -> &str;
}

impl CustomerOps for Customer {
    fn get_first_name(&self) -> &str {
        &self.first_name
    }
    fn get_last_name(&self) -> &str {
        &self.last_name
    }

    fn get_address(&self) -> &str {
        &self.address
    }
    fn get_customer_id(&self) -> Option<i32> {
        self.customer_id
    }
    fn get_email(&self) -> &str {
        &self.email
    }
    fn get_password(&self) -> &str {
        &self.password
    }
}
