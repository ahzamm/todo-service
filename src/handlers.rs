use crate::db;
use crate::models::*;
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    })
}

pub async fn get_all_lists(db_pool: web::Data<Pool>) -> impl Responder {
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

pub async fn get_one_list(
    db_pool: web::Data<Pool>,
    query_params: web::Query<QueryParams>,
) -> impl Responder {
    let list_id = query_params.id;
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

pub async fn checked_item(
    db_pool: web::Data<Pool>,
    query_params: web::Query<QueryParams>,
) -> impl Responder {
    let item_id = query_params.id;
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");
    let result = db::todo_item_checked(&client, &item_id, true).await;

    // we have to extract list id and use it

    match result {
        Ok(list_id) => HttpResponse::SeeOther()
            .header(
                actix_web::http::header::LOCATION,
                format!("http://localhost:8080/one-list/?id={:?}", list_id.unwrap()),
            )
            .finish(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
