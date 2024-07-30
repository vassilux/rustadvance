use actix_web::{post, web, HttpResponse, Responder};
use anyhow::Error;
use validator::Validate;

use crate::models::customer::Customer;
use crate::service::customer_post::CustomerPostService;

#[post("/insert_into_customers_table")]
async fn insert_into_customers_table(
    db: web::Data<CustomerPostService>,
    customer: web::Json<Customer>,
) -> impl Responder {
    let customer_data = customer.into_inner();
    let validation_result = customer_data.validate();

    if let Err(validation_errors) = validation_result {
        let _ = Error::msg(format!("Validation errors: {:?}", validation_errors));
        return HttpResponse::BadRequest()
            .body(format!("Validation errors: {:?}", validation_errors));
    }

    if let Err(err) = db.insert_data_into_customers_table(customer_data).await {
        let _ = Error::msg(format!("Error insert data: {:?}", err));
        return HttpResponse::InternalServerError()
            .body(format!("Error inserting data: {:?}", err));
    }

    HttpResponse::Ok().body("Data inserted successfully!")
}
