use crate::models::{TodoItem, TodoList};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_todos(client: &Client) -> Result<Vec<TodoList>, io::Error> {
    let statement = client
        .prepare("select * from todo_list order by id desc")
        .await
        .unwrap();
    let todo = client
        .query(&statement, &[])
        .await
        .expect("Error getting todo items")
        .iter()
        .map(|row| TodoList::from_row_ref(row).unwrap())
        .collect::<Vec<TodoList>>();

    Ok(todo)
}

pub async fn get_items(client: &Client, list_id: i32) -> Result<Vec<TodoItem>, io::Error> {
    let statement = client
        .prepare("select * from todo_item where list_id = $1 order by id desc")
        .await
        .unwrap();
    let todo = client
        .query(&statement, &[&list_id])
        .await
        .expect("Error getting todo items")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();

    Ok(todo)
}

pub async fn create_todo(client: &Client, title: String) -> Result<Vec<TodoItem>, io::Error> {
    let statement = client
        .prepare("INSERT INTO todo_list (title) VALUES ($1)")
        .await
        .unwrap();
    let todo = client
        .query(&statement, &[&title])
        .await
        .expect("Error creating new list")
        .iter()
        .map(|row| TodoItem::from_row_ref(row).unwrap())
        .collect::<Vec<TodoItem>>();

    Ok(todo)
}
