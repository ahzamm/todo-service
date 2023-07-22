use crate::db;
use crate::models::Status;
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    })
}

pub async fn get_todos(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to tohe database");

    let result = db::get_todos(&client).await;

    match result {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let list_id = path.0;
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to tohe database");

    let result = db::get_items(&client, list_id).await;

    match result {
        Ok(todo_items) => HttpResponse::Ok().json(todo_items),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
