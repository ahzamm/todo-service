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
            .route("/todos{_:/?}", web::get().to(get_todos))
            .route("/todo_items/{list_id}/items/", web::get().to(get_items))
            .route("/create_list", web::post().to(create_list))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
