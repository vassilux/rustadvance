use dotenv::dotenv;
use std::env;
use std::io;
use std::sync::Arc;

mod api;
mod database;
mod errors;
mod models;
mod service;
mod tls;

use api::customer_post_api::insert_into_customers_table;
use database::database::OracleDatabase;
use service::customer_post::CustomerPostService;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let username = env::var("ORACLE_USER").expect("ORACLE_USER not set");
    let password = env::var("ORACLE_PASSWORD").expect("ORACLE_PASSWORD not set");
    let hostname = env::var("ORACLE_HOST").expect("ORACLE_HOST not set");
    let service = env::var("ORACLE_SERVICE").expect("ORACLE_SERVICE not set");

    let db = match OracleDatabase::init(&username, &password, &hostname, &service) {
        Ok(db) => db,
        Err(err) => {
            println!("{}", err);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                err.to_string(),
            ));
        }
    };

    let db = Arc::new(db);

    let customer_post_service = CustomerPostService::new(db.clone());

    println!("Starting server on http://127.0.0.1:8080/");

    let tls_config = tls::load_rustls_config().unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(customer_post_service.clone()))
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .wrap(
                        Cors::default()
                            .allowed_origin("http://localhost:3000")
                            .allowed_methods(vec!["GET", "POST"])
                            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                            .allowed_header(header::CONTENT_TYPE)
                            .max_age(3600),
                    )
                    .configure(configure_services),
            )
    })
    //.bind("127.0.0.1:8080")?
    .bind_rustls(("0.0.0.0", 8443), tls_config)?
    .run()
    .await?;

    Ok(())
}

fn configure_services(cfg: &mut web::ServiceConfig) {
    cfg.service(insert_into_customers_table);
}
