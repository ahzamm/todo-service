use crate::db;
use crate::models::*;
use crate::redis_client;
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "Up".to_string(),
    })
}

pub async fn get_all_lists(db_pool: web::Data<Pool>) -> impl Responder {
    let key = String::from("all_lists");

    if let Some(all_lists) =
        redis_client::get_data_from_redis(&key).expect("Failed to get data from Redis")
    {
        let lists: Vec<TodoList> = serde_json::from_str(&all_lists).unwrap();
        let json_lists = serde_json::to_string_pretty(&lists).unwrap();

        HttpResponse::Ok().body(json_lists)
    } else {
        let client: Client = db_pool
            .get()
            .await
            .expect("Error connecting to the database");

        let result = db::get_all_lists(&client).await;

        match result {
            Ok(todos) => {
                let todos_json =
                    serde_json::to_string(&todos).expect("Failed to serialize todos to JSON");
                redis_client::set_data_in_redis(&key, &todos_json)
                    .expect("Failed to set data in Redis");
                HttpResponse::Ok().json(todos)
            }

            Err(_) => HttpResponse::InternalServerError().into(),
        }
    }
}

pub async fn get_one_list(
    db_pool: web::Data<Pool>,
    query_params: web::Query<QueryParams>,
) -> impl Responder {
    let list_id = query_params.id;
    let key = format!("list_no_{}", list_id).to_string();

    if let Some(one_list) =
        redis_client::get_data_from_redis(&key).expect("Failed to get data from Redis")
    {
        let list: Vec<TodoList> = serde_json::from_str(&one_list).unwrap();
        let json_list = serde_json::to_string_pretty(&list).unwrap();

        HttpResponse::Ok().body(json_list)
    } else {
        let client: Client = db_pool
            .get()
            .await
            .expect("Error connecting to the database");

        let result = db::get_one_list(&client, list_id).await;

        match result {
            Ok(one_list) => {
                let todos_json =
                    serde_json::to_string(&one_list).expect("Failed to serialize todos to JSON");
                redis_client::set_data_in_redis(&key, &todos_json)
                    .expect("Failed to set data in Redis");
                HttpResponse::Ok().json(one_list)
            }
            Err(_) => HttpResponse::InternalServerError().into(),
        }
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

    let result = db::create_list(&client, title).await;

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

    let result = db::create_item(&client, &title, &list_id).await;

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

    let result = db::checked_item(&client, &item_id, true).await;

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

pub async fn unchecked_item(
    db_pool: web::Data<Pool>,
    query_params: web::Query<QueryParams>,
) -> impl Responder {
    let item_id = query_params.id;

    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::checked_item(&client, &item_id, false).await;

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

pub async fn delete_list(
    db_pool: web::Data<Pool>,
    query_params: web::Query<QueryParams>,
) -> impl Responder {
    let id = query_params.id;

    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::delete_list(&client, &id).await;

    match result {
        Ok(_) => HttpResponse::SeeOther()
            .header(
                actix_web::http::header::LOCATION,
                "http://localhost:8080/all-lists",
            )
            .finish(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn delete_item(
    db_pool: web::Data<Pool>,
    query_params: web::Query<QueryParams>,
) -> impl Responder {
    let id = query_params.id;

    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::delete_item(&client, &id).await;

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
