mod models;
mod config;

use actix_web::{Responder, HttpServer, App, web};
use crate::models::Status;
use std::io;
use dotenv::dotenv;
use crate::config::Config;



async fn status() -> impl Responder {
    web::HttpResponse::Ok()
        .json(Status {status: "Up".to_string()})
}


#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = Config::from_env().unwrap();

    println!("Starting server at http://{}:{}", config.server.host, config.server.port);

    HttpServer::new(|| App::new() 
        .route("/", web::get().to(status))
    )
    .bind("{}:{}", config.server.host, config.server.port)?
    .run()
    .await
}