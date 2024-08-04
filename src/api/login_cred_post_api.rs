use actix_web::{post, web, HttpResponse, Responder};
//use anyhow::Error;
use validator::Validate;

use crate::models::login_credentials::{LoginCredentials, LoginParams};
use crate::service::login_post::LoginService;

use log::Log;

#[post("/customer_login")]
async fn customer_login(
    service: web::Data<LoginService>,
    login_params: web::Json<LoginParams>,
) -> impl Responder {
    // Log the incoming customer data

    println!("Received loginParams data: {:?}", login_params);

    let email_param = login_params.email().to_string();
    let password_param = login_params.password().to_string();

    let login_data = login_params.into_inner();
    let validator_result = login_data.validate();
    // Handle validation errors
    if let Err(validation_errors) = validator_result {
        println!("Validation errors: {:?}", validation_errors);
        return HttpResponse::BadRequest()
            .body(format!("Validation errors: {:?}", validation_errors));
    }

    let login_credentials = LoginCredentials {
        customer_id: 0,
        email: email_param,
        password: password_param,
    };

    // Validate the customer data
    match service.authenticate_user(&login_credentials) {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => {
            println!("Authentification error : {:?}", err);
            HttpResponse::BadRequest().body(format!("Authentification errors: {:?}", err))
        }
    }
}
