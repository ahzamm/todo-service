mod config;
mod db;
mod handlers;
mod models;

use crate::config::Config;
use crate::handlers::*;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::io;
use tokio_postgres::NoTls;

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
            .route("/all-lists{_:/?}", web::get().to(get_all_lists))
            .route("/one-list/", web::get().to(get_one_list))
            .route("/create_list", web::post().to(create_list))
            .route("/create_item", web::post().to(create_item))
            .route("/checked_item/", web::patch().to(checked_item))
            .route("/unchecked_item/", web::patch().to(unchecked_item))
            .route("/delete_list/", web::patch().to(delete_list))
            .route("/delete_item/", web::patch().to(delete_item))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
