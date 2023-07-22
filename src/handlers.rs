use crate::db;
use crate::models::*;
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
        .expect("Error connecting to the database");

    let result = db::get_items(&client, list_id).await;

    match result {
        Ok(todo_items) => HttpResponse::Ok().json(todo_items),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_list(
    db_pool: web::Data<Pool>,
    item: web::Json<TodoListRequest>,
) -> impl Responder {
    let title = item.title.clone();
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::create_todo(&client, title).await;
    match result {
        Ok(_todo_list) => HttpResponse::SeeOther()
            .header(
                actix_web::http::header::LOCATION,
                "http://localhost:8080/todos",
            )
            .finish(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_item(
    db_pool: web::Data<Pool>,
    item: web::Json<TodoItemRequest>,
) -> impl Responder {
    let title = item.title.clone();
    let list_id = item.list_id.clone();
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::create_todo_item(&client, &title, &list_id).await;

    match result {
        Ok(_todo_list) => HttpResponse::SeeOther()
            .header(
                actix_web::http::header::LOCATION,
                format!("http://localhost:8080/todo_items/{}/items/", list_id),
            )
            .finish(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
