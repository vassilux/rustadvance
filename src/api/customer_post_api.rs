use actix_web::{post, web, HttpResponse, Responder};
//use anyhow::Error;
use validator::Validate;

use crate::models::customer::Customer;
use crate::service::customer_post::CustomerPostService;

#[post("/insert_into_customers_table")]
async fn insert_into_customers_table(
    service: web::Data<CustomerPostService>,
    customer: web::Json<Customer>,
) -> impl Responder {
    // Log the incoming customer data
    println!("Received customer data: {:?}", customer);

    // Convert the incoming JSON data to the Customer struct
    let customer_data = customer.into_inner();

    // Validate the customer data
    match customer_data.validate() {
        Ok(_) => {
            // If validation is successful, attempt to insert data into the database
            match service
                .insert_data_into_customers_table(customer_data)
                .await
            {
                Ok(_) => HttpResponse::Ok().body("Data inserted successfully!"),
                Err(err) => {
                    println!("Error inserting data: {:?}", err);
                    HttpResponse::InternalServerError()
                        .body(format!("Error inserting data: {:?}", err))
                }
            }
        }
        Err(validation_errors) => {
            println!("Validation errors: {:?}", validation_errors);
            HttpResponse::BadRequest().body(format!("Validation errors: {:?}", validation_errors))
        }
    }
}
