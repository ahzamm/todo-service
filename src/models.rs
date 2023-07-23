use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo_list")]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "todo_item")]
pub struct TodoItem {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32,
}

#[derive(Deserialize)]
pub struct TodoListRequest {
    pub title: String,
}

#[derive(Deserialize)]
pub struct TodoItemRequest {
    pub title: String,
    pub list_id: i32,
}

#[derive(Deserialize)]
pub struct QueryParams {
    pub id: i32,
}
