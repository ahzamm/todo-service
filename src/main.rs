mod config;
mod models;

use crate::config::Config;
use crate::models::Status;
use actix_web::{web, App, HttpServer, Responder};
use dotenv::dotenv;
use std::io;
use tokio_postgres::NoTls;

async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    })
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    println!(
        "Starting server at http://{}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(status))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
